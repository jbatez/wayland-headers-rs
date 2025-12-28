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

        self.add_message_opcode_consts(interface, side);
        self.add_message_since_version_consts(interface);

        match side {
            Side::Client => self.add_client_wrapper_fns(interface),
            Side::Server => self.add_send_event_fns(interface),
        }

        self.add_enums(interface);
    }

    fn add_interface_extern_type(&mut self, interface: &Interface, side: Side) {
        let name = interface.name.as_ref().unwrap();
        if name == "wl_display" || (side == Side::Server && name == "wl_shm_pool") {
            // These are provided by `use super::wayland_{side_name}_core::*;`
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
            Self::rust_type_from_arg(arg)
        };

        *text += &format!("        {name}: {typ},\n");
    }

    fn rust_type_from_arg(arg: &Arg) -> String {
        match arg.typ.as_ref().unwrap().as_str() {
            "int" | "fd" => "i32".to_owned(),
            "new_id" | "uint" => "u32".to_owned(),
            "fixed" => "wl_fixed_t".to_owned(),
            "string" => "*const c_char".to_owned(),
            "object" => format!("*mut {}", arg.interface.as_ref().unwrap()),
            "array" => "*mut wl_array".to_owned(),
            _ => "i32".to_owned(),
        }
    }

    fn add_interface_add_listener_fn(&mut self, interface: &Interface) {
        let interface_name = interface.name.as_ref().unwrap();
        let name = format!("{interface_name}_add_listener");

        let text = format!(
            "\
#[inline]
pub unsafe extern \"C\" fn {name}(
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

    fn add_message_opcode_consts(&mut self, interface: &Interface, side: Side) {
        let mut opcode = 0;
        for content in &interface.contents {
            match side {
                Side::Client => {
                    if let InterfaceContent::Request(request) = content {
                        self.add_message_opcode_const(interface, request, opcode);
                        opcode += 1;
                    }
                }
                Side::Server => {
                    if let InterfaceContent::Event(event) = content {
                        self.add_message_opcode_const(interface, event, opcode);
                        opcode += 1;
                    }
                }
            }
        }
    }

    fn add_message_opcode_const(&mut self, interface: &Interface, message: &Message, opcode: u32) {
        let interface_name = interface.name.as_ref().unwrap();
        let message_name = message.name.as_ref().unwrap();
        let name = format!("{interface_name}_{message_name}").to_ascii_uppercase();
        let text = format!("pub const {name}: u32 = {opcode};");
        self.module.constants.push((name, text));
    }

    fn add_message_since_version_consts(&mut self, interface: &Interface) {
        for content in &interface.contents {
            match content {
                InterfaceContent::Request(message) | InterfaceContent::Event(message) => {
                    self.add_message_since_version_const(interface, message);
                }
                _ => (),
            }
        }
    }

    fn add_message_since_version_const(&mut self, interface: &Interface, message: &Message) {
        let interface_name = interface.name.as_ref().unwrap();
        let message_name = message.name.as_ref().unwrap();
        let name = format!("{interface_name}_{message_name}_since_version").to_ascii_uppercase();
        let since = message.since.as_ref().map(String::as_str).unwrap_or("1");
        let text = format!("pub const {name}: u32 = {since};");
        self.module.constants.push((name, text));
    }

    fn add_client_wrapper_fns(&mut self, interface: &Interface) {
        self.add_set_user_data_fn(interface);
        self.add_get_user_data_fn(interface);
        self.add_get_version_fn(interface);
        self.maybe_add_destroy_fn(interface);
        self.add_request_fns(interface);
    }

    fn add_set_user_data_fn(&mut self, interface: &Interface) {
        let interface_name = interface.name.as_ref().unwrap();
        let name = format!("{interface_name}_set_user_data");

        let text = format!(
            "\
#[inline]
pub unsafe extern \"C\" fn {name}(
    {interface_name}: *mut {interface_name},
    user_data: *mut c_void,
) {{
    unsafe {{
        wl_proxy_set_user_data(
            {interface_name}.cast(),
            user_data,
        )
    }}
}}"
        );

        self.module.functions.push((name, text));
    }

    fn add_get_user_data_fn(&mut self, interface: &Interface) {
        let interface_name = interface.name.as_ref().unwrap();
        let name = format!("{interface_name}_get_user_data");

        let text = format!(
            "\
#[inline]
pub unsafe extern \"C\" fn {name}(
    {interface_name}: *mut {interface_name},
) -> *mut c_void {{
    unsafe {{ wl_proxy_get_user_data({interface_name}.cast()) }}
}}"
        );

        self.module.functions.push((name, text));
    }

    fn add_get_version_fn(&mut self, interface: &Interface) {
        let interface_name = interface.name.as_ref().unwrap();
        let name = format!("{interface_name}_get_version");

        let text = format!(
            "\
#[inline]
pub unsafe extern \"C\" fn {name}(
    {interface_name}: *mut {interface_name},
) -> u32 {{
    unsafe {{ wl_proxy_get_version({interface_name}.cast()) }}
}}"
        );

        self.module.functions.push((name, text));
    }

    fn maybe_add_destroy_fn(&mut self, interface: &Interface) {
        if interface.name.as_ref().unwrap() == "wl_display" {
            return;
        }

        for content in &interface.contents {
            if let InterfaceContent::Request(request) = content {
                if request.name.as_ref().unwrap() == "destroy" {
                    return;
                }
            }
        }

        self.add_destroy_fn(interface);
    }

    fn add_destroy_fn(&mut self, interface: &Interface) {
        let interface_name = interface.name.as_ref().unwrap();
        let name = format!("{interface_name}_destroy");

        let text = format!(
            "\
#[inline]
pub unsafe extern \"C\" fn {name}(
    {interface_name}: *mut {interface_name},
) {{
    unsafe {{ wl_proxy_destroy({interface_name}.cast()) }}
}}"
        );

        self.module.functions.push((name, text));
    }

    fn add_request_fns(&mut self, interface: &Interface) {
        for content in &interface.contents {
            if let InterfaceContent::Request(request) = content {
                self.add_request_fn(interface, request);
            }
        }
    }

    fn add_request_fn(&mut self, interface: &Interface, request: &Message) {
        let interface_name = interface.name.as_ref().unwrap();
        let request_name = request.name.as_ref().unwrap();
        let name = format!("{interface_name}_{request_name}");

        let mut text = String::new();
        text += &format!("#[inline]\n");
        text += &format!("pub unsafe extern \"C\" fn {name}(\n");
        text += &format!("    {interface_name}: *mut {interface_name},\n");

        let mut ret_arg = None;
        for content in &request.contents {
            if let MessageContent::Arg(arg) = content {
                if arg.typ.as_ref().unwrap() == "new_id" {
                    assert!(ret_arg.is_none());
                    ret_arg = Some(arg);
                    if arg.interface.is_none() {
                        text += "    interface: *const wl_interface,\n";
                        text += "    version: u32,\n";
                    }
                } else {
                    let name = arg.name.as_ref().unwrap();
                    let typ = Self::rust_type_from_arg(arg);
                    text += &format!("    {name}: {typ},\n");
                }
            }
        }

        text += ")";
        if let Some(ret_arg) = ret_arg {
            if let Some(ret_interface) = ret_arg.interface.as_ref() {
                text += &format!(" -> *mut {ret_interface}");
            } else {
                text += " -> *mut c_void";
            }
        }

        text += &format!(" {{\n");
        text += &format!("    unsafe {{\n");

        let opcode = format!("{interface_name}_{request_name}").to_uppercase();
        if let Some(ret_arg) = ret_arg {
            if let Some(ret_interface) = ret_arg.interface.as_ref() {
                text += &format!("        wl_proxy_marshal_constructor(\n");
                text += &format!("            {interface_name}.cast(),\n");
                text += &format!("            {opcode},\n");
                text += &format!("            &{ret_interface}_interface,\n");
            } else {
                text += &format!("        wl_proxy_marshal_constructor_versioned(\n");
                text += &format!("            {interface_name}.cast(),\n");
                text += &format!("            {opcode},\n");
                text += &format!("            interface,\n");
                text += &format!("            version,\n");
            }
        } else {
            text += &format!("        wl_proxy_marshal(\n");
            text += &format!("            {interface_name}.cast(),\n");
            text += &format!("            {opcode},\n");
        }

        for content in &request.contents {
            if let MessageContent::Arg(arg) = content {
                if arg.typ.as_ref().unwrap() == "new_id" {
                    if arg.interface.is_none() {
                        text += "            (*interface).name,\n";
                        text += "            version,\n";
                    }
                    text += "            null_mut::<c_void>(),\n";
                } else {
                    text += "            ";
                    text += arg.name.as_ref().unwrap();
                    text += ",\n";
                }
            }
        }

        text += "        )";
        if ret_arg.is_some() {
            text += ".cast()\n";
        } else {
            text += ";\n"
        }

        if request.typ.as_ref().map(String::as_str) == Some("destructor") {
            text += &format!("        wl_proxy_destroy(\n");
            text += &format!("            {interface_name}.cast(),\n");
            text += &format!("        );\n");
        }

        text += "    }\n";
        text += "}";
        self.module.functions.push((name, text));
    }

    fn add_send_event_fns(&mut self, interface: &Interface) {
        if interface.name.as_ref().unwrap() == "wl_display" {
            return;
        }

        for content in &interface.contents {
            if let InterfaceContent::Event(event) = content {
                self.add_send_event_fn(interface, event);
            }
        }
    }

    fn add_send_event_fn(&mut self, interface: &Interface, event: &Message) {
        let interface_name = interface.name.as_ref().unwrap();
        let event_name = event.name.as_ref().unwrap();
        let name = format!("{interface_name}_send_{event_name}");

        let mut text = String::new();
        text += &format!("#[inline]\n");
        text += &format!("pub unsafe extern \"C\" fn {name}(\n");
        text += &format!("    resource_: *mut wl_resource,\n");

        for content in &event.contents {
            if let MessageContent::Arg(arg) = content {
                let name = arg.name.as_ref().unwrap();
                let typ = match arg.typ.as_ref().unwrap().as_str() {
                    "new_id" | "object" => "*mut wl_resource".to_owned(),
                    _ => Self::rust_type_from_arg(arg),
                };
                text += &format!("    {name}: {typ},\n");
            }
        }

        text += ") {\n";
        text += "    unsafe {\n";
        text += "        wl_resource_post_event(\n";
        text += "            resource_,\n";

        let opcode = format!("{interface_name}_{event_name}").to_ascii_uppercase();
        text += &format!("            {opcode},\n");

        for content in &event.contents {
            if let MessageContent::Arg(arg) = content {
                text += "            ";
                text += arg.name.as_ref().unwrap();
                text += ",\n";
            }
        }

        text += "        )\n";
        text += "    }\n";
        text += "}";
        self.module.functions.push((name, text));
    }

    fn add_enums(&mut self, interface: &Interface) {
        for content in &interface.contents {
            if let InterfaceContent::Enum(enu) = content {
                self.add_enum(interface, enu)
            }
        }
    }

    fn add_enum(&mut self, interface: &Interface, enu: &Enum) {
        for content in &enu.contents {
            if let EnumContent::Entry(entry) = content {
                self.add_enum_entry(interface, enu, entry);
            }
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
}
