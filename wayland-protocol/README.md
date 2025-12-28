# Vulkan API Registry for Rust

This library parses
[wayland.xml](https://gitlab.freedesktop.org/wayland/wayland/-/blob/1.18.0/protocol/wayland.xml?ref_type=tags)
into Rust data structures.

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
