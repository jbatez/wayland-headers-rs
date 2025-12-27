use std::collections::{HashMap, HashSet};

use wayland_protocol::*;

use crate::module::*;

#[derive(Clone, Copy, Eq, PartialEq)]
enum Side {
    Client,
    Server,
}

impl Side {
    fn to_str(self) -> &'static str {
        match self {
            Side::Client => "client",
            Side::Server => "server",
        }
    }
}

pub(crate) struct Generator {
    module: Module,
    extern_types: HashSet<String>,
    enum_types: HashMap<String, &'static str>,
}

impl Generator {
    fn new(name: String) -> Self {
        Self {
            module: Module::new(name),
            extern_types: HashSet::new(),
            enum_types: HashMap::new(),
        }
    }

    pub(crate) fn generate() {
        let protocol = Protocol::wayland();
        Self::generate_protocol_module(&protocol, Side::Client);
        Self::generate_protocol_module(&protocol, Side::Server);
    }

    fn generate_protocol_module(protocol: &Protocol, side: Side) {
        let side_str = side.to_str();
        let mut generator = Generator::new(format!("wayland_{side_str}_protocol"));
        generator.add_import_side_core(side_str);
        generator.pre_exclude_core_extern_types(side);
        generator.pre_visit_protocol(protocol);
        generator.visit_protocol(protocol, side);
        generator.module.write_file();
    }

    fn add_import_side_core(&mut self, side_str: &str) {
        let text = format!("use super::wayland_{side_str}_core::*;");
        self.module.imports.push(text);
    }

    fn pre_exclude_core_extern_types(&mut self, side: Side) {
        let names = match side {
            Side::Client => ["wl_display"].as_slice(),
            Side::Server => ["wl_display", "wl_shm_pool"].as_slice(),
        };
        for &name in names {
            self.extern_types.insert(name.to_owned());
        }
    }

    fn pre_visit_protocol(&mut self, protocol: &Protocol) {
        for content in &protocol.contents {
            if let ProtocolContent::Interface(interface) = content {
                self.pre_visit_interface(interface);
            }
        }
    }

    fn pre_visit_interface(&mut self, interface: &Interface) {
        self.add_extern_type(interface.name.as_ref().unwrap());

        for content in &interface.contents {
            match content {
                InterfaceContent::Description(_) => (),
                InterfaceContent::Request(request) => self.pre_visit_message(interface, request),
                InterfaceContent::Event(event) => self.pre_visit_message(interface, event),
                InterfaceContent::Enum(_) => (),
            }
        }
    }

    fn add_extern_type(&mut self, name: &str) {
        if self.extern_types.insert(name.to_owned()) {
            let text = format!(
                "\
#[repr(C)]
pub struct {name} {{
    _data: (),
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}}"
            );
            self.module.structs.push((name.to_owned(), text));
        }
    }

    fn pre_visit_message(&mut self, interface: &Interface, message: &Message) {
        for content in &message.contents {
            if let MessageContent::Arg(arg) = content {
                self.pre_visit_arg(interface, arg);
            }
        }
    }

    fn pre_visit_arg(&mut self, interface: &Interface, arg: &Arg) {
        if let Some(enum_name) = arg.enu.as_ref() {
            self.save_enum_type(interface, arg, enum_name);
        }
    }

    fn save_enum_type(&mut self, interface: &Interface, arg: &Arg, enum_name: &str) {
        let full_enum_name = if enum_name.contains('.') {
            enum_name.to_owned()
        } else {
            let interface_name = interface.name.as_ref().unwrap();
            format!("{interface_name}.{enum_name}")
        };

        let typ = if full_enum_name == "wl_output.transform" {
            "i32"
        } else {
            match arg.typ.as_ref().unwrap().as_str() {
                "int" => "i32",
                "uint" => "u32",
                _ => panic!("unexpected enum arg type"),
            }
        };

        let old_type = self.enum_types.insert(full_enum_name, typ);
        if let Some(old_type) = old_type {
            assert_eq!(old_type, typ);
        }
    }

    fn visit_protocol(&mut self, protocol: &Protocol, side: Side) {
        for content in &protocol.contents {
            if let ProtocolContent::Interface(interface) = content {
                self.visit_interface(interface, side);
            }
        }
    }

    fn visit_interface(&mut self, interface: &Interface, side: Side) {
        self.add_extern_static_interface(interface);

        for content in &interface.contents {
            match content {
                InterfaceContent::Description(_) => (),
                InterfaceContent::Request(request) => self.visit_request(interface, request, side),
                InterfaceContent::Event(event) => self.visit_event(interface, event, side),
                InterfaceContent::Enum(enu) => self.visit_enum(interface, enu),
            }
        }
    }

    fn add_extern_static_interface(&mut self, interface: &Interface) {
        let interface_name = interface.name.as_ref().unwrap();
        let name = format!("{interface_name}_interface");
        let text = format!("    pub static {name}: wl_interface;");
        self.module.extern_statics.push((name, text));
    }

    fn visit_request(&mut self, interface: &Interface, request: &Message, side: Side) {
        // TODO
    }

    fn visit_event(&mut self, interface: &Interface, event: &Message, side: Side) {
        // TODO
    }

    fn visit_enum(&mut self, interface: &Interface, enu: &Enum) {
        for content in &enu.contents {
            if let EnumContent::Entry(entry) = content {
                self.visit_entry(interface, enu, entry);
            }
        }
    }

    fn visit_entry(&mut self, interface: &Interface, enu: &Enum, entry: &Entry) {
        let interface_name = interface.name.as_ref().unwrap();
        let enum_name = enu.name.as_ref().unwrap();
        let entry_name = entry.name.as_ref().unwrap();
        let name = format!("{interface_name}_{enum_name}_{entry_name}").to_ascii_uppercase();

        let typ = {
            let full_enum_name = format!("{interface_name}.{enum_name}");
            if let Some(&typ) = self.enum_types.get(&full_enum_name) {
                typ
            } else {
                assert_eq!(enum_name, "error");
                "u32"
            }
        };

        let value = entry.value.as_ref().unwrap();
        let text = format!("pub const {name}: {typ} = {value};");
        self.module.constants.push((name, text));
    }
}
