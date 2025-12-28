# Generate Wayland Headers for Rust

This program uses [wayland-protocol](../wayland-protocol) to generate
[wayland_client_protocol.rs](../wayland-headers/src/wayland_client_protocol.rs)
and
[wayland_server_protocol.rs](../wayland-headers/src/wayland_server_protocol.rs)
for the [wayland-headers](../wayland-headers) Rust library.

## Usage

In the root directory of this repository (the parent of this directory), run:

```
cargo run -p wayland-headers-generator
```
