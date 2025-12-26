use quick_xml::{
    Reader,
    escape::resolve_xml_entity,
    events::{BytesStart, Event, attributes::Attribute},
};

use crate::protocol::*;

impl Protocol {
    /// Parses the given XML text. This library is only tested with the bundled
    /// XML file version, but may work with others.
    pub fn parse(xml: &str) -> Self {
        let mut parser = Parser {
            reader: Reader::from_str(xml),
        };
        parser.parse_file()
    }

    /// Parses the bundled copy of
    /// [wayland.xml](https://gitlab.freedesktop.org/wayland/wayland/-/blob/main/protocol/wayland.xml).
    pub fn wayland() -> Self {
        Self::parse(include_str!("wayland.xml"))
    }
}

struct Parser<'a> {
    reader: Reader<&'a [u8]>,
}

enum Content<'a> {
    Text(&'a str),
    Elem(Elem<'a>),
}

#[derive(Debug)]
struct Elem<'a> {
    is_empty: bool,
    start: BytesStart<'a>,
}

impl<'a> Parser<'a> {
    fn next_event<'b>(&mut self, buf: &'b mut Vec<u8>) -> Event<'b> {
        self.reader.read_event_into(buf).unwrap()
    }

    fn save_attr(&mut self, attr: Attribute, out: &mut Option<String>) {
        assert_eq!(*out, None);
        let decoder = self.reader.decoder();
        let value = attr.decode_and_unescape_value(decoder).unwrap();
        *out = Some(value.into_owned());
    }

    fn assert_is_ws(&mut self, text: &[u8]) {
        for &b in text {
            assert!(matches!(b, b'\n' | b'\r' | b' '));
        }
    }

    fn parse_contents<F>(&mut self, elem: Elem, mut f: F)
    where
        F: FnMut(&mut Parser, Content),
    {
        if elem.is_empty {
            return;
        }

        let mut buf = Vec::new();
        loop {
            match self.next_event(&mut buf) {
                Event::Comment(_) => {
                    ();
                }
                Event::Text(text) => {
                    let text = text.xml_content().unwrap();
                    f(self, Content::Text(&text));
                }
                Event::GeneralRef(text) => {
                    let text = text.xml_content().unwrap();
                    let text = resolve_xml_entity(&text).unwrap();
                    f(self, Content::Text(text));
                }
                Event::Empty(start) => {
                    let is_empty = true;
                    f(self, Content::Elem(Elem { is_empty, start }));
                }
                Event::Start(start) => {
                    let is_empty = false;
                    f(self, Content::Elem(Elem { is_empty, start }));
                }
                Event::End(end) => {
                    assert_eq!(end.name(), elem.start.name());
                    break;
                }
                event => {
                    panic!("unexpected event: {event:?}");
                }
            }
            buf.clear();
        }
    }

    fn parse_file(&mut self) -> Protocol {
        let mut protocol = None;

        let mut buf = Vec::new();
        loop {
            match self.next_event(&mut buf) {
                Event::Decl(_) => {
                    ();
                }
                Event::Text(text) => {
                    self.assert_is_ws(&text);
                }
                Event::Start(start) => match start.name().as_ref() {
                    b"protocol" => {
                        let is_empty = false;
                        assert_eq!(protocol, None);
                        protocol = Some(self.parse_protocol(Elem { is_empty, start }));
                    }
                    _ => {
                        panic!("unexpected elem: {start:?}");
                    }
                },
                Event::Eof => {
                    break;
                }
                event => {
                    panic!("unexpected event: {event:?}");
                }
            }
            buf.clear();
        }

        protocol.unwrap()
    }

    fn parse_protocol(&mut self, elem: Elem) -> Protocol {
        let mut name = None;

        for attr in elem.start.attributes() {
            let attr = attr.unwrap();
            match attr.key.as_ref() {
                b"name" => self.save_attr(attr, &mut name),
                _ => panic!("unexpected attr: {attr:?}"),
            }
        }

        let mut contents = Vec::new();
        self.parse_contents(elem, |this, content| match content {
            Content::Text(text) => this.assert_is_ws(text.as_bytes()),
            Content::Elem(elem) => match elem.start.name().as_ref() {
                b"copyright" => {
                    let text = this.parse_text_elem(elem);
                    contents.push(ProtocolContent::Copyright(text));
                }
                b"interface" => {
                    let interface = this.parse_interface(elem);
                    contents.push(ProtocolContent::Interface(interface));
                }
                _ => {
                    panic!("unexpected elem: {elem:?}");
                }
            },
        });

        Protocol { name, contents }
    }

    fn parse_text_elem(&mut self, elem: Elem) -> String {
        for attr in elem.start.attributes() {
            let attr = attr.unwrap();
            match attr.key.as_ref() {
                _ => panic!("unexpected attr: {attr:?}"),
            }
        }

        let mut contents = String::new();
        self.parse_contents(elem, |_this, content| match content {
            Content::Text(text) => contents += text,
            Content::Elem(elem) => panic!("unexpected elem: {elem:?}"),
        });

        contents
    }

    fn parse_description(&mut self, elem: Elem) -> Description {
        let mut summary = None;

        for attr in elem.start.attributes() {
            let attr = attr.unwrap();
            match attr.key.as_ref() {
                b"summary" => self.save_attr(attr, &mut summary),
                _ => panic!("unexpected attr: {attr:?}"),
            }
        }

        let mut contents = String::new();
        self.parse_contents(elem, |_this, content| match content {
            Content::Text(text) => contents += text,
            Content::Elem(elem) => panic!("unexpected elem: {elem:?}"),
        });

        Description { summary, contents }
    }

    fn parse_interface(&mut self, elem: Elem) -> Interface {
        let mut name = None;
        let mut version = None;

        for attr in elem.start.attributes() {
            let attr = attr.unwrap();
            match attr.key.as_ref() {
                b"name" => self.save_attr(attr, &mut name),
                b"version" => self.save_attr(attr, &mut version),
                _ => panic!("unexpected attr: {attr:?}"),
            }
        }

        let mut contents = Vec::new();
        self.parse_contents(elem, |this, content| match content {
            Content::Text(text) => this.assert_is_ws(text.as_bytes()),
            Content::Elem(elem) => match elem.start.name().as_ref() {
                b"description" => {
                    let description = this.parse_description(elem);
                    contents.push(InterfaceContent::Description(description));
                }
                b"request" => {
                    let request = this.parse_request_or_event(elem);
                    contents.push(InterfaceContent::Request(request));
                }
                b"event" => {
                    let event = this.parse_request_or_event(elem);
                    contents.push(InterfaceContent::Event(event));
                }
                b"enum" => {
                    let enu = this.parse_enum(elem);
                    contents.push(InterfaceContent::Enum(enu));
                }
                _ => {
                    panic!("unexpected elem: {elem:?}");
                }
            },
        });

        Interface {
            name,
            version,
            contents,
        }
    }

    fn parse_request_or_event(&mut self, elem: Elem) -> RequestOrEvent {
        let mut name = None;
        let mut typ = None;
        let mut since = None;
        let mut deprecated_since = None;

        for attr in elem.start.attributes() {
            let attr = attr.unwrap();
            match attr.key.as_ref() {
                b"name" => self.save_attr(attr, &mut name),
                b"type" => self.save_attr(attr, &mut typ),
                b"since" => self.save_attr(attr, &mut since),
                b"deprecated-since" => self.save_attr(attr, &mut deprecated_since),
                _ => panic!("unexpected attr: {attr:?}"),
            }
        }

        let mut contents = Vec::new();
        self.parse_contents(elem, |this, content| match content {
            Content::Text(text) => this.assert_is_ws(text.as_bytes()),
            Content::Elem(elem) => match elem.start.name().as_ref() {
                b"description" => {
                    let description = this.parse_description(elem);
                    contents.push(RequestOrEventContent::Description(description));
                }
                b"arg" => {
                    let arg = this.parse_arg(elem);
                    contents.push(RequestOrEventContent::Arg(arg));
                }
                _ => {
                    panic!("unexpected elem: {elem:?}");
                }
            },
        });

        RequestOrEvent {
            name,
            typ,
            since,
            deprecated_since,
            contents,
        }
    }

    fn parse_arg(&mut self, elem: Elem) -> Arg {
        let mut name = None;
        let mut typ = None;
        let mut interface = None;
        let mut enu = None;
        let mut allow_null = None;
        let mut summary = None;

        for attr in elem.start.attributes() {
            let attr = attr.unwrap();
            match attr.key.as_ref() {
                b"name" => self.save_attr(attr, &mut name),
                b"type" => self.save_attr(attr, &mut typ),
                b"interface" => self.save_attr(attr, &mut interface),
                b"enum" => self.save_attr(attr, &mut enu),
                b"allow-null" => self.save_attr(attr, &mut allow_null),
                b"summary" => self.save_attr(attr, &mut summary),
                _ => panic!("unexpected attr: {attr:?}"),
            }
        }

        assert_eq!(elem.is_empty, true);
        Arg {
            name,
            typ,
            interface,
            enu,
            allow_null,
            summary,
        }
    }

    fn parse_enum(&mut self, elem: Elem) -> Enum {
        let mut name = None;
        let mut bitfield = None;
        let mut since = None;

        for attr in elem.start.attributes() {
            let attr = attr.unwrap();
            match attr.key.as_ref() {
                b"name" => self.save_attr(attr, &mut name),
                b"bitfield" => self.save_attr(attr, &mut bitfield),
                b"since" => self.save_attr(attr, &mut since),
                _ => panic!("unexpected attr: {attr:?}"),
            }
        }

        let mut contents = Vec::new();
        self.parse_contents(elem, |this, content| match content {
            Content::Text(text) => this.assert_is_ws(text.as_bytes()),
            Content::Elem(elem) => match elem.start.name().as_ref() {
                b"description" => {
                    let description = this.parse_description(elem);
                    contents.push(EnumContent::Description(description));
                }
                b"entry" => {
                    let entry = this.parse_entry(elem);
                    contents.push(EnumContent::Entry(entry));
                }
                _ => {
                    panic!("unexpected elem: {elem:?}");
                }
            },
        });

        Enum {
            name,
            bitfield,
            since,
            contents,
        }
    }

    fn parse_entry(&mut self, elem: Elem) -> Entry {
        let mut name = None;
        let mut value = None;
        let mut since = None;
        let mut summary = None;

        for attr in elem.start.attributes() {
            let attr = attr.unwrap();
            match attr.key.as_ref() {
                b"name" => self.save_attr(attr, &mut name),
                b"value" => self.save_attr(attr, &mut value),
                b"since" => self.save_attr(attr, &mut since),
                b"summary" => self.save_attr(attr, &mut summary),
                _ => panic!("unexpected attr: {attr:?}"),
            }
        }

        assert_eq!(elem.is_empty, true);
        Entry {
            name,
            value,
            since,
            summary,
        }
    }
}
