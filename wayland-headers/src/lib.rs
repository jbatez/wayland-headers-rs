#![allow(nonstandard_style)]
#![cfg_attr(not(any(doc, feature = "std")), no_std)]

#[doc(hidden)]
pub mod _macro_helpers;

mod prelude;

pub mod wayland_client;
pub mod wayland_client_core;
pub mod wayland_client_protocol;
pub mod wayland_cursor;
pub mod wayland_egl;
pub mod wayland_egl_core;
pub mod wayland_server;
pub mod wayland_server_core;
pub mod wayland_server_protocol;
pub mod wayland_util;
pub mod wayland_version;
