# Wayland Protocol Statics

This library provides definitions for stable Wayland extension protocol `wl_interface` statics.

Specifically, it includes the C source file results of running `wayland-scanner {public,private}-code` with [wayland-protocols 1.20](https://gitlab.freedesktop.org/wayland/wayland-protocols/-/tree/1.20?ref_type=tags) stable XML files and automatically compiles/links them into your Rust program at build time.

Each protocol has a corresponding `export-<protocol>` and `private-<protocol>` feature. The `export` version will define the symbols with `WL_EXPORT` (i.e. `__attribute__ ((visibility("default")))`) whereas the `private` version will define the symbols with `WL_PRIVATE` (i.e. `__attribute__ ((visibility("hidden")))`). If both features are enabled, the `export` version takes precedence.

For example, if you need `xdg-shell` statics defined in your program and don't need them exported from your binary, include the following in your `Cargo.toml`:

```toml
[dependencies.wayland-protocol-statics]
version = "0.1"
features = ["private-xdg-shell"]
```

This library only provides C definitions. To access them from Rust, see [wayland-headers](https://crates.io/crates/wayland-headers).
