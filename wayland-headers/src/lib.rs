#![allow(nonstandard_style)]
#![cfg_attr(not(any(doc, feature = "std")), no_std)]

#[doc(hidden)]
pub mod _macro_helpers;

pub mod wayland_client_core;
pub mod wayland_client_protocol;
pub mod wayland_server_core;
pub mod wayland_server_protocol;
pub mod wayland_util;
pub mod wayland_version;
