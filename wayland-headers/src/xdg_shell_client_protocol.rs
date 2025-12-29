use crate::prelude::*;

use super::wayland_client::*;

#[repr(C)]
pub struct xdg_popup {
    _data: (),
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct xdg_popup_listener {
    pub configure: Option<unsafe extern "C" fn(
        data: *mut c_void,
        xdg_popup: *mut xdg_popup,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    )>,
    pub popup_done: Option<unsafe extern "C" fn(
        data: *mut c_void,
        xdg_popup: *mut xdg_popup,
    )>,
    pub repositioned: Option<unsafe extern "C" fn(
        data: *mut c_void,
        xdg_popup: *mut xdg_popup,
        token: u32,
    )>,
}

#[repr(C)]
pub struct xdg_positioner {
    _data: (),
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[repr(C)]
pub struct xdg_surface {
    _data: (),
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct xdg_surface_listener {
    pub configure: Option<unsafe extern "C" fn(
        data: *mut c_void,
        xdg_surface: *mut xdg_surface,
        serial: u32,
    )>,
}

#[repr(C)]
pub struct xdg_toplevel {
    _data: (),
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct xdg_toplevel_listener {
    pub configure: Option<unsafe extern "C" fn(
        data: *mut c_void,
        xdg_toplevel: *mut xdg_toplevel,
        width: i32,
        height: i32,
        states: *mut wl_array,
    )>,
    pub close: Option<unsafe extern "C" fn(
        data: *mut c_void,
        xdg_toplevel: *mut xdg_toplevel,
    )>,
}

#[repr(C)]
pub struct xdg_wm_base {
    _data: (),
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct xdg_wm_base_listener {
    pub ping: Option<unsafe extern "C" fn(
        data: *mut c_void,
        xdg_wm_base: *mut xdg_wm_base,
        serial: u32,
    )>,
}

pub const XDG_POPUP_CONFIGURE_SINCE_VERSION: u32 = 1;
pub const XDG_POPUP_DESTROY: u32 = 0;
pub const XDG_POPUP_DESTROY_SINCE_VERSION: u32 = 1;
pub const XDG_POPUP_ERROR_INVALID_GRAB: u32 = 0;
pub const XDG_POPUP_GRAB: u32 = 1;
pub const XDG_POPUP_GRAB_SINCE_VERSION: u32 = 1;
pub const XDG_POPUP_POPUP_DONE_SINCE_VERSION: u32 = 1;
pub const XDG_POPUP_REPOSITION: u32 = 2;
pub const XDG_POPUP_REPOSITIONED_SINCE_VERSION: u32 = 3;
pub const XDG_POPUP_REPOSITION_SINCE_VERSION: u32 = 3;
pub const XDG_POSITIONER_ANCHOR_BOTTOM: u32 = 2;
pub const XDG_POSITIONER_ANCHOR_BOTTOM_LEFT: u32 = 6;
pub const XDG_POSITIONER_ANCHOR_BOTTOM_RIGHT: u32 = 8;
pub const XDG_POSITIONER_ANCHOR_LEFT: u32 = 3;
pub const XDG_POSITIONER_ANCHOR_NONE: u32 = 0;
pub const XDG_POSITIONER_ANCHOR_RIGHT: u32 = 4;
pub const XDG_POSITIONER_ANCHOR_TOP: u32 = 1;
pub const XDG_POSITIONER_ANCHOR_TOP_LEFT: u32 = 5;
pub const XDG_POSITIONER_ANCHOR_TOP_RIGHT: u32 = 7;
pub const XDG_POSITIONER_CONSTRAINT_ADJUSTMENT_FLIP_X: u32 = 4;
pub const XDG_POSITIONER_CONSTRAINT_ADJUSTMENT_FLIP_Y: u32 = 8;
pub const XDG_POSITIONER_CONSTRAINT_ADJUSTMENT_NONE: u32 = 0;
pub const XDG_POSITIONER_CONSTRAINT_ADJUSTMENT_RESIZE_X: u32 = 16;
pub const XDG_POSITIONER_CONSTRAINT_ADJUSTMENT_RESIZE_Y: u32 = 32;
pub const XDG_POSITIONER_CONSTRAINT_ADJUSTMENT_SLIDE_X: u32 = 1;
pub const XDG_POSITIONER_CONSTRAINT_ADJUSTMENT_SLIDE_Y: u32 = 2;
pub const XDG_POSITIONER_DESTROY: u32 = 0;
pub const XDG_POSITIONER_DESTROY_SINCE_VERSION: u32 = 1;
pub const XDG_POSITIONER_ERROR_INVALID_INPUT: u32 = 0;
pub const XDG_POSITIONER_GRAVITY_BOTTOM: u32 = 2;
pub const XDG_POSITIONER_GRAVITY_BOTTOM_LEFT: u32 = 6;
pub const XDG_POSITIONER_GRAVITY_BOTTOM_RIGHT: u32 = 8;
pub const XDG_POSITIONER_GRAVITY_LEFT: u32 = 3;
pub const XDG_POSITIONER_GRAVITY_NONE: u32 = 0;
pub const XDG_POSITIONER_GRAVITY_RIGHT: u32 = 4;
pub const XDG_POSITIONER_GRAVITY_TOP: u32 = 1;
pub const XDG_POSITIONER_GRAVITY_TOP_LEFT: u32 = 5;
pub const XDG_POSITIONER_GRAVITY_TOP_RIGHT: u32 = 7;
pub const XDG_POSITIONER_SET_ANCHOR: u32 = 3;
pub const XDG_POSITIONER_SET_ANCHOR_RECT: u32 = 2;
pub const XDG_POSITIONER_SET_ANCHOR_RECT_SINCE_VERSION: u32 = 1;
pub const XDG_POSITIONER_SET_ANCHOR_SINCE_VERSION: u32 = 1;
pub const XDG_POSITIONER_SET_CONSTRAINT_ADJUSTMENT: u32 = 5;
pub const XDG_POSITIONER_SET_CONSTRAINT_ADJUSTMENT_SINCE_VERSION: u32 = 1;
pub const XDG_POSITIONER_SET_GRAVITY: u32 = 4;
pub const XDG_POSITIONER_SET_GRAVITY_SINCE_VERSION: u32 = 1;
pub const XDG_POSITIONER_SET_OFFSET: u32 = 6;
pub const XDG_POSITIONER_SET_OFFSET_SINCE_VERSION: u32 = 1;
pub const XDG_POSITIONER_SET_PARENT_CONFIGURE: u32 = 9;
pub const XDG_POSITIONER_SET_PARENT_CONFIGURE_SINCE_VERSION: u32 = 3;
pub const XDG_POSITIONER_SET_PARENT_SIZE: u32 = 8;
pub const XDG_POSITIONER_SET_PARENT_SIZE_SINCE_VERSION: u32 = 3;
pub const XDG_POSITIONER_SET_REACTIVE: u32 = 7;
pub const XDG_POSITIONER_SET_REACTIVE_SINCE_VERSION: u32 = 3;
pub const XDG_POSITIONER_SET_SIZE: u32 = 1;
pub const XDG_POSITIONER_SET_SIZE_SINCE_VERSION: u32 = 1;
pub const XDG_SURFACE_ACK_CONFIGURE: u32 = 4;
pub const XDG_SURFACE_ACK_CONFIGURE_SINCE_VERSION: u32 = 1;
pub const XDG_SURFACE_CONFIGURE_SINCE_VERSION: u32 = 1;
pub const XDG_SURFACE_DESTROY: u32 = 0;
pub const XDG_SURFACE_DESTROY_SINCE_VERSION: u32 = 1;
pub const XDG_SURFACE_ERROR_ALREADY_CONSTRUCTED: u32 = 2;
pub const XDG_SURFACE_ERROR_NOT_CONSTRUCTED: u32 = 1;
pub const XDG_SURFACE_ERROR_UNCONFIGURED_BUFFER: u32 = 3;
pub const XDG_SURFACE_GET_POPUP: u32 = 2;
pub const XDG_SURFACE_GET_POPUP_SINCE_VERSION: u32 = 1;
pub const XDG_SURFACE_GET_TOPLEVEL: u32 = 1;
pub const XDG_SURFACE_GET_TOPLEVEL_SINCE_VERSION: u32 = 1;
pub const XDG_SURFACE_SET_WINDOW_GEOMETRY: u32 = 3;
pub const XDG_SURFACE_SET_WINDOW_GEOMETRY_SINCE_VERSION: u32 = 1;
pub const XDG_TOPLEVEL_CLOSE_SINCE_VERSION: u32 = 1;
pub const XDG_TOPLEVEL_CONFIGURE_SINCE_VERSION: u32 = 1;
pub const XDG_TOPLEVEL_DESTROY: u32 = 0;
pub const XDG_TOPLEVEL_DESTROY_SINCE_VERSION: u32 = 1;
pub const XDG_TOPLEVEL_MOVE: u32 = 5;
pub const XDG_TOPLEVEL_MOVE_SINCE_VERSION: u32 = 1;
pub const XDG_TOPLEVEL_RESIZE: u32 = 6;
pub const XDG_TOPLEVEL_RESIZE_EDGE_BOTTOM: u32 = 2;
pub const XDG_TOPLEVEL_RESIZE_EDGE_BOTTOM_LEFT: u32 = 6;
pub const XDG_TOPLEVEL_RESIZE_EDGE_BOTTOM_RIGHT: u32 = 10;
pub const XDG_TOPLEVEL_RESIZE_EDGE_LEFT: u32 = 4;
pub const XDG_TOPLEVEL_RESIZE_EDGE_NONE: u32 = 0;
pub const XDG_TOPLEVEL_RESIZE_EDGE_RIGHT: u32 = 8;
pub const XDG_TOPLEVEL_RESIZE_EDGE_TOP: u32 = 1;
pub const XDG_TOPLEVEL_RESIZE_EDGE_TOP_LEFT: u32 = 5;
pub const XDG_TOPLEVEL_RESIZE_EDGE_TOP_RIGHT: u32 = 9;
pub const XDG_TOPLEVEL_RESIZE_SINCE_VERSION: u32 = 1;
pub const XDG_TOPLEVEL_SET_APP_ID: u32 = 3;
pub const XDG_TOPLEVEL_SET_APP_ID_SINCE_VERSION: u32 = 1;
pub const XDG_TOPLEVEL_SET_FULLSCREEN: u32 = 11;
pub const XDG_TOPLEVEL_SET_FULLSCREEN_SINCE_VERSION: u32 = 1;
pub const XDG_TOPLEVEL_SET_MAXIMIZED: u32 = 9;
pub const XDG_TOPLEVEL_SET_MAXIMIZED_SINCE_VERSION: u32 = 1;
pub const XDG_TOPLEVEL_SET_MAX_SIZE: u32 = 7;
pub const XDG_TOPLEVEL_SET_MAX_SIZE_SINCE_VERSION: u32 = 1;
pub const XDG_TOPLEVEL_SET_MINIMIZED: u32 = 13;
pub const XDG_TOPLEVEL_SET_MINIMIZED_SINCE_VERSION: u32 = 1;
pub const XDG_TOPLEVEL_SET_MIN_SIZE: u32 = 8;
pub const XDG_TOPLEVEL_SET_MIN_SIZE_SINCE_VERSION: u32 = 1;
pub const XDG_TOPLEVEL_SET_PARENT: u32 = 1;
pub const XDG_TOPLEVEL_SET_PARENT_SINCE_VERSION: u32 = 1;
pub const XDG_TOPLEVEL_SET_TITLE: u32 = 2;
pub const XDG_TOPLEVEL_SET_TITLE_SINCE_VERSION: u32 = 1;
pub const XDG_TOPLEVEL_SHOW_WINDOW_MENU: u32 = 4;
pub const XDG_TOPLEVEL_SHOW_WINDOW_MENU_SINCE_VERSION: u32 = 1;
pub const XDG_TOPLEVEL_STATE_ACTIVATED: u32 = 4;
pub const XDG_TOPLEVEL_STATE_FULLSCREEN: u32 = 2;
pub const XDG_TOPLEVEL_STATE_MAXIMIZED: u32 = 1;
pub const XDG_TOPLEVEL_STATE_RESIZING: u32 = 3;
pub const XDG_TOPLEVEL_STATE_TILED_BOTTOM: u32 = 8;
pub const XDG_TOPLEVEL_STATE_TILED_BOTTOM_SINCE_VERSION: u32 = 2;
pub const XDG_TOPLEVEL_STATE_TILED_LEFT: u32 = 5;
pub const XDG_TOPLEVEL_STATE_TILED_LEFT_SINCE_VERSION: u32 = 2;
pub const XDG_TOPLEVEL_STATE_TILED_RIGHT: u32 = 6;
pub const XDG_TOPLEVEL_STATE_TILED_RIGHT_SINCE_VERSION: u32 = 2;
pub const XDG_TOPLEVEL_STATE_TILED_TOP: u32 = 7;
pub const XDG_TOPLEVEL_STATE_TILED_TOP_SINCE_VERSION: u32 = 2;
pub const XDG_TOPLEVEL_UNSET_FULLSCREEN: u32 = 12;
pub const XDG_TOPLEVEL_UNSET_FULLSCREEN_SINCE_VERSION: u32 = 1;
pub const XDG_TOPLEVEL_UNSET_MAXIMIZED: u32 = 10;
pub const XDG_TOPLEVEL_UNSET_MAXIMIZED_SINCE_VERSION: u32 = 1;
pub const XDG_WM_BASE_CREATE_POSITIONER: u32 = 1;
pub const XDG_WM_BASE_CREATE_POSITIONER_SINCE_VERSION: u32 = 1;
pub const XDG_WM_BASE_DESTROY: u32 = 0;
pub const XDG_WM_BASE_DESTROY_SINCE_VERSION: u32 = 1;
pub const XDG_WM_BASE_ERROR_DEFUNCT_SURFACES: u32 = 1;
pub const XDG_WM_BASE_ERROR_INVALID_POPUP_PARENT: u32 = 3;
pub const XDG_WM_BASE_ERROR_INVALID_POSITIONER: u32 = 5;
pub const XDG_WM_BASE_ERROR_INVALID_SURFACE_STATE: u32 = 4;
pub const XDG_WM_BASE_ERROR_NOT_THE_TOPMOST_POPUP: u32 = 2;
pub const XDG_WM_BASE_ERROR_ROLE: u32 = 0;
pub const XDG_WM_BASE_GET_XDG_SURFACE: u32 = 2;
pub const XDG_WM_BASE_GET_XDG_SURFACE_SINCE_VERSION: u32 = 1;
pub const XDG_WM_BASE_PING_SINCE_VERSION: u32 = 1;
pub const XDG_WM_BASE_PONG: u32 = 3;
pub const XDG_WM_BASE_PONG_SINCE_VERSION: u32 = 1;

unsafe extern "C" {
    pub static xdg_popup_interface: wl_interface;
    pub static xdg_positioner_interface: wl_interface;
    pub static xdg_surface_interface: wl_interface;
    pub static xdg_toplevel_interface: wl_interface;
    pub static xdg_wm_base_interface: wl_interface;
}

#[inline]
pub unsafe extern "C" fn xdg_popup_add_listener(
    xdg_popup: *mut xdg_popup,
    listener: *const xdg_popup_listener,
    data: *mut c_void,
) -> c_int {
    unsafe {
        wl_proxy_add_listener(
            xdg_popup.cast(),
            listener.cast_mut().cast(),
            data,
        )
    }
}

#[inline]
pub unsafe extern "C" fn xdg_popup_destroy(
    xdg_popup: *mut xdg_popup,
) {
    unsafe {
        wl_proxy_marshal(
            xdg_popup.cast(),
            XDG_POPUP_DESTROY,
        );
        wl_proxy_destroy(
            xdg_popup.cast(),
        );
    }
}

#[inline]
pub unsafe extern "C" fn xdg_popup_get_user_data(
    xdg_popup: *mut xdg_popup,
) -> *mut c_void {
    unsafe { wl_proxy_get_user_data(xdg_popup.cast()) }
}

#[inline]
pub unsafe extern "C" fn xdg_popup_get_version(
    xdg_popup: *mut xdg_popup,
) -> u32 {
    unsafe { wl_proxy_get_version(xdg_popup.cast()) }
}

#[inline]
pub unsafe extern "C" fn xdg_popup_grab(
    xdg_popup: *mut xdg_popup,
    seat: *mut wl_seat,
    serial: u32,
) {
    unsafe {
        wl_proxy_marshal(
            xdg_popup.cast(),
            XDG_POPUP_GRAB,
            seat,
            serial,
        );
    }
}

#[inline]
pub unsafe extern "C" fn xdg_popup_reposition(
    xdg_popup: *mut xdg_popup,
    positioner: *mut xdg_positioner,
    token: u32,
) {
    unsafe {
        wl_proxy_marshal(
            xdg_popup.cast(),
            XDG_POPUP_REPOSITION,
            positioner,
            token,
        );
    }
}

#[inline]
pub unsafe extern "C" fn xdg_popup_set_user_data(
    xdg_popup: *mut xdg_popup,
    user_data: *mut c_void,
) {
    unsafe {
        wl_proxy_set_user_data(
            xdg_popup.cast(),
            user_data,
        )
    }
}

#[inline]
pub unsafe extern "C" fn xdg_positioner_destroy(
    xdg_positioner: *mut xdg_positioner,
) {
    unsafe {
        wl_proxy_marshal(
            xdg_positioner.cast(),
            XDG_POSITIONER_DESTROY,
        );
        wl_proxy_destroy(
            xdg_positioner.cast(),
        );
    }
}

#[inline]
pub unsafe extern "C" fn xdg_positioner_get_user_data(
    xdg_positioner: *mut xdg_positioner,
) -> *mut c_void {
    unsafe { wl_proxy_get_user_data(xdg_positioner.cast()) }
}

#[inline]
pub unsafe extern "C" fn xdg_positioner_get_version(
    xdg_positioner: *mut xdg_positioner,
) -> u32 {
    unsafe { wl_proxy_get_version(xdg_positioner.cast()) }
}

#[inline]
pub unsafe extern "C" fn xdg_positioner_set_anchor(
    xdg_positioner: *mut xdg_positioner,
    anchor: u32,
) {
    unsafe {
        wl_proxy_marshal(
            xdg_positioner.cast(),
            XDG_POSITIONER_SET_ANCHOR,
            anchor,
        );
    }
}

#[inline]
pub unsafe extern "C" fn xdg_positioner_set_anchor_rect(
    xdg_positioner: *mut xdg_positioner,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
) {
    unsafe {
        wl_proxy_marshal(
            xdg_positioner.cast(),
            XDG_POSITIONER_SET_ANCHOR_RECT,
            x,
            y,
            width,
            height,
        );
    }
}

#[inline]
pub unsafe extern "C" fn xdg_positioner_set_constraint_adjustment(
    xdg_positioner: *mut xdg_positioner,
    constraint_adjustment: u32,
) {
    unsafe {
        wl_proxy_marshal(
            xdg_positioner.cast(),
            XDG_POSITIONER_SET_CONSTRAINT_ADJUSTMENT,
            constraint_adjustment,
        );
    }
}

#[inline]
pub unsafe extern "C" fn xdg_positioner_set_gravity(
    xdg_positioner: *mut xdg_positioner,
    gravity: u32,
) {
    unsafe {
        wl_proxy_marshal(
            xdg_positioner.cast(),
            XDG_POSITIONER_SET_GRAVITY,
            gravity,
        );
    }
}

#[inline]
pub unsafe extern "C" fn xdg_positioner_set_offset(
    xdg_positioner: *mut xdg_positioner,
    x: i32,
    y: i32,
) {
    unsafe {
        wl_proxy_marshal(
            xdg_positioner.cast(),
            XDG_POSITIONER_SET_OFFSET,
            x,
            y,
        );
    }
}

#[inline]
pub unsafe extern "C" fn xdg_positioner_set_parent_configure(
    xdg_positioner: *mut xdg_positioner,
    serial: u32,
) {
    unsafe {
        wl_proxy_marshal(
            xdg_positioner.cast(),
            XDG_POSITIONER_SET_PARENT_CONFIGURE,
            serial,
        );
    }
}

#[inline]
pub unsafe extern "C" fn xdg_positioner_set_parent_size(
    xdg_positioner: *mut xdg_positioner,
    parent_width: i32,
    parent_height: i32,
) {
    unsafe {
        wl_proxy_marshal(
            xdg_positioner.cast(),
            XDG_POSITIONER_SET_PARENT_SIZE,
            parent_width,
            parent_height,
        );
    }
}

#[inline]
pub unsafe extern "C" fn xdg_positioner_set_reactive(
    xdg_positioner: *mut xdg_positioner,
) {
    unsafe {
        wl_proxy_marshal(
            xdg_positioner.cast(),
            XDG_POSITIONER_SET_REACTIVE,
        );
    }
}

#[inline]
pub unsafe extern "C" fn xdg_positioner_set_size(
    xdg_positioner: *mut xdg_positioner,
    width: i32,
    height: i32,
) {
    unsafe {
        wl_proxy_marshal(
            xdg_positioner.cast(),
            XDG_POSITIONER_SET_SIZE,
            width,
            height,
        );
    }
}

#[inline]
pub unsafe extern "C" fn xdg_positioner_set_user_data(
    xdg_positioner: *mut xdg_positioner,
    user_data: *mut c_void,
) {
    unsafe {
        wl_proxy_set_user_data(
            xdg_positioner.cast(),
            user_data,
        )
    }
}

#[inline]
pub unsafe extern "C" fn xdg_surface_ack_configure(
    xdg_surface: *mut xdg_surface,
    serial: u32,
) {
    unsafe {
        wl_proxy_marshal(
            xdg_surface.cast(),
            XDG_SURFACE_ACK_CONFIGURE,
            serial,
        );
    }
}

#[inline]
pub unsafe extern "C" fn xdg_surface_add_listener(
    xdg_surface: *mut xdg_surface,
    listener: *const xdg_surface_listener,
    data: *mut c_void,
) -> c_int {
    unsafe {
        wl_proxy_add_listener(
            xdg_surface.cast(),
            listener.cast_mut().cast(),
            data,
        )
    }
}

#[inline]
pub unsafe extern "C" fn xdg_surface_destroy(
    xdg_surface: *mut xdg_surface,
) {
    unsafe {
        wl_proxy_marshal(
            xdg_surface.cast(),
            XDG_SURFACE_DESTROY,
        );
        wl_proxy_destroy(
            xdg_surface.cast(),
        );
    }
}

#[inline]
pub unsafe extern "C" fn xdg_surface_get_popup(
    xdg_surface: *mut xdg_surface,
    parent: *mut xdg_surface,
    positioner: *mut xdg_positioner,
) -> *mut xdg_popup {
    unsafe {
        wl_proxy_marshal_constructor(
            xdg_surface.cast(),
            XDG_SURFACE_GET_POPUP,
            &xdg_popup_interface,
            null_mut::<c_void>(),
            parent,
            positioner,
        ).cast()
    }
}

#[inline]
pub unsafe extern "C" fn xdg_surface_get_toplevel(
    xdg_surface: *mut xdg_surface,
) -> *mut xdg_toplevel {
    unsafe {
        wl_proxy_marshal_constructor(
            xdg_surface.cast(),
            XDG_SURFACE_GET_TOPLEVEL,
            &xdg_toplevel_interface,
            null_mut::<c_void>(),
        ).cast()
    }
}

#[inline]
pub unsafe extern "C" fn xdg_surface_get_user_data(
    xdg_surface: *mut xdg_surface,
) -> *mut c_void {
    unsafe { wl_proxy_get_user_data(xdg_surface.cast()) }
}

#[inline]
pub unsafe extern "C" fn xdg_surface_get_version(
    xdg_surface: *mut xdg_surface,
) -> u32 {
    unsafe { wl_proxy_get_version(xdg_surface.cast()) }
}

#[inline]
pub unsafe extern "C" fn xdg_surface_set_user_data(
    xdg_surface: *mut xdg_surface,
    user_data: *mut c_void,
) {
    unsafe {
        wl_proxy_set_user_data(
            xdg_surface.cast(),
            user_data,
        )
    }
}

#[inline]
pub unsafe extern "C" fn xdg_surface_set_window_geometry(
    xdg_surface: *mut xdg_surface,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
) {
    unsafe {
        wl_proxy_marshal(
            xdg_surface.cast(),
            XDG_SURFACE_SET_WINDOW_GEOMETRY,
            x,
            y,
            width,
            height,
        );
    }
}

#[inline]
pub unsafe extern "C" fn xdg_toplevel_add_listener(
    xdg_toplevel: *mut xdg_toplevel,
    listener: *const xdg_toplevel_listener,
    data: *mut c_void,
) -> c_int {
    unsafe {
        wl_proxy_add_listener(
            xdg_toplevel.cast(),
            listener.cast_mut().cast(),
            data,
        )
    }
}

#[inline]
pub unsafe extern "C" fn xdg_toplevel_destroy(
    xdg_toplevel: *mut xdg_toplevel,
) {
    unsafe {
        wl_proxy_marshal(
            xdg_toplevel.cast(),
            XDG_TOPLEVEL_DESTROY,
        );
        wl_proxy_destroy(
            xdg_toplevel.cast(),
        );
    }
}

#[inline]
pub unsafe extern "C" fn xdg_toplevel_get_user_data(
    xdg_toplevel: *mut xdg_toplevel,
) -> *mut c_void {
    unsafe { wl_proxy_get_user_data(xdg_toplevel.cast()) }
}

#[inline]
pub unsafe extern "C" fn xdg_toplevel_get_version(
    xdg_toplevel: *mut xdg_toplevel,
) -> u32 {
    unsafe { wl_proxy_get_version(xdg_toplevel.cast()) }
}

#[inline]
pub unsafe extern "C" fn xdg_toplevel_move(
    xdg_toplevel: *mut xdg_toplevel,
    seat: *mut wl_seat,
    serial: u32,
) {
    unsafe {
        wl_proxy_marshal(
            xdg_toplevel.cast(),
            XDG_TOPLEVEL_MOVE,
            seat,
            serial,
        );
    }
}

#[inline]
pub unsafe extern "C" fn xdg_toplevel_resize(
    xdg_toplevel: *mut xdg_toplevel,
    seat: *mut wl_seat,
    serial: u32,
    edges: u32,
) {
    unsafe {
        wl_proxy_marshal(
            xdg_toplevel.cast(),
            XDG_TOPLEVEL_RESIZE,
            seat,
            serial,
            edges,
        );
    }
}

#[inline]
pub unsafe extern "C" fn xdg_toplevel_set_app_id(
    xdg_toplevel: *mut xdg_toplevel,
    app_id: *const c_char,
) {
    unsafe {
        wl_proxy_marshal(
            xdg_toplevel.cast(),
            XDG_TOPLEVEL_SET_APP_ID,
            app_id,
        );
    }
}

#[inline]
pub unsafe extern "C" fn xdg_toplevel_set_fullscreen(
    xdg_toplevel: *mut xdg_toplevel,
    output: *mut wl_output,
) {
    unsafe {
        wl_proxy_marshal(
            xdg_toplevel.cast(),
            XDG_TOPLEVEL_SET_FULLSCREEN,
            output,
        );
    }
}

#[inline]
pub unsafe extern "C" fn xdg_toplevel_set_max_size(
    xdg_toplevel: *mut xdg_toplevel,
    width: i32,
    height: i32,
) {
    unsafe {
        wl_proxy_marshal(
            xdg_toplevel.cast(),
            XDG_TOPLEVEL_SET_MAX_SIZE,
            width,
            height,
        );
    }
}

#[inline]
pub unsafe extern "C" fn xdg_toplevel_set_maximized(
    xdg_toplevel: *mut xdg_toplevel,
) {
    unsafe {
        wl_proxy_marshal(
            xdg_toplevel.cast(),
            XDG_TOPLEVEL_SET_MAXIMIZED,
        );
    }
}

#[inline]
pub unsafe extern "C" fn xdg_toplevel_set_min_size(
    xdg_toplevel: *mut xdg_toplevel,
    width: i32,
    height: i32,
) {
    unsafe {
        wl_proxy_marshal(
            xdg_toplevel.cast(),
            XDG_TOPLEVEL_SET_MIN_SIZE,
            width,
            height,
        );
    }
}

#[inline]
pub unsafe extern "C" fn xdg_toplevel_set_minimized(
    xdg_toplevel: *mut xdg_toplevel,
) {
    unsafe {
        wl_proxy_marshal(
            xdg_toplevel.cast(),
            XDG_TOPLEVEL_SET_MINIMIZED,
        );
    }
}

#[inline]
pub unsafe extern "C" fn xdg_toplevel_set_parent(
    xdg_toplevel: *mut xdg_toplevel,
    parent: *mut xdg_toplevel,
) {
    unsafe {
        wl_proxy_marshal(
            xdg_toplevel.cast(),
            XDG_TOPLEVEL_SET_PARENT,
            parent,
        );
    }
}

#[inline]
pub unsafe extern "C" fn xdg_toplevel_set_title(
    xdg_toplevel: *mut xdg_toplevel,
    title: *const c_char,
) {
    unsafe {
        wl_proxy_marshal(
            xdg_toplevel.cast(),
            XDG_TOPLEVEL_SET_TITLE,
            title,
        );
    }
}

#[inline]
pub unsafe extern "C" fn xdg_toplevel_set_user_data(
    xdg_toplevel: *mut xdg_toplevel,
    user_data: *mut c_void,
) {
    unsafe {
        wl_proxy_set_user_data(
            xdg_toplevel.cast(),
            user_data,
        )
    }
}

#[inline]
pub unsafe extern "C" fn xdg_toplevel_show_window_menu(
    xdg_toplevel: *mut xdg_toplevel,
    seat: *mut wl_seat,
    serial: u32,
    x: i32,
    y: i32,
) {
    unsafe {
        wl_proxy_marshal(
            xdg_toplevel.cast(),
            XDG_TOPLEVEL_SHOW_WINDOW_MENU,
            seat,
            serial,
            x,
            y,
        );
    }
}

#[inline]
pub unsafe extern "C" fn xdg_toplevel_unset_fullscreen(
    xdg_toplevel: *mut xdg_toplevel,
) {
    unsafe {
        wl_proxy_marshal(
            xdg_toplevel.cast(),
            XDG_TOPLEVEL_UNSET_FULLSCREEN,
        );
    }
}

#[inline]
pub unsafe extern "C" fn xdg_toplevel_unset_maximized(
    xdg_toplevel: *mut xdg_toplevel,
) {
    unsafe {
        wl_proxy_marshal(
            xdg_toplevel.cast(),
            XDG_TOPLEVEL_UNSET_MAXIMIZED,
        );
    }
}

#[inline]
pub unsafe extern "C" fn xdg_wm_base_add_listener(
    xdg_wm_base: *mut xdg_wm_base,
    listener: *const xdg_wm_base_listener,
    data: *mut c_void,
) -> c_int {
    unsafe {
        wl_proxy_add_listener(
            xdg_wm_base.cast(),
            listener.cast_mut().cast(),
            data,
        )
    }
}

#[inline]
pub unsafe extern "C" fn xdg_wm_base_create_positioner(
    xdg_wm_base: *mut xdg_wm_base,
) -> *mut xdg_positioner {
    unsafe {
        wl_proxy_marshal_constructor(
            xdg_wm_base.cast(),
            XDG_WM_BASE_CREATE_POSITIONER,
            &xdg_positioner_interface,
            null_mut::<c_void>(),
        ).cast()
    }
}

#[inline]
pub unsafe extern "C" fn xdg_wm_base_destroy(
    xdg_wm_base: *mut xdg_wm_base,
) {
    unsafe {
        wl_proxy_marshal(
            xdg_wm_base.cast(),
            XDG_WM_BASE_DESTROY,
        );
        wl_proxy_destroy(
            xdg_wm_base.cast(),
        );
    }
}

#[inline]
pub unsafe extern "C" fn xdg_wm_base_get_user_data(
    xdg_wm_base: *mut xdg_wm_base,
) -> *mut c_void {
    unsafe { wl_proxy_get_user_data(xdg_wm_base.cast()) }
}

#[inline]
pub unsafe extern "C" fn xdg_wm_base_get_version(
    xdg_wm_base: *mut xdg_wm_base,
) -> u32 {
    unsafe { wl_proxy_get_version(xdg_wm_base.cast()) }
}

#[inline]
pub unsafe extern "C" fn xdg_wm_base_get_xdg_surface(
    xdg_wm_base: *mut xdg_wm_base,
    surface: *mut wl_surface,
) -> *mut xdg_surface {
    unsafe {
        wl_proxy_marshal_constructor(
            xdg_wm_base.cast(),
            XDG_WM_BASE_GET_XDG_SURFACE,
            &xdg_surface_interface,
            null_mut::<c_void>(),
            surface,
        ).cast()
    }
}

#[inline]
pub unsafe extern "C" fn xdg_wm_base_pong(
    xdg_wm_base: *mut xdg_wm_base,
    serial: u32,
) {
    unsafe {
        wl_proxy_marshal(
            xdg_wm_base.cast(),
            XDG_WM_BASE_PONG,
            serial,
        );
    }
}

#[inline]
pub unsafe extern "C" fn xdg_wm_base_set_user_data(
    xdg_wm_base: *mut xdg_wm_base,
    user_data: *mut c_void,
) {
    unsafe {
        wl_proxy_set_user_data(
            xdg_wm_base.cast(),
            user_data,
        )
    }
}
