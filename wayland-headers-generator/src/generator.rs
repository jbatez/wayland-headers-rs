use std::collections::HashSet;

use wayland_protocol::*;

use crate::module::*;

#[derive(Clone, Copy, Eq, PartialEq)]
enum Side {
    Client,
    Server,
}

pub(crate) struct Generator {
    module: Module,
    extern_struct_names: HashSet<String>,
}

impl Generator {
    fn new(name: &str) -> Self {
        Self {
            module: Module::new(name),
            extern_struct_names: HashSet::new(),
        }
    }

    pub(crate) fn generate() {
        let protocol = Protocol::wayland();
        Self::generate_protocol_module(&protocol, Side::Client);
        Self::generate_protocol_module(&protocol, Side::Server);
    }

    fn generate_protocol_module(protocol: &Protocol, side: Side) {
        let name = match side {
            Side::Client => "wayland_client_protocol",
            Side::Server => "wayland_server_protocol",
        };

        let mut generator = Generator::new(name);
        generator.visit_protocol(protocol, side);
        generator.module.write_file();
    }

    fn visit_protocol(&mut self, protocol: &Protocol, side: Side) {
        for content in &protocol.contents {
            if let ProtocolContent::Interface(interface) = content {
                self.visit_interface(interface, side);
            }
        }
    }

    fn visit_interface(&mut self, interface: &Interface, side: Side) {
        for content in &interface.contents {
            match content {
                InterfaceContent::Description(_) => (),
                InterfaceContent::Request(request) => self.visit_request(interface, request, side),
                InterfaceContent::Event(event) => self.visit_event(interface, event, side),
                InterfaceContent::Enum(enu) => self.visit_enum(interface, enu, side),
            }
        }
    }

    fn visit_request(&mut self, interface: &Interface, request: &Message, side: Side) {
        // TODO
    }

    fn visit_event(&mut self, interface: &Interface, event: &Message, side: Side) {
        // TODO
    }

    fn visit_enum(&mut self, interface: &Interface, enu: &Enum, side: Side) {
        // TODO
    }
}
