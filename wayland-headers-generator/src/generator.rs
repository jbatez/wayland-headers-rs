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
        let side_str = side.to_str();
        let mut generator = Generator::new(format!("wayland_{side_str}_protocol"));
        generator.add_import_core(side_str);
        generator.visit_protocol(protocol, side);
        generator.module.write_file();
    }

    fn add_import_core(&mut self, side_str: &str) {
        let text = format!("use super::wayland_{side_str}_core::*;");
        self.module.imports.push(text);
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
        self.add_enum_type_alias(interface, enu);

        // TODO
    }

    fn add_enum_type_alias(&mut self, interface: &Interface, enu: &Enum) {
        let interface_name = interface.name.as_ref().unwrap();
        let enum_name = enu.name.as_ref().unwrap();
        let name = format!("{interface_name}_{enum_name}");
        let text = format!("pub type {name} = c_int;");
        self.module.type_aliases.push((name.to_owned(), text));
    }
}
