use crate::prelude::*;

use super::wayland_client::*;

#[repr(C)]
pub struct wl_egl_window {
    _data: (),
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

pub const WL_EGL_PLATFORM: c_int = 1;

unsafe extern "C" {
    pub fn wl_egl_window_create(
        surface: *mut wl_surface,
        width: c_int,
        height: c_int,
    ) -> *mut wl_egl_window;

    pub fn wl_egl_window_destroy(egl_window: *mut wl_egl_window);

    pub fn wl_egl_window_get_attached_size(
        egl_window: *mut wl_egl_window,
        width: *mut c_int,
        height: *mut c_int,
    );

    pub fn wl_egl_window_resize(
        egl_window: *mut wl_egl_window,
        width: c_int,
        height: c_int,
        dx: c_int,
        dy: c_int,
    );
}
