pub use self::protocol::*;

mod parser;
mod protocol;

#[cfg(test)]
#[test]
fn test() {
    Protocol::wayland();
}
