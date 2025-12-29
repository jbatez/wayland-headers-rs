This repository contains four projects:

  * [wayland-headers](wayland-headers) contains minimalist Rust FFI bindings for
    Wayland in a way that's roughly equivalent to the official
    [Wayland 1.18.0](https://gitlab.freedesktop.org/wayland/wayland/-/tree/1.18.0?ref_type=tags)
    headers for C/C++ along with the stable protocols found in
    [wayland-protocols 1.20](https://gitlab.freedesktop.org/wayland/wayland-protocols/-/tree/1.20?ref_type=tags).
  * [wayland-headers-generator](wayland-headers-generator) uses
    `wayland-protocol` to generate `*_protocol` modules for the
    `wayland-headers` Rust library.
  * [wayland-protocol](wayland-protocol) parses Wayland protocol XML files into
    Rust data structures.
  * [wayland-protocol-statics](wayland-protocol-statics) provides definitions
    for stable Wayland extension protocol `wl_interface` statics.
