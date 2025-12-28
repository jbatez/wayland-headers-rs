use crate::prelude::*;

use super::wayland_client::*;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct wl_cursor {
    pub image_count: c_uint,
    pub images: *mut *mut wl_cursor_image,
    pub name: *mut c_char,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct wl_cursor_image {
    pub width: u32,
    pub height: u32,
    pub hotspot_x: u32,
    pub hotspot_y: u32,
    pub delay: u32,
}

#[repr(C)]
pub struct wl_cursor_theme {
    _data: (),
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

unsafe extern "C" {
    pub fn wl_cursor_frame(cursor: *mut wl_cursor, time: u32) -> c_int;

    pub fn wl_cursor_frame_and_duration(
        cursor: *mut wl_cursor,
        time: u32,
        duration: *mut u32,
    ) -> c_int;

    pub fn wl_cursor_image_get_buffer(image: *mut wl_cursor_image) -> *mut wl_buffer;

    pub fn wl_cursor_theme_destroy(theme: *mut wl_cursor_theme);

    pub fn wl_cursor_theme_get_cursor(
        theme: *mut wl_cursor_theme,
        name: *const c_char,
    ) -> *mut wl_cursor;

    pub fn wl_cursor_theme_load(
        name: *const c_char,
        size: c_int,
        shm: *mut wl_shm,
    ) -> *mut wl_cursor_theme;
}
