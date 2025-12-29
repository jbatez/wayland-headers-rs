use crate::prelude::*;

use super::wayland_client::*;

#[repr(C)]
pub struct wp_viewport {
    _data: (),
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[repr(C)]
pub struct wp_viewporter {
    _data: (),
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

pub const WP_VIEWPORTER_DESTROY: u32 = 0;
pub const WP_VIEWPORTER_DESTROY_SINCE_VERSION: u32 = 1;
pub const WP_VIEWPORTER_ERROR_VIEWPORT_EXISTS: u32 = 0;
pub const WP_VIEWPORTER_GET_VIEWPORT: u32 = 1;
pub const WP_VIEWPORTER_GET_VIEWPORT_SINCE_VERSION: u32 = 1;
pub const WP_VIEWPORT_DESTROY: u32 = 0;
pub const WP_VIEWPORT_DESTROY_SINCE_VERSION: u32 = 1;
pub const WP_VIEWPORT_ERROR_BAD_SIZE: u32 = 1;
pub const WP_VIEWPORT_ERROR_BAD_VALUE: u32 = 0;
pub const WP_VIEWPORT_ERROR_NO_SURFACE: u32 = 3;
pub const WP_VIEWPORT_ERROR_OUT_OF_BUFFER: u32 = 2;
pub const WP_VIEWPORT_SET_DESTINATION: u32 = 2;
pub const WP_VIEWPORT_SET_DESTINATION_SINCE_VERSION: u32 = 1;
pub const WP_VIEWPORT_SET_SOURCE: u32 = 1;
pub const WP_VIEWPORT_SET_SOURCE_SINCE_VERSION: u32 = 1;

unsafe extern "C" {
    pub static wp_viewport_interface: wl_interface;
    pub static wp_viewporter_interface: wl_interface;
}

#[inline]
pub unsafe extern "C" fn wp_viewport_destroy(
    wp_viewport: *mut wp_viewport,
) {
    unsafe {
        wl_proxy_marshal(
            wp_viewport.cast(),
            WP_VIEWPORT_DESTROY,
        );
        wl_proxy_destroy(
            wp_viewport.cast(),
        );
    }
}

#[inline]
pub unsafe extern "C" fn wp_viewport_get_user_data(
    wp_viewport: *mut wp_viewport,
) -> *mut c_void {
    unsafe { wl_proxy_get_user_data(wp_viewport.cast()) }
}

#[inline]
pub unsafe extern "C" fn wp_viewport_get_version(
    wp_viewport: *mut wp_viewport,
) -> u32 {
    unsafe { wl_proxy_get_version(wp_viewport.cast()) }
}

#[inline]
pub unsafe extern "C" fn wp_viewport_set_destination(
    wp_viewport: *mut wp_viewport,
    width: i32,
    height: i32,
) {
    unsafe {
        wl_proxy_marshal(
            wp_viewport.cast(),
            WP_VIEWPORT_SET_DESTINATION,
            width,
            height,
        );
    }
}

#[inline]
pub unsafe extern "C" fn wp_viewport_set_source(
    wp_viewport: *mut wp_viewport,
    x: wl_fixed_t,
    y: wl_fixed_t,
    width: wl_fixed_t,
    height: wl_fixed_t,
) {
    unsafe {
        wl_proxy_marshal(
            wp_viewport.cast(),
            WP_VIEWPORT_SET_SOURCE,
            x,
            y,
            width,
            height,
        );
    }
}

#[inline]
pub unsafe extern "C" fn wp_viewport_set_user_data(
    wp_viewport: *mut wp_viewport,
    user_data: *mut c_void,
) {
    unsafe {
        wl_proxy_set_user_data(
            wp_viewport.cast(),
            user_data,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wp_viewporter_destroy(
    wp_viewporter: *mut wp_viewporter,
) {
    unsafe {
        wl_proxy_marshal(
            wp_viewporter.cast(),
            WP_VIEWPORTER_DESTROY,
        );
        wl_proxy_destroy(
            wp_viewporter.cast(),
        );
    }
}

#[inline]
pub unsafe extern "C" fn wp_viewporter_get_user_data(
    wp_viewporter: *mut wp_viewporter,
) -> *mut c_void {
    unsafe { wl_proxy_get_user_data(wp_viewporter.cast()) }
}

#[inline]
pub unsafe extern "C" fn wp_viewporter_get_version(
    wp_viewporter: *mut wp_viewporter,
) -> u32 {
    unsafe { wl_proxy_get_version(wp_viewporter.cast()) }
}

#[inline]
pub unsafe extern "C" fn wp_viewporter_get_viewport(
    wp_viewporter: *mut wp_viewporter,
    surface: *mut wl_surface,
) -> *mut wp_viewport {
    unsafe {
        wl_proxy_marshal_constructor(
            wp_viewporter.cast(),
            WP_VIEWPORTER_GET_VIEWPORT,
            &wp_viewport_interface,
            null_mut::<c_void>(),
            surface,
        ).cast()
    }
}

#[inline]
pub unsafe extern "C" fn wp_viewporter_set_user_data(
    wp_viewporter: *mut wp_viewporter,
    user_data: *mut c_void,
) {
    unsafe {
        wl_proxy_set_user_data(
            wp_viewporter.cast(),
            user_data,
        )
    }
}
