use crate::prelude::*;

use super::wayland_server::*;

#[repr(C)]
pub struct wp_viewport {
    _data: (),
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct wp_viewport_interface {
    pub destroy: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
    )>,
    pub set_source: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
        x: wl_fixed_t,
        y: wl_fixed_t,
        width: wl_fixed_t,
        height: wl_fixed_t,
    )>,
    pub set_destination: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
        width: i32,
        height: i32,
    )>,
}

#[repr(C)]
pub struct wp_viewporter {
    _data: (),
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct wp_viewporter_interface {
    pub destroy: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
    )>,
    pub get_viewport: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
        id: u32,
        surface: *mut wl_resource,
    )>,
}

pub const WP_VIEWPORTER_DESTROY_SINCE_VERSION: u32 = 1;
pub const WP_VIEWPORTER_ERROR_VIEWPORT_EXISTS: u32 = 0;
pub const WP_VIEWPORTER_GET_VIEWPORT_SINCE_VERSION: u32 = 1;
pub const WP_VIEWPORT_DESTROY_SINCE_VERSION: u32 = 1;
pub const WP_VIEWPORT_ERROR_BAD_SIZE: u32 = 1;
pub const WP_VIEWPORT_ERROR_BAD_VALUE: u32 = 0;
pub const WP_VIEWPORT_ERROR_NO_SURFACE: u32 = 3;
pub const WP_VIEWPORT_ERROR_OUT_OF_BUFFER: u32 = 2;
pub const WP_VIEWPORT_SET_DESTINATION_SINCE_VERSION: u32 = 1;
pub const WP_VIEWPORT_SET_SOURCE_SINCE_VERSION: u32 = 1;

unsafe extern "C" {
    pub static wp_viewport_interface: wl_interface;
    pub static wp_viewporter_interface: wl_interface;
}
