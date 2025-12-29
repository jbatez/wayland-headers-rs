use crate::prelude::*;

use super::wayland_server::*;

#[repr(C)]
pub struct xdg_popup {
    _data: (),
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct xdg_popup_interface {
    pub destroy: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
    )>,
    pub grab: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
        seat: *mut wl_resource,
        serial: u32,
    )>,
    pub reposition: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
        positioner: *mut wl_resource,
        token: u32,
    )>,
}

#[repr(C)]
pub struct xdg_positioner {
    _data: (),
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct xdg_positioner_interface {
    pub destroy: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
    )>,
    pub set_size: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
        width: i32,
        height: i32,
    )>,
    pub set_anchor_rect: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    )>,
    pub set_anchor: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
        anchor: u32,
    )>,
    pub set_gravity: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
        gravity: u32,
    )>,
    pub set_constraint_adjustment: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
        constraint_adjustment: u32,
    )>,
    pub set_offset: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
        x: i32,
        y: i32,
    )>,
    pub set_reactive: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
    )>,
    pub set_parent_size: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
        parent_width: i32,
        parent_height: i32,
    )>,
    pub set_parent_configure: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
        serial: u32,
    )>,
}

#[repr(C)]
pub struct xdg_surface {
    _data: (),
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct xdg_surface_interface {
    pub destroy: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
    )>,
    pub get_toplevel: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
        id: u32,
    )>,
    pub get_popup: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
        id: u32,
        parent: *mut wl_resource,
        positioner: *mut wl_resource,
    )>,
    pub set_window_geometry: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    )>,
    pub ack_configure: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
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
pub struct xdg_toplevel_interface {
    pub destroy: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
    )>,
    pub set_parent: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
        parent: *mut wl_resource,
    )>,
    pub set_title: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
        title: *const c_char,
    )>,
    pub set_app_id: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
        app_id: *const c_char,
    )>,
    pub show_window_menu: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
        seat: *mut wl_resource,
        serial: u32,
        x: i32,
        y: i32,
    )>,
    pub mov: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
        seat: *mut wl_resource,
        serial: u32,
    )>,
    pub resize: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
        seat: *mut wl_resource,
        serial: u32,
        edges: u32,
    )>,
    pub set_max_size: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
        width: i32,
        height: i32,
    )>,
    pub set_min_size: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
        width: i32,
        height: i32,
    )>,
    pub set_maximized: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
    )>,
    pub unset_maximized: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
    )>,
    pub set_fullscreen: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
        output: *mut wl_resource,
    )>,
    pub unset_fullscreen: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
    )>,
    pub set_minimized: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
    )>,
}

#[repr(C)]
pub struct xdg_wm_base {
    _data: (),
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct xdg_wm_base_interface {
    pub destroy: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
    )>,
    pub create_positioner: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
        id: u32,
    )>,
    pub get_xdg_surface: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
        id: u32,
        surface: *mut wl_resource,
    )>,
    pub pong: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
        serial: u32,
    )>,
}

pub const XDG_POPUP_CONFIGURE: u32 = 0;
pub const XDG_POPUP_CONFIGURE_SINCE_VERSION: u32 = 1;
pub const XDG_POPUP_DESTROY_SINCE_VERSION: u32 = 1;
pub const XDG_POPUP_ERROR_INVALID_GRAB: u32 = 0;
pub const XDG_POPUP_GRAB_SINCE_VERSION: u32 = 1;
pub const XDG_POPUP_POPUP_DONE: u32 = 1;
pub const XDG_POPUP_POPUP_DONE_SINCE_VERSION: u32 = 1;
pub const XDG_POPUP_REPOSITIONED: u32 = 2;
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
pub const XDG_POSITIONER_SET_ANCHOR_RECT_SINCE_VERSION: u32 = 1;
pub const XDG_POSITIONER_SET_ANCHOR_SINCE_VERSION: u32 = 1;
pub const XDG_POSITIONER_SET_CONSTRAINT_ADJUSTMENT_SINCE_VERSION: u32 = 1;
pub const XDG_POSITIONER_SET_GRAVITY_SINCE_VERSION: u32 = 1;
pub const XDG_POSITIONER_SET_OFFSET_SINCE_VERSION: u32 = 1;
pub const XDG_POSITIONER_SET_PARENT_CONFIGURE_SINCE_VERSION: u32 = 3;
pub const XDG_POSITIONER_SET_PARENT_SIZE_SINCE_VERSION: u32 = 3;
pub const XDG_POSITIONER_SET_REACTIVE_SINCE_VERSION: u32 = 3;
pub const XDG_POSITIONER_SET_SIZE_SINCE_VERSION: u32 = 1;
pub const XDG_SURFACE_ACK_CONFIGURE_SINCE_VERSION: u32 = 1;
pub const XDG_SURFACE_CONFIGURE: u32 = 0;
pub const XDG_SURFACE_CONFIGURE_SINCE_VERSION: u32 = 1;
pub const XDG_SURFACE_DESTROY_SINCE_VERSION: u32 = 1;
pub const XDG_SURFACE_ERROR_ALREADY_CONSTRUCTED: u32 = 2;
pub const XDG_SURFACE_ERROR_NOT_CONSTRUCTED: u32 = 1;
pub const XDG_SURFACE_ERROR_UNCONFIGURED_BUFFER: u32 = 3;
pub const XDG_SURFACE_GET_POPUP_SINCE_VERSION: u32 = 1;
pub const XDG_SURFACE_GET_TOPLEVEL_SINCE_VERSION: u32 = 1;
pub const XDG_SURFACE_SET_WINDOW_GEOMETRY_SINCE_VERSION: u32 = 1;
pub const XDG_TOPLEVEL_CLOSE: u32 = 1;
pub const XDG_TOPLEVEL_CLOSE_SINCE_VERSION: u32 = 1;
pub const XDG_TOPLEVEL_CONFIGURE: u32 = 0;
pub const XDG_TOPLEVEL_CONFIGURE_SINCE_VERSION: u32 = 1;
pub const XDG_TOPLEVEL_DESTROY_SINCE_VERSION: u32 = 1;
pub const XDG_TOPLEVEL_MOVE_SINCE_VERSION: u32 = 1;
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
pub const XDG_TOPLEVEL_SET_APP_ID_SINCE_VERSION: u32 = 1;
pub const XDG_TOPLEVEL_SET_FULLSCREEN_SINCE_VERSION: u32 = 1;
pub const XDG_TOPLEVEL_SET_MAXIMIZED_SINCE_VERSION: u32 = 1;
pub const XDG_TOPLEVEL_SET_MAX_SIZE_SINCE_VERSION: u32 = 1;
pub const XDG_TOPLEVEL_SET_MINIMIZED_SINCE_VERSION: u32 = 1;
pub const XDG_TOPLEVEL_SET_MIN_SIZE_SINCE_VERSION: u32 = 1;
pub const XDG_TOPLEVEL_SET_PARENT_SINCE_VERSION: u32 = 1;
pub const XDG_TOPLEVEL_SET_TITLE_SINCE_VERSION: u32 = 1;
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
pub const XDG_TOPLEVEL_UNSET_FULLSCREEN_SINCE_VERSION: u32 = 1;
pub const XDG_TOPLEVEL_UNSET_MAXIMIZED_SINCE_VERSION: u32 = 1;
pub const XDG_WM_BASE_CREATE_POSITIONER_SINCE_VERSION: u32 = 1;
pub const XDG_WM_BASE_DESTROY_SINCE_VERSION: u32 = 1;
pub const XDG_WM_BASE_ERROR_DEFUNCT_SURFACES: u32 = 1;
pub const XDG_WM_BASE_ERROR_INVALID_POPUP_PARENT: u32 = 3;
pub const XDG_WM_BASE_ERROR_INVALID_POSITIONER: u32 = 5;
pub const XDG_WM_BASE_ERROR_INVALID_SURFACE_STATE: u32 = 4;
pub const XDG_WM_BASE_ERROR_NOT_THE_TOPMOST_POPUP: u32 = 2;
pub const XDG_WM_BASE_ERROR_ROLE: u32 = 0;
pub const XDG_WM_BASE_GET_XDG_SURFACE_SINCE_VERSION: u32 = 1;
pub const XDG_WM_BASE_PING: u32 = 0;
pub const XDG_WM_BASE_PING_SINCE_VERSION: u32 = 1;
pub const XDG_WM_BASE_PONG_SINCE_VERSION: u32 = 1;

unsafe extern "C" {
    pub static xdg_popup_interface: wl_interface;
    pub static xdg_positioner_interface: wl_interface;
    pub static xdg_surface_interface: wl_interface;
    pub static xdg_toplevel_interface: wl_interface;
    pub static xdg_wm_base_interface: wl_interface;
}

#[inline]
pub unsafe extern "C" fn xdg_popup_send_configure(
    resource_: *mut wl_resource,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
) {
    unsafe {
        wl_resource_post_event(
            resource_,
            XDG_POPUP_CONFIGURE,
            x,
            y,
            width,
            height,
        )
    }
}

#[inline]
pub unsafe extern "C" fn xdg_popup_send_popup_done(
    resource_: *mut wl_resource,
) {
    unsafe {
        wl_resource_post_event(
            resource_,
            XDG_POPUP_POPUP_DONE,
        )
    }
}

#[inline]
pub unsafe extern "C" fn xdg_popup_send_repositioned(
    resource_: *mut wl_resource,
    token: u32,
) {
    unsafe {
        wl_resource_post_event(
            resource_,
            XDG_POPUP_REPOSITIONED,
            token,
        )
    }
}

#[inline]
pub unsafe extern "C" fn xdg_surface_send_configure(
    resource_: *mut wl_resource,
    serial: u32,
) {
    unsafe {
        wl_resource_post_event(
            resource_,
            XDG_SURFACE_CONFIGURE,
            serial,
        )
    }
}

#[inline]
pub unsafe extern "C" fn xdg_toplevel_send_close(
    resource_: *mut wl_resource,
) {
    unsafe {
        wl_resource_post_event(
            resource_,
            XDG_TOPLEVEL_CLOSE,
        )
    }
}

#[inline]
pub unsafe extern "C" fn xdg_toplevel_send_configure(
    resource_: *mut wl_resource,
    width: i32,
    height: i32,
    states: *mut wl_array,
) {
    unsafe {
        wl_resource_post_event(
            resource_,
            XDG_TOPLEVEL_CONFIGURE,
            width,
            height,
            states,
        )
    }
}

#[inline]
pub unsafe extern "C" fn xdg_wm_base_send_ping(
    resource_: *mut wl_resource,
    serial: u32,
) {
    unsafe {
        wl_resource_post_event(
            resource_,
            XDG_WM_BASE_PING,
            serial,
        )
    }
}
