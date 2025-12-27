use std::collections::HashMap;

use wayland_protocol::*;

use crate::module::*;

#[derive(Clone, Copy, Eq, PartialEq)]
enum Side {
    Client,
    Server,
}

impl Side {
    fn name(self) -> &'static str {
        match self {
            Side::Client => "client",
            Side::Server => "server",
        }
    }
}

pub(crate) struct Generator {
    module: Module,
}

impl Generator {
    fn new(name: String) -> Self {
        Self {
            module: Module::new(name),
        }
    }

    pub(crate) fn generate() {
        let protocol = Protocol::wayland();
        Self::generate_protocol_module(&protocol, Side::Client);
        Self::generate_protocol_module(&protocol, Side::Server);
    }

    fn generate_protocol_module(protocol: &Protocol, side: Side) {
        let side_name = side.name();
        let mut generator = Generator::new(format!("wayland_{side_name}_protocol"));
        generator.add_import_side_core(side_name);
        generator.add_protocol(protocol, side);
        generator.module.write_file();
    }

    fn add_import_side_core(&mut self, side_name: &str) {
        let text = format!("use super::wayland_{side_name}_core::*;");
        self.module.imports.push(text);
    }

    fn add_protocol(&mut self, protocol: &Protocol, side: Side) {
        for content in &protocol.contents {
            if let ProtocolContent::Interface(interface) = content {
                self.add_interface(interface, side);
            }
        }
    }

    fn add_interface(&mut self, interface: &Interface, side: Side) {
        self.add_interface_extern_type(interface, side);
        self.add_interface_extern_static(interface);
        self.add_interface_struct(interface, side);
        self.add_enums(interface, side);
    }

    fn add_interface_extern_type(&mut self, interface: &Interface, side: Side) {
        let name = interface.name.as_ref().unwrap();
        if name == "wl_display" || (side == Side::Server && name == "wl_shm_pool") {
            return;
        }

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

    fn add_interface_extern_static(&mut self, interface: &Interface) {
        let interface_name = interface.name.as_ref().unwrap();
        let name = format!("{interface_name}_interface");
        let text = format!("    pub static {name}: wl_interface;");
        self.module.extern_statics.push((name, text));
    }

    fn add_interface_struct(&mut self, interface: &Interface, side: Side) {
        let interface_name = interface.name.as_ref().unwrap();
        let suffix = match side {
            Side::Client => "listener",
            Side::Server => "interface",
        };
        let name = format!("{interface_name}_{suffix}");

        let mut text = String::new();
        text += &format!("#[derive(Clone, Copy)]\n");
        text += &format!("#[repr(C)]\n");
        text += &format!("pub struct {name} {{\n");

        let mut empty = true;
        for content in &interface.contents {
            match side {
                Side::Client => {
                    if let InterfaceContent::Event(event) = content {
                        Self::add_interface_struct_member(interface, event, side, &mut text);
                        empty = false;
                    }
                }
                Side::Server => {
                    if let InterfaceContent::Request(request) = content {
                        Self::add_interface_struct_member(interface, request, side, &mut text);
                        empty = false;
                    }
                }
            }
        }

        if !empty {
            text += "}";
            self.module.structs.push((name, text));

            if side == Side::Client {
                self.add_interface_add_listener_fn(interface);
            }
        }
    }

    fn add_interface_struct_member(
        interface: &Interface,
        message: &Message,
        side: Side,
        text: &mut String,
    ) {
        let interface_name = interface.name.as_ref().unwrap();
        let message_name = match message.name.as_ref().unwrap().as_str() {
            "move" => "mov",
            name => name,
        };

        *text += &format!("    pub {message_name}: Option<unsafe extern \"C\" fn(\n");

        match side {
            Side::Client => {
                *text += &format!("        data: *mut c_void,\n");
                *text += &format!("        {interface_name}: *mut {interface_name},\n");
            }
            Side::Server => {
                *text += "        client: *mut wl_client,\n";
                *text += "        resource: *mut wl_resource,\n";
            }
        }

        for content in &message.contents {
            if let MessageContent::Arg(arg) = content {
                Self::add_interface_struct_member_arg(arg, side, text);
            }
        }

        *text += "    )>,\n";
    }

    fn add_interface_struct_member_arg(arg: &Arg, side: Side, text: &mut String) {
        let name = arg.name.as_ref().unwrap();

        let typ = arg.typ.as_ref().unwrap().as_str();
        let typ = if side == Side::Server && typ == "object" {
            "*mut wl_resource".to_owned()
        } else if side == Side::Server && typ == "new_id" && arg.interface.is_none() {
            *text += "        interface: *const c_char,\n";
            *text += "        version: u32,\n";
            "u32".to_owned()
        } else if side == Side::Client && typ == "object" && arg.interface.is_none() {
            "*mut c_void".to_owned()
        } else if side == Side::Client && typ == "new_id" {
            format!("*mut {}", arg.interface.as_ref().unwrap())
        } else {
            match typ {
                "int" | "fd" => "i32".to_owned(),
                "new_id" | "uint" => "u32".to_owned(),
                "fixed" => "wl_fixed_t".to_owned(),
                "string" => "*const c_char".to_owned(),
                "object" => format!("*mut {}", arg.interface.as_ref().unwrap()),
                "array" => "*mut wl_array".to_owned(),
                _ => "i32".to_owned(),
            }
        };

        *text += &format!("        {name}: {typ},\n");
    }

    fn add_interface_add_listener_fn(&mut self, interface: &Interface) {
        let interface_name = interface.name.as_ref().unwrap();
        let name = format!("{interface_name}_add_listener");

        let text = format!(
            "\
#[inline]
pub unsafe fn {name}(
    {interface_name}: *mut {interface_name},
    listener: *const {interface_name}_listener,
    data: *mut c_void,
) -> c_int {{
    unsafe {{
        wl_proxy_add_listener(
            {interface_name}.cast(),
            listener.cast_mut().cast(),
            data,
        )
    }}
}}"
        );

        self.module.functions.push((name, text));
    }

    fn add_enums(&mut self, interface: &Interface, side: Side) {
        for content in &interface.contents {
            if let InterfaceContent::Enum(enu) = content {
                self.add_enum(interface, enu, side)
            }
        }
    }

    fn add_enum(&mut self, interface: &Interface, enu: &Enum, side: Side) {
        for content in &enu.contents {
            if let EnumContent::Entry(entry) = content {
                self.add_enum_entry(interface, enu, entry);
            }
        }

        if side == Side::Server {
            self.add_enum_is_valid_fn(interface, enu);
        }
    }

    fn add_enum_entry(&mut self, interface: &Interface, enu: &Enum, entry: &Entry) {
        self.add_enum_entry_const(interface, enu, entry);
        self.add_enum_entry_since_version_const(interface, enu, entry);
    }

    fn build_enum_entry_name(interface: &Interface, enu: &Enum, entry: &Entry) -> String {
        let interface_name = interface.name.as_ref().unwrap();
        let enum_name = enu.name.as_ref().unwrap();
        let entry_name = entry.name.as_ref().unwrap();
        format!("{interface_name}_{enum_name}_{entry_name}").to_ascii_uppercase()
    }

    fn add_enum_entry_const(&mut self, interface: &Interface, enu: &Enum, entry: &Entry) {
        let name = Self::build_enum_entry_name(interface, enu, entry);
        let value = entry.value.as_ref().unwrap();
        let text = format!("pub const {name}: u32 = {value};");
        self.module.constants.push((name.clone(), text));
    }

    fn add_enum_entry_since_version_const(
        &mut self,
        interface: &Interface,
        enu: &Enum,
        entry: &Entry,
    ) {
        if let Some(since) = entry.since.as_ref()
            && since != "1"
        {
            let name = Self::build_enum_entry_name(interface, enu, entry);
            let name = format!("{name}_SINCE_VERSION");
            let text = format!("pub const {name}: u32 = {since};");
            self.module.constants.push((name, text));
        }
    }

    fn add_enum_is_valid_fn(&mut self, interface: &Interface, enu: &Enum) {
        let interface_name = interface.name.as_ref().unwrap();
        let enum_name = enu.name.as_ref().unwrap();
        let name = format!("{interface_name}_{enum_name}_is_valid");

        let mut text = String::new();
        text += &format!("#[inline]\n");
        text += &format!("pub fn {name}(value: u32, version: u32) -> bool {{\n");

        let is_bitfield = match enu.bitfield.as_ref().map(String::as_str) {
            None => false,
            Some("true") => true,
            Some("false") => false,
            Some(value) => panic!("unexpected bitfield attribute value: {value:?}"),
        };

        if is_bitfield {
            text += "    let mut valid = 0;\n";
        } else {
            text += "    match value {\n";
        }

        for content in &enu.contents {
            let EnumContent::Entry(entry) = content else {
                continue;
            };

            let name = Self::build_enum_entry_name(interface, enu, entry);
            let since = entry.since.as_ref().map(String::as_str).unwrap_or("1");

            if is_bitfield {
                text += &format!("    if version >= {since} {{\n");
                text += &format!("        valid |= {name};\n");
                text += &format!("    }}\n");
            } else {
                text += &format!("        {name} => version >= {since},\n")
            }
        }

        if is_bitfield {
            text += "    (value & !valid) == 0\n";
        } else {
            text += "        _ => false,\n";
            text += "    }\n";
        }

        text += "}";
        self.module.functions.push((name, text));
    }
}
