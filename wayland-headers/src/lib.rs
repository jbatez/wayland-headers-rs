#![allow(nonstandard_style)]
#![doc = include_str!("../README.md")]
#![no_std]

#[doc(hidden)]
pub mod _macro_helpers;

mod prelude;

pub mod presentation_time_client_protocol;
pub mod presentation_time_server_protocol;
pub mod viewporter_client_protocol;
pub mod viewporter_server_protocol;
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
pub mod xdg_shell_client_protocol;
pub mod xdg_shell_server_protocol;
