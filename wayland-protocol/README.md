# Wayland Protocol for Rust

This library parses parses Wayland protocol XML files into Rust data structures.

## Example

List all Wayland interfaces:

```rust
fn main() {
    use wayland_protocol::*;

    let protocol = Protocol::wayland(); // wayland.xml
    for content in &protocol.contents {
        if let ProtocolContent::Interface(interface) = content {
            println!("{:?}", interface.name);
        }
    }
}
```
