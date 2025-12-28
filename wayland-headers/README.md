# Wayland Headers for Rust

This library contains minimalist Rust FFI bindings for Wayland in a way that's
roughly equivalent to the official
[Wayland 1.18.0](https://gitlab.freedesktop.org/wayland/wayland/-/tree/1.18.0?ref_type=tags)
headers for C/C++. It makes no attempt at providing safe or idiomatic Rust
wrappers and doesn't rename any C identifiers to match Rust's style guidelines.

For example, the following Rust code:

```rust
use wayland_headers::wayland_client::*;
```

is roughly equivalent to the following C code:

```c
#include <wayland-client.h>
```

This library is based specifically on Wayland 1.18.0 since that's the version
provided by
[Steam Runtime 3 'sniper'](https://gitlab.steamos.cloud/steamrt/steamrt/-/blob/steamrt/sniper/README.md).

Using this library does not automatically link against any `libwayland-*.so`
libraries.
