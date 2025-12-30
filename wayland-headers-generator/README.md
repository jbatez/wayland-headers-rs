# Generate Wayland Headers for Rust

This program uses [wayland-protocol](../wayland-protocol) to generate `*_protocol` modules for the [wayland-headers](../wayland-headers) Rust library.

## Usage

In the root directory of this repository (the parent of this directory), run:

```
cargo run -p wayland-headers-generator
```
