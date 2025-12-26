#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Protocol {
    pub name: Option<String>,
    pub contents: Vec<ProtocolContent>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProtocolContent {
    Copyright(String),
    Interface(Interface),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Description {
    pub summary: Option<String>,
    pub contents: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Interface {
    pub name: Option<String>,
    pub version: Option<String>,
    pub contents: Vec<InterfaceContent>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum InterfaceContent {
    Description(Description),
    Request(RequestOrEvent),
    Event(RequestOrEvent),
    Enum(Enum),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RequestOrEvent {
    pub name: Option<String>,
    pub typ: Option<String>,
    pub since: Option<String>,
    pub deprecated_since: Option<String>,
    pub contents: Vec<RequestOrEventContent>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RequestOrEventContent {
    Description(Description),
    Arg(Arg),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Arg {
    pub name: Option<String>,
    pub typ: Option<String>,
    pub interface: Option<String>,
    pub enu: Option<String>,
    pub allow_null: Option<String>,
    pub summary: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Enum {
    pub name: Option<String>,
    pub bitfield: Option<String>,
    pub since: Option<String>,
    pub contents: Vec<EnumContent>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EnumContent {
    Description(Description),
    Entry(Entry),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Entry {
    pub name: Option<String>,
    pub value: Option<String>,
    pub since: Option<String>,
    pub summary: Option<String>,
}
