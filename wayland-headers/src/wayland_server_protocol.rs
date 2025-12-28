use crate::prelude::*;

use super::wayland_server_core::*;

#[repr(C)]
pub struct wl_buffer {
    _data: (),
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct wl_buffer_interface {
    pub destroy: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
    )>,
}

#[repr(C)]
pub struct wl_callback {
    _data: (),
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[repr(C)]
pub struct wl_compositor {
    _data: (),
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct wl_compositor_interface {
    pub create_surface: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
        id: u32,
    )>,
    pub create_region: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
        id: u32,
    )>,
}

#[repr(C)]
pub struct wl_data_device {
    _data: (),
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct wl_data_device_interface {
    pub start_drag: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
        source: *mut wl_resource,
        origin: *mut wl_resource,
        icon: *mut wl_resource,
        serial: u32,
    )>,
    pub set_selection: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
        source: *mut wl_resource,
        serial: u32,
    )>,
    pub release: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
    )>,
}

#[repr(C)]
pub struct wl_data_device_manager {
    _data: (),
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct wl_data_device_manager_interface {
    pub create_data_source: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
        id: u32,
    )>,
    pub get_data_device: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
        id: u32,
        seat: *mut wl_resource,
    )>,
}

#[repr(C)]
pub struct wl_data_offer {
    _data: (),
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct wl_data_offer_interface {
    pub accept: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
        serial: u32,
        mime_type: *const c_char,
    )>,
    pub receive: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
        mime_type: *const c_char,
        fd: i32,
    )>,
    pub destroy: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
    )>,
    pub finish: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
    )>,
    pub set_actions: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
        dnd_actions: u32,
        preferred_action: u32,
    )>,
}

#[repr(C)]
pub struct wl_data_source {
    _data: (),
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct wl_data_source_interface {
    pub offer: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
        mime_type: *const c_char,
    )>,
    pub destroy: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
    )>,
    pub set_actions: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
        dnd_actions: u32,
    )>,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct wl_display_interface {
    pub sync: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
        callback: u32,
    )>,
    pub get_registry: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
        registry: u32,
    )>,
}

#[repr(C)]
pub struct wl_keyboard {
    _data: (),
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct wl_keyboard_interface {
    pub release: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
    )>,
}

#[repr(C)]
pub struct wl_output {
    _data: (),
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct wl_output_interface {
    pub release: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
    )>,
}

#[repr(C)]
pub struct wl_pointer {
    _data: (),
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct wl_pointer_interface {
    pub set_cursor: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
        serial: u32,
        surface: *mut wl_resource,
        hotspot_x: i32,
        hotspot_y: i32,
    )>,
    pub release: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
    )>,
}

#[repr(C)]
pub struct wl_region {
    _data: (),
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct wl_region_interface {
    pub destroy: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
    )>,
    pub add: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    )>,
    pub subtract: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    )>,
}

#[repr(C)]
pub struct wl_registry {
    _data: (),
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct wl_registry_interface {
    pub bind: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
        name: u32,
        interface: *const c_char,
        version: u32,
        id: u32,
    )>,
}

#[repr(C)]
pub struct wl_seat {
    _data: (),
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct wl_seat_interface {
    pub get_pointer: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
        id: u32,
    )>,
    pub get_keyboard: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
        id: u32,
    )>,
    pub get_touch: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
        id: u32,
    )>,
    pub release: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
    )>,
}

#[repr(C)]
pub struct wl_shell {
    _data: (),
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct wl_shell_interface {
    pub get_shell_surface: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
        id: u32,
        surface: *mut wl_resource,
    )>,
}

#[repr(C)]
pub struct wl_shell_surface {
    _data: (),
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct wl_shell_surface_interface {
    pub pong: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
        serial: u32,
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
    pub set_toplevel: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
    )>,
    pub set_transient: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
        parent: *mut wl_resource,
        x: i32,
        y: i32,
        flags: u32,
    )>,
    pub set_fullscreen: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
        method: u32,
        framerate: u32,
        output: *mut wl_resource,
    )>,
    pub set_popup: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
        seat: *mut wl_resource,
        serial: u32,
        parent: *mut wl_resource,
        x: i32,
        y: i32,
        flags: u32,
    )>,
    pub set_maximized: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
        output: *mut wl_resource,
    )>,
    pub set_title: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
        title: *const c_char,
    )>,
    pub set_class: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
        class_: *const c_char,
    )>,
}

#[repr(C)]
pub struct wl_shm {
    _data: (),
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct wl_shm_interface {
    pub create_pool: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
        id: u32,
        fd: i32,
        size: i32,
    )>,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct wl_shm_pool_interface {
    pub create_buffer: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
        id: u32,
        offset: i32,
        width: i32,
        height: i32,
        stride: i32,
        format: u32,
    )>,
    pub destroy: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
    )>,
    pub resize: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
        size: i32,
    )>,
}

#[repr(C)]
pub struct wl_subcompositor {
    _data: (),
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct wl_subcompositor_interface {
    pub destroy: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
    )>,
    pub get_subsurface: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
        id: u32,
        surface: *mut wl_resource,
        parent: *mut wl_resource,
    )>,
}

#[repr(C)]
pub struct wl_subsurface {
    _data: (),
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct wl_subsurface_interface {
    pub destroy: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
    )>,
    pub set_position: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
        x: i32,
        y: i32,
    )>,
    pub place_above: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
        sibling: *mut wl_resource,
    )>,
    pub place_below: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
        sibling: *mut wl_resource,
    )>,
    pub set_sync: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
    )>,
    pub set_desync: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
    )>,
}

#[repr(C)]
pub struct wl_surface {
    _data: (),
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct wl_surface_interface {
    pub destroy: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
    )>,
    pub attach: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
        buffer: *mut wl_resource,
        x: i32,
        y: i32,
    )>,
    pub damage: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    )>,
    pub frame: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
        callback: u32,
    )>,
    pub set_opaque_region: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
        region: *mut wl_resource,
    )>,
    pub set_input_region: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
        region: *mut wl_resource,
    )>,
    pub commit: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
    )>,
    pub set_buffer_transform: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
        transform: i32,
    )>,
    pub set_buffer_scale: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
        scale: i32,
    )>,
    pub damage_buffer: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    )>,
}

#[repr(C)]
pub struct wl_touch {
    _data: (),
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct wl_touch_interface {
    pub release: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
    )>,
}

pub const WL_BUFFER_DESTROY_SINCE_VERSION: u32 = 1;
pub const WL_BUFFER_RELEASE: u32 = 0;
pub const WL_BUFFER_RELEASE_SINCE_VERSION: u32 = 1;
pub const WL_CALLBACK_DONE: u32 = 0;
pub const WL_CALLBACK_DONE_SINCE_VERSION: u32 = 1;
pub const WL_COMPOSITOR_CREATE_REGION_SINCE_VERSION: u32 = 1;
pub const WL_COMPOSITOR_CREATE_SURFACE_SINCE_VERSION: u32 = 1;
pub const WL_DATA_DEVICE_DATA_OFFER: u32 = 0;
pub const WL_DATA_DEVICE_DATA_OFFER_SINCE_VERSION: u32 = 1;
pub const WL_DATA_DEVICE_DROP: u32 = 4;
pub const WL_DATA_DEVICE_DROP_SINCE_VERSION: u32 = 1;
pub const WL_DATA_DEVICE_ENTER: u32 = 1;
pub const WL_DATA_DEVICE_ENTER_SINCE_VERSION: u32 = 1;
pub const WL_DATA_DEVICE_ERROR_ROLE: u32 = 0;
pub const WL_DATA_DEVICE_LEAVE: u32 = 2;
pub const WL_DATA_DEVICE_LEAVE_SINCE_VERSION: u32 = 1;
pub const WL_DATA_DEVICE_MANAGER_CREATE_DATA_SOURCE_SINCE_VERSION: u32 = 1;
pub const WL_DATA_DEVICE_MANAGER_DND_ACTION_ASK: u32 = 4;
pub const WL_DATA_DEVICE_MANAGER_DND_ACTION_COPY: u32 = 1;
pub const WL_DATA_DEVICE_MANAGER_DND_ACTION_MOVE: u32 = 2;
pub const WL_DATA_DEVICE_MANAGER_DND_ACTION_NONE: u32 = 0;
pub const WL_DATA_DEVICE_MANAGER_GET_DATA_DEVICE_SINCE_VERSION: u32 = 1;
pub const WL_DATA_DEVICE_MOTION: u32 = 3;
pub const WL_DATA_DEVICE_MOTION_SINCE_VERSION: u32 = 1;
pub const WL_DATA_DEVICE_RELEASE_SINCE_VERSION: u32 = 2;
pub const WL_DATA_DEVICE_SELECTION: u32 = 5;
pub const WL_DATA_DEVICE_SELECTION_SINCE_VERSION: u32 = 1;
pub const WL_DATA_DEVICE_SET_SELECTION_SINCE_VERSION: u32 = 1;
pub const WL_DATA_DEVICE_START_DRAG_SINCE_VERSION: u32 = 1;
pub const WL_DATA_OFFER_ACCEPT_SINCE_VERSION: u32 = 1;
pub const WL_DATA_OFFER_ACTION: u32 = 2;
pub const WL_DATA_OFFER_ACTION_SINCE_VERSION: u32 = 3;
pub const WL_DATA_OFFER_DESTROY_SINCE_VERSION: u32 = 1;
pub const WL_DATA_OFFER_ERROR_INVALID_ACTION: u32 = 2;
pub const WL_DATA_OFFER_ERROR_INVALID_ACTION_MASK: u32 = 1;
pub const WL_DATA_OFFER_ERROR_INVALID_FINISH: u32 = 0;
pub const WL_DATA_OFFER_ERROR_INVALID_OFFER: u32 = 3;
pub const WL_DATA_OFFER_FINISH_SINCE_VERSION: u32 = 3;
pub const WL_DATA_OFFER_OFFER: u32 = 0;
pub const WL_DATA_OFFER_OFFER_SINCE_VERSION: u32 = 1;
pub const WL_DATA_OFFER_RECEIVE_SINCE_VERSION: u32 = 1;
pub const WL_DATA_OFFER_SET_ACTIONS_SINCE_VERSION: u32 = 3;
pub const WL_DATA_OFFER_SOURCE_ACTIONS: u32 = 1;
pub const WL_DATA_OFFER_SOURCE_ACTIONS_SINCE_VERSION: u32 = 3;
pub const WL_DATA_SOURCE_ACTION: u32 = 5;
pub const WL_DATA_SOURCE_ACTION_SINCE_VERSION: u32 = 3;
pub const WL_DATA_SOURCE_CANCELLED: u32 = 2;
pub const WL_DATA_SOURCE_CANCELLED_SINCE_VERSION: u32 = 1;
pub const WL_DATA_SOURCE_DESTROY_SINCE_VERSION: u32 = 1;
pub const WL_DATA_SOURCE_DND_DROP_PERFORMED: u32 = 3;
pub const WL_DATA_SOURCE_DND_DROP_PERFORMED_SINCE_VERSION: u32 = 3;
pub const WL_DATA_SOURCE_DND_FINISHED: u32 = 4;
pub const WL_DATA_SOURCE_DND_FINISHED_SINCE_VERSION: u32 = 3;
pub const WL_DATA_SOURCE_ERROR_INVALID_ACTION_MASK: u32 = 0;
pub const WL_DATA_SOURCE_ERROR_INVALID_SOURCE: u32 = 1;
pub const WL_DATA_SOURCE_OFFER_SINCE_VERSION: u32 = 1;
pub const WL_DATA_SOURCE_SEND: u32 = 1;
pub const WL_DATA_SOURCE_SEND_SINCE_VERSION: u32 = 1;
pub const WL_DATA_SOURCE_SET_ACTIONS_SINCE_VERSION: u32 = 3;
pub const WL_DATA_SOURCE_TARGET: u32 = 0;
pub const WL_DATA_SOURCE_TARGET_SINCE_VERSION: u32 = 1;
pub const WL_DISPLAY_DELETE_ID: u32 = 1;
pub const WL_DISPLAY_DELETE_ID_SINCE_VERSION: u32 = 1;
pub const WL_DISPLAY_ERROR: u32 = 0;
pub const WL_DISPLAY_ERROR_IMPLEMENTATION: u32 = 3;
pub const WL_DISPLAY_ERROR_INVALID_METHOD: u32 = 1;
pub const WL_DISPLAY_ERROR_INVALID_OBJECT: u32 = 0;
pub const WL_DISPLAY_ERROR_NO_MEMORY: u32 = 2;
pub const WL_DISPLAY_ERROR_SINCE_VERSION: u32 = 1;
pub const WL_DISPLAY_GET_REGISTRY_SINCE_VERSION: u32 = 1;
pub const WL_DISPLAY_SYNC_SINCE_VERSION: u32 = 1;
pub const WL_KEYBOARD_ENTER: u32 = 1;
pub const WL_KEYBOARD_ENTER_SINCE_VERSION: u32 = 1;
pub const WL_KEYBOARD_KEY: u32 = 3;
pub const WL_KEYBOARD_KEYMAP: u32 = 0;
pub const WL_KEYBOARD_KEYMAP_FORMAT_NO_KEYMAP: u32 = 0;
pub const WL_KEYBOARD_KEYMAP_FORMAT_XKB_V1: u32 = 1;
pub const WL_KEYBOARD_KEYMAP_SINCE_VERSION: u32 = 1;
pub const WL_KEYBOARD_KEY_SINCE_VERSION: u32 = 1;
pub const WL_KEYBOARD_KEY_STATE_PRESSED: u32 = 1;
pub const WL_KEYBOARD_KEY_STATE_RELEASED: u32 = 0;
pub const WL_KEYBOARD_LEAVE: u32 = 2;
pub const WL_KEYBOARD_LEAVE_SINCE_VERSION: u32 = 1;
pub const WL_KEYBOARD_MODIFIERS: u32 = 4;
pub const WL_KEYBOARD_MODIFIERS_SINCE_VERSION: u32 = 1;
pub const WL_KEYBOARD_RELEASE_SINCE_VERSION: u32 = 3;
pub const WL_KEYBOARD_REPEAT_INFO: u32 = 5;
pub const WL_KEYBOARD_REPEAT_INFO_SINCE_VERSION: u32 = 4;
pub const WL_OUTPUT_DONE: u32 = 2;
pub const WL_OUTPUT_DONE_SINCE_VERSION: u32 = 2;
pub const WL_OUTPUT_GEOMETRY: u32 = 0;
pub const WL_OUTPUT_GEOMETRY_SINCE_VERSION: u32 = 1;
pub const WL_OUTPUT_MODE: u32 = 1;
pub const WL_OUTPUT_MODE_CURRENT: u32 = 0x1;
pub const WL_OUTPUT_MODE_PREFERRED: u32 = 0x2;
pub const WL_OUTPUT_MODE_SINCE_VERSION: u32 = 1;
pub const WL_OUTPUT_RELEASE_SINCE_VERSION: u32 = 3;
pub const WL_OUTPUT_SCALE: u32 = 3;
pub const WL_OUTPUT_SCALE_SINCE_VERSION: u32 = 2;
pub const WL_OUTPUT_SUBPIXEL_HORIZONTAL_BGR: u32 = 3;
pub const WL_OUTPUT_SUBPIXEL_HORIZONTAL_RGB: u32 = 2;
pub const WL_OUTPUT_SUBPIXEL_NONE: u32 = 1;
pub const WL_OUTPUT_SUBPIXEL_UNKNOWN: u32 = 0;
pub const WL_OUTPUT_SUBPIXEL_VERTICAL_BGR: u32 = 5;
pub const WL_OUTPUT_SUBPIXEL_VERTICAL_RGB: u32 = 4;
pub const WL_OUTPUT_TRANSFORM_180: u32 = 2;
pub const WL_OUTPUT_TRANSFORM_270: u32 = 3;
pub const WL_OUTPUT_TRANSFORM_90: u32 = 1;
pub const WL_OUTPUT_TRANSFORM_FLIPPED: u32 = 4;
pub const WL_OUTPUT_TRANSFORM_FLIPPED_180: u32 = 6;
pub const WL_OUTPUT_TRANSFORM_FLIPPED_270: u32 = 7;
pub const WL_OUTPUT_TRANSFORM_FLIPPED_90: u32 = 5;
pub const WL_OUTPUT_TRANSFORM_NORMAL: u32 = 0;
pub const WL_POINTER_AXIS: u32 = 4;
pub const WL_POINTER_AXIS_DISCRETE: u32 = 8;
pub const WL_POINTER_AXIS_DISCRETE_SINCE_VERSION: u32 = 5;
pub const WL_POINTER_AXIS_HORIZONTAL_SCROLL: u32 = 1;
pub const WL_POINTER_AXIS_SINCE_VERSION: u32 = 1;
pub const WL_POINTER_AXIS_SOURCE: u32 = 6;
pub const WL_POINTER_AXIS_SOURCE_CONTINUOUS: u32 = 2;
pub const WL_POINTER_AXIS_SOURCE_FINGER: u32 = 1;
pub const WL_POINTER_AXIS_SOURCE_SINCE_VERSION: u32 = 5;
pub const WL_POINTER_AXIS_SOURCE_WHEEL: u32 = 0;
pub const WL_POINTER_AXIS_SOURCE_WHEEL_TILT: u32 = 3;
pub const WL_POINTER_AXIS_SOURCE_WHEEL_TILT_SINCE_VERSION: u32 = 6;
pub const WL_POINTER_AXIS_STOP: u32 = 7;
pub const WL_POINTER_AXIS_STOP_SINCE_VERSION: u32 = 5;
pub const WL_POINTER_AXIS_VERTICAL_SCROLL: u32 = 0;
pub const WL_POINTER_BUTTON: u32 = 3;
pub const WL_POINTER_BUTTON_SINCE_VERSION: u32 = 1;
pub const WL_POINTER_BUTTON_STATE_PRESSED: u32 = 1;
pub const WL_POINTER_BUTTON_STATE_RELEASED: u32 = 0;
pub const WL_POINTER_ENTER: u32 = 0;
pub const WL_POINTER_ENTER_SINCE_VERSION: u32 = 1;
pub const WL_POINTER_ERROR_ROLE: u32 = 0;
pub const WL_POINTER_FRAME: u32 = 5;
pub const WL_POINTER_FRAME_SINCE_VERSION: u32 = 5;
pub const WL_POINTER_LEAVE: u32 = 1;
pub const WL_POINTER_LEAVE_SINCE_VERSION: u32 = 1;
pub const WL_POINTER_MOTION: u32 = 2;
pub const WL_POINTER_MOTION_SINCE_VERSION: u32 = 1;
pub const WL_POINTER_RELEASE_SINCE_VERSION: u32 = 3;
pub const WL_POINTER_SET_CURSOR_SINCE_VERSION: u32 = 1;
pub const WL_REGION_ADD_SINCE_VERSION: u32 = 1;
pub const WL_REGION_DESTROY_SINCE_VERSION: u32 = 1;
pub const WL_REGION_SUBTRACT_SINCE_VERSION: u32 = 1;
pub const WL_REGISTRY_BIND_SINCE_VERSION: u32 = 1;
pub const WL_REGISTRY_GLOBAL: u32 = 0;
pub const WL_REGISTRY_GLOBAL_REMOVE: u32 = 1;
pub const WL_REGISTRY_GLOBAL_REMOVE_SINCE_VERSION: u32 = 1;
pub const WL_REGISTRY_GLOBAL_SINCE_VERSION: u32 = 1;
pub const WL_SEAT_CAPABILITIES: u32 = 0;
pub const WL_SEAT_CAPABILITIES_SINCE_VERSION: u32 = 1;
pub const WL_SEAT_CAPABILITY_KEYBOARD: u32 = 2;
pub const WL_SEAT_CAPABILITY_POINTER: u32 = 1;
pub const WL_SEAT_CAPABILITY_TOUCH: u32 = 4;
pub const WL_SEAT_GET_KEYBOARD_SINCE_VERSION: u32 = 1;
pub const WL_SEAT_GET_POINTER_SINCE_VERSION: u32 = 1;
pub const WL_SEAT_GET_TOUCH_SINCE_VERSION: u32 = 1;
pub const WL_SEAT_NAME: u32 = 1;
pub const WL_SEAT_NAME_SINCE_VERSION: u32 = 2;
pub const WL_SEAT_RELEASE_SINCE_VERSION: u32 = 5;
pub const WL_SHELL_ERROR_ROLE: u32 = 0;
pub const WL_SHELL_GET_SHELL_SURFACE_SINCE_VERSION: u32 = 1;
pub const WL_SHELL_SURFACE_CONFIGURE: u32 = 1;
pub const WL_SHELL_SURFACE_CONFIGURE_SINCE_VERSION: u32 = 1;
pub const WL_SHELL_SURFACE_FULLSCREEN_METHOD_DEFAULT: u32 = 0;
pub const WL_SHELL_SURFACE_FULLSCREEN_METHOD_DRIVER: u32 = 2;
pub const WL_SHELL_SURFACE_FULLSCREEN_METHOD_FILL: u32 = 3;
pub const WL_SHELL_SURFACE_FULLSCREEN_METHOD_SCALE: u32 = 1;
pub const WL_SHELL_SURFACE_MOVE_SINCE_VERSION: u32 = 1;
pub const WL_SHELL_SURFACE_PING: u32 = 0;
pub const WL_SHELL_SURFACE_PING_SINCE_VERSION: u32 = 1;
pub const WL_SHELL_SURFACE_PONG_SINCE_VERSION: u32 = 1;
pub const WL_SHELL_SURFACE_POPUP_DONE: u32 = 2;
pub const WL_SHELL_SURFACE_POPUP_DONE_SINCE_VERSION: u32 = 1;
pub const WL_SHELL_SURFACE_RESIZE_BOTTOM: u32 = 2;
pub const WL_SHELL_SURFACE_RESIZE_BOTTOM_LEFT: u32 = 6;
pub const WL_SHELL_SURFACE_RESIZE_BOTTOM_RIGHT: u32 = 10;
pub const WL_SHELL_SURFACE_RESIZE_LEFT: u32 = 4;
pub const WL_SHELL_SURFACE_RESIZE_NONE: u32 = 0;
pub const WL_SHELL_SURFACE_RESIZE_RIGHT: u32 = 8;
pub const WL_SHELL_SURFACE_RESIZE_SINCE_VERSION: u32 = 1;
pub const WL_SHELL_SURFACE_RESIZE_TOP: u32 = 1;
pub const WL_SHELL_SURFACE_RESIZE_TOP_LEFT: u32 = 5;
pub const WL_SHELL_SURFACE_RESIZE_TOP_RIGHT: u32 = 9;
pub const WL_SHELL_SURFACE_SET_CLASS_SINCE_VERSION: u32 = 1;
pub const WL_SHELL_SURFACE_SET_FULLSCREEN_SINCE_VERSION: u32 = 1;
pub const WL_SHELL_SURFACE_SET_MAXIMIZED_SINCE_VERSION: u32 = 1;
pub const WL_SHELL_SURFACE_SET_POPUP_SINCE_VERSION: u32 = 1;
pub const WL_SHELL_SURFACE_SET_TITLE_SINCE_VERSION: u32 = 1;
pub const WL_SHELL_SURFACE_SET_TOPLEVEL_SINCE_VERSION: u32 = 1;
pub const WL_SHELL_SURFACE_SET_TRANSIENT_SINCE_VERSION: u32 = 1;
pub const WL_SHELL_SURFACE_TRANSIENT_INACTIVE: u32 = 0x1;
pub const WL_SHM_CREATE_POOL_SINCE_VERSION: u32 = 1;
pub const WL_SHM_ERROR_INVALID_FD: u32 = 2;
pub const WL_SHM_ERROR_INVALID_FORMAT: u32 = 0;
pub const WL_SHM_ERROR_INVALID_STRIDE: u32 = 1;
pub const WL_SHM_FORMAT: u32 = 0;
pub const WL_SHM_FORMAT_ABGR1555: u32 = 0x35314241;
pub const WL_SHM_FORMAT_ABGR16161616F: u32 = 0x48344241;
pub const WL_SHM_FORMAT_ABGR2101010: u32 = 0x30334241;
pub const WL_SHM_FORMAT_ABGR4444: u32 = 0x32314241;
pub const WL_SHM_FORMAT_ABGR8888: u32 = 0x34324241;
pub const WL_SHM_FORMAT_ARGB1555: u32 = 0x35315241;
pub const WL_SHM_FORMAT_ARGB16161616F: u32 = 0x48345241;
pub const WL_SHM_FORMAT_ARGB2101010: u32 = 0x30335241;
pub const WL_SHM_FORMAT_ARGB4444: u32 = 0x32315241;
pub const WL_SHM_FORMAT_ARGB8888: u32 = 0;
pub const WL_SHM_FORMAT_AYUV: u32 = 0x56555941;
pub const WL_SHM_FORMAT_BGR233: u32 = 0x38524742;
pub const WL_SHM_FORMAT_BGR565: u32 = 0x36314742;
pub const WL_SHM_FORMAT_BGR565_A8: u32 = 0x38413542;
pub const WL_SHM_FORMAT_BGR888: u32 = 0x34324742;
pub const WL_SHM_FORMAT_BGR888_A8: u32 = 0x38413842;
pub const WL_SHM_FORMAT_BGRA1010102: u32 = 0x30334142;
pub const WL_SHM_FORMAT_BGRA4444: u32 = 0x32314142;
pub const WL_SHM_FORMAT_BGRA5551: u32 = 0x35314142;
pub const WL_SHM_FORMAT_BGRA8888: u32 = 0x34324142;
pub const WL_SHM_FORMAT_BGRX1010102: u32 = 0x30335842;
pub const WL_SHM_FORMAT_BGRX4444: u32 = 0x32315842;
pub const WL_SHM_FORMAT_BGRX5551: u32 = 0x35315842;
pub const WL_SHM_FORMAT_BGRX8888: u32 = 0x34325842;
pub const WL_SHM_FORMAT_BGRX8888_A8: u32 = 0x38415842;
pub const WL_SHM_FORMAT_C8: u32 = 0x20203843;
pub const WL_SHM_FORMAT_GR1616: u32 = 0x32335247;
pub const WL_SHM_FORMAT_GR88: u32 = 0x38385247;
pub const WL_SHM_FORMAT_NV12: u32 = 0x3231564e;
pub const WL_SHM_FORMAT_NV16: u32 = 0x3631564e;
pub const WL_SHM_FORMAT_NV21: u32 = 0x3132564e;
pub const WL_SHM_FORMAT_NV24: u32 = 0x3432564e;
pub const WL_SHM_FORMAT_NV42: u32 = 0x3234564e;
pub const WL_SHM_FORMAT_NV61: u32 = 0x3136564e;
pub const WL_SHM_FORMAT_P010: u32 = 0x30313050;
pub const WL_SHM_FORMAT_P012: u32 = 0x32313050;
pub const WL_SHM_FORMAT_P016: u32 = 0x36313050;
pub const WL_SHM_FORMAT_P210: u32 = 0x30313250;
pub const WL_SHM_FORMAT_R16: u32 = 0x20363152;
pub const WL_SHM_FORMAT_R8: u32 = 0x20203852;
pub const WL_SHM_FORMAT_RG1616: u32 = 0x32334752;
pub const WL_SHM_FORMAT_RG88: u32 = 0x38384752;
pub const WL_SHM_FORMAT_RGB332: u32 = 0x38424752;
pub const WL_SHM_FORMAT_RGB565: u32 = 0x36314752;
pub const WL_SHM_FORMAT_RGB565_A8: u32 = 0x38413552;
pub const WL_SHM_FORMAT_RGB888: u32 = 0x34324752;
pub const WL_SHM_FORMAT_RGB888_A8: u32 = 0x38413852;
pub const WL_SHM_FORMAT_RGBA1010102: u32 = 0x30334152;
pub const WL_SHM_FORMAT_RGBA4444: u32 = 0x32314152;
pub const WL_SHM_FORMAT_RGBA5551: u32 = 0x35314152;
pub const WL_SHM_FORMAT_RGBA8888: u32 = 0x34324152;
pub const WL_SHM_FORMAT_RGBX1010102: u32 = 0x30335852;
pub const WL_SHM_FORMAT_RGBX4444: u32 = 0x32315852;
pub const WL_SHM_FORMAT_RGBX5551: u32 = 0x35315852;
pub const WL_SHM_FORMAT_RGBX8888: u32 = 0x34325852;
pub const WL_SHM_FORMAT_RGBX8888_A8: u32 = 0x38415852;
pub const WL_SHM_FORMAT_SINCE_VERSION: u32 = 1;
pub const WL_SHM_FORMAT_UYVY: u32 = 0x59565955;
pub const WL_SHM_FORMAT_VUY101010: u32 = 0x30335556;
pub const WL_SHM_FORMAT_VUY888: u32 = 0x34325556;
pub const WL_SHM_FORMAT_VYUY: u32 = 0x59555956;
pub const WL_SHM_FORMAT_X0L0: u32 = 0x304c3058;
pub const WL_SHM_FORMAT_X0L2: u32 = 0x324c3058;
pub const WL_SHM_FORMAT_XBGR1555: u32 = 0x35314258;
pub const WL_SHM_FORMAT_XBGR16161616F: u32 = 0x48344258;
pub const WL_SHM_FORMAT_XBGR2101010: u32 = 0x30334258;
pub const WL_SHM_FORMAT_XBGR4444: u32 = 0x32314258;
pub const WL_SHM_FORMAT_XBGR8888: u32 = 0x34324258;
pub const WL_SHM_FORMAT_XBGR8888_A8: u32 = 0x38414258;
pub const WL_SHM_FORMAT_XRGB1555: u32 = 0x35315258;
pub const WL_SHM_FORMAT_XRGB16161616F: u32 = 0x48345258;
pub const WL_SHM_FORMAT_XRGB2101010: u32 = 0x30335258;
pub const WL_SHM_FORMAT_XRGB4444: u32 = 0x32315258;
pub const WL_SHM_FORMAT_XRGB8888: u32 = 1;
pub const WL_SHM_FORMAT_XRGB8888_A8: u32 = 0x38415258;
pub const WL_SHM_FORMAT_XVYU12_16161616: u32 = 0x36335658;
pub const WL_SHM_FORMAT_XVYU16161616: u32 = 0x38345658;
pub const WL_SHM_FORMAT_XVYU2101010: u32 = 0x30335658;
pub const WL_SHM_FORMAT_XYUV8888: u32 = 0x56555958;
pub const WL_SHM_FORMAT_Y0L0: u32 = 0x304c3059;
pub const WL_SHM_FORMAT_Y0L2: u32 = 0x324c3059;
pub const WL_SHM_FORMAT_Y210: u32 = 0x30313259;
pub const WL_SHM_FORMAT_Y212: u32 = 0x32313259;
pub const WL_SHM_FORMAT_Y216: u32 = 0x36313259;
pub const WL_SHM_FORMAT_Y410: u32 = 0x30313459;
pub const WL_SHM_FORMAT_Y412: u32 = 0x32313459;
pub const WL_SHM_FORMAT_Y416: u32 = 0x36313459;
pub const WL_SHM_FORMAT_YUV410: u32 = 0x39565559;
pub const WL_SHM_FORMAT_YUV411: u32 = 0x31315559;
pub const WL_SHM_FORMAT_YUV420: u32 = 0x32315559;
pub const WL_SHM_FORMAT_YUV420_10BIT: u32 = 0x30315559;
pub const WL_SHM_FORMAT_YUV420_8BIT: u32 = 0x38305559;
pub const WL_SHM_FORMAT_YUV422: u32 = 0x36315559;
pub const WL_SHM_FORMAT_YUV444: u32 = 0x34325559;
pub const WL_SHM_FORMAT_YUYV: u32 = 0x56595559;
pub const WL_SHM_FORMAT_YVU410: u32 = 0x39555659;
pub const WL_SHM_FORMAT_YVU411: u32 = 0x31315659;
pub const WL_SHM_FORMAT_YVU420: u32 = 0x32315659;
pub const WL_SHM_FORMAT_YVU422: u32 = 0x36315659;
pub const WL_SHM_FORMAT_YVU444: u32 = 0x34325659;
pub const WL_SHM_FORMAT_YVYU: u32 = 0x55595659;
pub const WL_SHM_POOL_CREATE_BUFFER_SINCE_VERSION: u32 = 1;
pub const WL_SHM_POOL_DESTROY_SINCE_VERSION: u32 = 1;
pub const WL_SHM_POOL_RESIZE_SINCE_VERSION: u32 = 1;
pub const WL_SUBCOMPOSITOR_DESTROY_SINCE_VERSION: u32 = 1;
pub const WL_SUBCOMPOSITOR_ERROR_BAD_SURFACE: u32 = 0;
pub const WL_SUBCOMPOSITOR_GET_SUBSURFACE_SINCE_VERSION: u32 = 1;
pub const WL_SUBSURFACE_DESTROY_SINCE_VERSION: u32 = 1;
pub const WL_SUBSURFACE_ERROR_BAD_SURFACE: u32 = 0;
pub const WL_SUBSURFACE_PLACE_ABOVE_SINCE_VERSION: u32 = 1;
pub const WL_SUBSURFACE_PLACE_BELOW_SINCE_VERSION: u32 = 1;
pub const WL_SUBSURFACE_SET_DESYNC_SINCE_VERSION: u32 = 1;
pub const WL_SUBSURFACE_SET_POSITION_SINCE_VERSION: u32 = 1;
pub const WL_SUBSURFACE_SET_SYNC_SINCE_VERSION: u32 = 1;
pub const WL_SURFACE_ATTACH_SINCE_VERSION: u32 = 1;
pub const WL_SURFACE_COMMIT_SINCE_VERSION: u32 = 1;
pub const WL_SURFACE_DAMAGE_BUFFER_SINCE_VERSION: u32 = 4;
pub const WL_SURFACE_DAMAGE_SINCE_VERSION: u32 = 1;
pub const WL_SURFACE_DESTROY_SINCE_VERSION: u32 = 1;
pub const WL_SURFACE_ENTER: u32 = 0;
pub const WL_SURFACE_ENTER_SINCE_VERSION: u32 = 1;
pub const WL_SURFACE_ERROR_INVALID_SCALE: u32 = 0;
pub const WL_SURFACE_ERROR_INVALID_TRANSFORM: u32 = 1;
pub const WL_SURFACE_FRAME_SINCE_VERSION: u32 = 1;
pub const WL_SURFACE_LEAVE: u32 = 1;
pub const WL_SURFACE_LEAVE_SINCE_VERSION: u32 = 1;
pub const WL_SURFACE_SET_BUFFER_SCALE_SINCE_VERSION: u32 = 3;
pub const WL_SURFACE_SET_BUFFER_TRANSFORM_SINCE_VERSION: u32 = 2;
pub const WL_SURFACE_SET_INPUT_REGION_SINCE_VERSION: u32 = 1;
pub const WL_SURFACE_SET_OPAQUE_REGION_SINCE_VERSION: u32 = 1;
pub const WL_TOUCH_CANCEL: u32 = 4;
pub const WL_TOUCH_CANCEL_SINCE_VERSION: u32 = 1;
pub const WL_TOUCH_DOWN: u32 = 0;
pub const WL_TOUCH_DOWN_SINCE_VERSION: u32 = 1;
pub const WL_TOUCH_FRAME: u32 = 3;
pub const WL_TOUCH_FRAME_SINCE_VERSION: u32 = 1;
pub const WL_TOUCH_MOTION: u32 = 2;
pub const WL_TOUCH_MOTION_SINCE_VERSION: u32 = 1;
pub const WL_TOUCH_ORIENTATION: u32 = 6;
pub const WL_TOUCH_ORIENTATION_SINCE_VERSION: u32 = 6;
pub const WL_TOUCH_RELEASE_SINCE_VERSION: u32 = 3;
pub const WL_TOUCH_SHAPE: u32 = 5;
pub const WL_TOUCH_SHAPE_SINCE_VERSION: u32 = 6;
pub const WL_TOUCH_UP: u32 = 1;
pub const WL_TOUCH_UP_SINCE_VERSION: u32 = 1;

unsafe extern "C" {
    pub static wl_buffer_interface: wl_interface;
    pub static wl_callback_interface: wl_interface;
    pub static wl_compositor_interface: wl_interface;
    pub static wl_data_device_interface: wl_interface;
    pub static wl_data_device_manager_interface: wl_interface;
    pub static wl_data_offer_interface: wl_interface;
    pub static wl_data_source_interface: wl_interface;
    pub static wl_display_interface: wl_interface;
    pub static wl_keyboard_interface: wl_interface;
    pub static wl_output_interface: wl_interface;
    pub static wl_pointer_interface: wl_interface;
    pub static wl_region_interface: wl_interface;
    pub static wl_registry_interface: wl_interface;
    pub static wl_seat_interface: wl_interface;
    pub static wl_shell_interface: wl_interface;
    pub static wl_shell_surface_interface: wl_interface;
    pub static wl_shm_interface: wl_interface;
    pub static wl_shm_pool_interface: wl_interface;
    pub static wl_subcompositor_interface: wl_interface;
    pub static wl_subsurface_interface: wl_interface;
    pub static wl_surface_interface: wl_interface;
    pub static wl_touch_interface: wl_interface;
}

#[inline]
pub unsafe extern "C" fn wl_buffer_send_release(
    resource_: *mut wl_resource,
) {
    unsafe {
        wl_resource_post_event(
            resource_,
            WL_BUFFER_RELEASE,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_callback_send_done(
    resource_: *mut wl_resource,
    callback_data: u32,
) {
    unsafe {
        wl_resource_post_event(
            resource_,
            WL_CALLBACK_DONE,
            callback_data,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_data_device_send_data_offer(
    resource_: *mut wl_resource,
    id: *mut wl_resource,
) {
    unsafe {
        wl_resource_post_event(
            resource_,
            WL_DATA_DEVICE_DATA_OFFER,
            id,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_data_device_send_drop(
    resource_: *mut wl_resource,
) {
    unsafe {
        wl_resource_post_event(
            resource_,
            WL_DATA_DEVICE_DROP,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_data_device_send_enter(
    resource_: *mut wl_resource,
    serial: u32,
    surface: *mut wl_resource,
    x: wl_fixed_t,
    y: wl_fixed_t,
    id: *mut wl_resource,
) {
    unsafe {
        wl_resource_post_event(
            resource_,
            WL_DATA_DEVICE_ENTER,
            serial,
            surface,
            x,
            y,
            id,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_data_device_send_leave(
    resource_: *mut wl_resource,
) {
    unsafe {
        wl_resource_post_event(
            resource_,
            WL_DATA_DEVICE_LEAVE,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_data_device_send_motion(
    resource_: *mut wl_resource,
    time: u32,
    x: wl_fixed_t,
    y: wl_fixed_t,
) {
    unsafe {
        wl_resource_post_event(
            resource_,
            WL_DATA_DEVICE_MOTION,
            time,
            x,
            y,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_data_device_send_selection(
    resource_: *mut wl_resource,
    id: *mut wl_resource,
) {
    unsafe {
        wl_resource_post_event(
            resource_,
            WL_DATA_DEVICE_SELECTION,
            id,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_data_offer_send_action(
    resource_: *mut wl_resource,
    dnd_action: u32,
) {
    unsafe {
        wl_resource_post_event(
            resource_,
            WL_DATA_OFFER_ACTION,
            dnd_action,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_data_offer_send_offer(
    resource_: *mut wl_resource,
    mime_type: *const c_char,
) {
    unsafe {
        wl_resource_post_event(
            resource_,
            WL_DATA_OFFER_OFFER,
            mime_type,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_data_offer_send_source_actions(
    resource_: *mut wl_resource,
    source_actions: u32,
) {
    unsafe {
        wl_resource_post_event(
            resource_,
            WL_DATA_OFFER_SOURCE_ACTIONS,
            source_actions,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_data_source_send_action(
    resource_: *mut wl_resource,
    dnd_action: u32,
) {
    unsafe {
        wl_resource_post_event(
            resource_,
            WL_DATA_SOURCE_ACTION,
            dnd_action,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_data_source_send_cancelled(
    resource_: *mut wl_resource,
) {
    unsafe {
        wl_resource_post_event(
            resource_,
            WL_DATA_SOURCE_CANCELLED,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_data_source_send_dnd_drop_performed(
    resource_: *mut wl_resource,
) {
    unsafe {
        wl_resource_post_event(
            resource_,
            WL_DATA_SOURCE_DND_DROP_PERFORMED,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_data_source_send_dnd_finished(
    resource_: *mut wl_resource,
) {
    unsafe {
        wl_resource_post_event(
            resource_,
            WL_DATA_SOURCE_DND_FINISHED,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_data_source_send_send(
    resource_: *mut wl_resource,
    mime_type: *const c_char,
    fd: i32,
) {
    unsafe {
        wl_resource_post_event(
            resource_,
            WL_DATA_SOURCE_SEND,
            mime_type,
            fd,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_data_source_send_target(
    resource_: *mut wl_resource,
    mime_type: *const c_char,
) {
    unsafe {
        wl_resource_post_event(
            resource_,
            WL_DATA_SOURCE_TARGET,
            mime_type,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_keyboard_send_enter(
    resource_: *mut wl_resource,
    serial: u32,
    surface: *mut wl_resource,
    keys: *mut wl_array,
) {
    unsafe {
        wl_resource_post_event(
            resource_,
            WL_KEYBOARD_ENTER,
            serial,
            surface,
            keys,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_keyboard_send_key(
    resource_: *mut wl_resource,
    serial: u32,
    time: u32,
    key: u32,
    state: u32,
) {
    unsafe {
        wl_resource_post_event(
            resource_,
            WL_KEYBOARD_KEY,
            serial,
            time,
            key,
            state,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_keyboard_send_keymap(
    resource_: *mut wl_resource,
    format: u32,
    fd: i32,
    size: u32,
) {
    unsafe {
        wl_resource_post_event(
            resource_,
            WL_KEYBOARD_KEYMAP,
            format,
            fd,
            size,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_keyboard_send_leave(
    resource_: *mut wl_resource,
    serial: u32,
    surface: *mut wl_resource,
) {
    unsafe {
        wl_resource_post_event(
            resource_,
            WL_KEYBOARD_LEAVE,
            serial,
            surface,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_keyboard_send_modifiers(
    resource_: *mut wl_resource,
    serial: u32,
    mods_depressed: u32,
    mods_latched: u32,
    mods_locked: u32,
    group: u32,
) {
    unsafe {
        wl_resource_post_event(
            resource_,
            WL_KEYBOARD_MODIFIERS,
            serial,
            mods_depressed,
            mods_latched,
            mods_locked,
            group,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_keyboard_send_repeat_info(
    resource_: *mut wl_resource,
    rate: i32,
    delay: i32,
) {
    unsafe {
        wl_resource_post_event(
            resource_,
            WL_KEYBOARD_REPEAT_INFO,
            rate,
            delay,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_output_send_done(
    resource_: *mut wl_resource,
) {
    unsafe {
        wl_resource_post_event(
            resource_,
            WL_OUTPUT_DONE,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_output_send_geometry(
    resource_: *mut wl_resource,
    x: i32,
    y: i32,
    physical_width: i32,
    physical_height: i32,
    subpixel: i32,
    make: *const c_char,
    model: *const c_char,
    transform: i32,
) {
    unsafe {
        wl_resource_post_event(
            resource_,
            WL_OUTPUT_GEOMETRY,
            x,
            y,
            physical_width,
            physical_height,
            subpixel,
            make,
            model,
            transform,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_output_send_mode(
    resource_: *mut wl_resource,
    flags: u32,
    width: i32,
    height: i32,
    refresh: i32,
) {
    unsafe {
        wl_resource_post_event(
            resource_,
            WL_OUTPUT_MODE,
            flags,
            width,
            height,
            refresh,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_output_send_scale(
    resource_: *mut wl_resource,
    factor: i32,
) {
    unsafe {
        wl_resource_post_event(
            resource_,
            WL_OUTPUT_SCALE,
            factor,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_pointer_send_axis(
    resource_: *mut wl_resource,
    time: u32,
    axis: u32,
    value: wl_fixed_t,
) {
    unsafe {
        wl_resource_post_event(
            resource_,
            WL_POINTER_AXIS,
            time,
            axis,
            value,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_pointer_send_axis_discrete(
    resource_: *mut wl_resource,
    axis: u32,
    discrete: i32,
) {
    unsafe {
        wl_resource_post_event(
            resource_,
            WL_POINTER_AXIS_DISCRETE,
            axis,
            discrete,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_pointer_send_axis_source(
    resource_: *mut wl_resource,
    axis_source: u32,
) {
    unsafe {
        wl_resource_post_event(
            resource_,
            WL_POINTER_AXIS_SOURCE,
            axis_source,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_pointer_send_axis_stop(
    resource_: *mut wl_resource,
    time: u32,
    axis: u32,
) {
    unsafe {
        wl_resource_post_event(
            resource_,
            WL_POINTER_AXIS_STOP,
            time,
            axis,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_pointer_send_button(
    resource_: *mut wl_resource,
    serial: u32,
    time: u32,
    button: u32,
    state: u32,
) {
    unsafe {
        wl_resource_post_event(
            resource_,
            WL_POINTER_BUTTON,
            serial,
            time,
            button,
            state,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_pointer_send_enter(
    resource_: *mut wl_resource,
    serial: u32,
    surface: *mut wl_resource,
    surface_x: wl_fixed_t,
    surface_y: wl_fixed_t,
) {
    unsafe {
        wl_resource_post_event(
            resource_,
            WL_POINTER_ENTER,
            serial,
            surface,
            surface_x,
            surface_y,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_pointer_send_frame(
    resource_: *mut wl_resource,
) {
    unsafe {
        wl_resource_post_event(
            resource_,
            WL_POINTER_FRAME,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_pointer_send_leave(
    resource_: *mut wl_resource,
    serial: u32,
    surface: *mut wl_resource,
) {
    unsafe {
        wl_resource_post_event(
            resource_,
            WL_POINTER_LEAVE,
            serial,
            surface,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_pointer_send_motion(
    resource_: *mut wl_resource,
    time: u32,
    surface_x: wl_fixed_t,
    surface_y: wl_fixed_t,
) {
    unsafe {
        wl_resource_post_event(
            resource_,
            WL_POINTER_MOTION,
            time,
            surface_x,
            surface_y,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_registry_send_global(
    resource_: *mut wl_resource,
    name: u32,
    interface: *const c_char,
    version: u32,
) {
    unsafe {
        wl_resource_post_event(
            resource_,
            WL_REGISTRY_GLOBAL,
            name,
            interface,
            version,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_registry_send_global_remove(
    resource_: *mut wl_resource,
    name: u32,
) {
    unsafe {
        wl_resource_post_event(
            resource_,
            WL_REGISTRY_GLOBAL_REMOVE,
            name,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_seat_send_capabilities(
    resource_: *mut wl_resource,
    capabilities: u32,
) {
    unsafe {
        wl_resource_post_event(
            resource_,
            WL_SEAT_CAPABILITIES,
            capabilities,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_seat_send_name(
    resource_: *mut wl_resource,
    name: *const c_char,
) {
    unsafe {
        wl_resource_post_event(
            resource_,
            WL_SEAT_NAME,
            name,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_shell_surface_send_configure(
    resource_: *mut wl_resource,
    edges: u32,
    width: i32,
    height: i32,
) {
    unsafe {
        wl_resource_post_event(
            resource_,
            WL_SHELL_SURFACE_CONFIGURE,
            edges,
            width,
            height,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_shell_surface_send_ping(
    resource_: *mut wl_resource,
    serial: u32,
) {
    unsafe {
        wl_resource_post_event(
            resource_,
            WL_SHELL_SURFACE_PING,
            serial,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_shell_surface_send_popup_done(
    resource_: *mut wl_resource,
) {
    unsafe {
        wl_resource_post_event(
            resource_,
            WL_SHELL_SURFACE_POPUP_DONE,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_shm_send_format(
    resource_: *mut wl_resource,
    format: u32,
) {
    unsafe {
        wl_resource_post_event(
            resource_,
            WL_SHM_FORMAT,
            format,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_surface_send_enter(
    resource_: *mut wl_resource,
    output: *mut wl_resource,
) {
    unsafe {
        wl_resource_post_event(
            resource_,
            WL_SURFACE_ENTER,
            output,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_surface_send_leave(
    resource_: *mut wl_resource,
    output: *mut wl_resource,
) {
    unsafe {
        wl_resource_post_event(
            resource_,
            WL_SURFACE_LEAVE,
            output,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_touch_send_cancel(
    resource_: *mut wl_resource,
) {
    unsafe {
        wl_resource_post_event(
            resource_,
            WL_TOUCH_CANCEL,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_touch_send_down(
    resource_: *mut wl_resource,
    serial: u32,
    time: u32,
    surface: *mut wl_resource,
    id: i32,
    x: wl_fixed_t,
    y: wl_fixed_t,
) {
    unsafe {
        wl_resource_post_event(
            resource_,
            WL_TOUCH_DOWN,
            serial,
            time,
            surface,
            id,
            x,
            y,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_touch_send_frame(
    resource_: *mut wl_resource,
) {
    unsafe {
        wl_resource_post_event(
            resource_,
            WL_TOUCH_FRAME,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_touch_send_motion(
    resource_: *mut wl_resource,
    time: u32,
    id: i32,
    x: wl_fixed_t,
    y: wl_fixed_t,
) {
    unsafe {
        wl_resource_post_event(
            resource_,
            WL_TOUCH_MOTION,
            time,
            id,
            x,
            y,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_touch_send_orientation(
    resource_: *mut wl_resource,
    id: i32,
    orientation: wl_fixed_t,
) {
    unsafe {
        wl_resource_post_event(
            resource_,
            WL_TOUCH_ORIENTATION,
            id,
            orientation,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_touch_send_shape(
    resource_: *mut wl_resource,
    id: i32,
    major: wl_fixed_t,
    minor: wl_fixed_t,
) {
    unsafe {
        wl_resource_post_event(
            resource_,
            WL_TOUCH_SHAPE,
            id,
            major,
            minor,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_touch_send_up(
    resource_: *mut wl_resource,
    serial: u32,
    time: u32,
    id: i32,
) {
    unsafe {
        wl_resource_post_event(
            resource_,
            WL_TOUCH_UP,
            serial,
            time,
            id,
        )
    }
}
