This repository contains three projects:

  * [wayland-headers](wayland-headers) contains minimalist Rust FFI bindings for
    Wayland in a way that's roughly equivalent to the official
    [Wayland 1.18.0](https://gitlab.freedesktop.org/wayland/wayland/-/tree/1.18.0?ref_type=tags)
    headers for C/C++.
  * [wayland-headers-generator](wayland-headers-generator) uses
    `wayland-protocol` to generate the `wayland-headers` Rust library.
  * [wayland-protocol](wayland-protocol) parses
    [wayland.xml](https://gitlab.freedesktop.org/wayland/wayland/-/blob/1.18.0/protocol/wayland.xml?ref_type=tags)
    into Rust data structures.
