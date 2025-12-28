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
    Request(Message),
    Event(Message),
    Enum(Enum),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Message {
    pub name: Option<String>,
    pub since: Option<String>,
    pub typ: Option<String>,
    pub contents: Vec<MessageContent>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MessageContent {
    Description(Description),
    Arg(Arg),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Arg {
    pub allow_null: Option<String>,
    pub enu: Option<String>,
    pub interface: Option<String>,
    pub name: Option<String>,
    pub summary: Option<String>,
    pub typ: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Enum {
    pub bitfield: Option<String>,
    pub name: Option<String>,
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
    pub since: Option<String>,
    pub summary: Option<String>,
    pub value: Option<String>,
}
