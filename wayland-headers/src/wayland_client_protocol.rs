use crate::prelude::*;
use super::wayland_client_core::*;

#[repr(C)]
pub struct wl_buffer {
    _data: (),
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct wl_buffer_listener {
    pub release: Option<unsafe extern "C" fn(
        data: *mut c_void,
        wl_buffer: *mut wl_buffer,
    )>,
}

#[repr(C)]
pub struct wl_callback {
    _data: (),
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct wl_callback_listener {
    pub done: Option<unsafe extern "C" fn(
        data: *mut c_void,
        wl_callback: *mut wl_callback,
        callback_data: u32,
    )>,
}

#[repr(C)]
pub struct wl_compositor {
    _data: (),
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[repr(C)]
pub struct wl_data_device {
    _data: (),
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct wl_data_device_listener {
    pub data_offer: Option<unsafe extern "C" fn(
        data: *mut c_void,
        wl_data_device: *mut wl_data_device,
        id: *mut wl_data_offer,
    )>,
    pub enter: Option<unsafe extern "C" fn(
        data: *mut c_void,
        wl_data_device: *mut wl_data_device,
        serial: u32,
        surface: *mut wl_surface,
        x: wl_fixed_t,
        y: wl_fixed_t,
        id: *mut wl_data_offer,
    )>,
    pub leave: Option<unsafe extern "C" fn(
        data: *mut c_void,
        wl_data_device: *mut wl_data_device,
    )>,
    pub motion: Option<unsafe extern "C" fn(
        data: *mut c_void,
        wl_data_device: *mut wl_data_device,
        time: u32,
        x: wl_fixed_t,
        y: wl_fixed_t,
    )>,
    pub drop: Option<unsafe extern "C" fn(
        data: *mut c_void,
        wl_data_device: *mut wl_data_device,
    )>,
    pub selection: Option<unsafe extern "C" fn(
        data: *mut c_void,
        wl_data_device: *mut wl_data_device,
        id: *mut wl_data_offer,
    )>,
}

#[repr(C)]
pub struct wl_data_device_manager {
    _data: (),
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[repr(C)]
pub struct wl_data_offer {
    _data: (),
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct wl_data_offer_listener {
    pub offer: Option<unsafe extern "C" fn(
        data: *mut c_void,
        wl_data_offer: *mut wl_data_offer,
        mime_type: *const c_char,
    )>,
    pub source_actions: Option<unsafe extern "C" fn(
        data: *mut c_void,
        wl_data_offer: *mut wl_data_offer,
        source_actions: u32,
    )>,
    pub action: Option<unsafe extern "C" fn(
        data: *mut c_void,
        wl_data_offer: *mut wl_data_offer,
        dnd_action: u32,
    )>,
}

#[repr(C)]
pub struct wl_data_source {
    _data: (),
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct wl_data_source_listener {
    pub target: Option<unsafe extern "C" fn(
        data: *mut c_void,
        wl_data_source: *mut wl_data_source,
        mime_type: *const c_char,
    )>,
    pub send: Option<unsafe extern "C" fn(
        data: *mut c_void,
        wl_data_source: *mut wl_data_source,
        mime_type: *const c_char,
        fd: i32,
    )>,
    pub cancelled: Option<unsafe extern "C" fn(
        data: *mut c_void,
        wl_data_source: *mut wl_data_source,
    )>,
    pub dnd_drop_performed: Option<unsafe extern "C" fn(
        data: *mut c_void,
        wl_data_source: *mut wl_data_source,
    )>,
    pub dnd_finished: Option<unsafe extern "C" fn(
        data: *mut c_void,
        wl_data_source: *mut wl_data_source,
    )>,
    pub action: Option<unsafe extern "C" fn(
        data: *mut c_void,
        wl_data_source: *mut wl_data_source,
        dnd_action: u32,
    )>,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct wl_display_listener {
    pub error: Option<unsafe extern "C" fn(
        data: *mut c_void,
        wl_display: *mut wl_display,
        object_id: *mut c_void,
        code: u32,
        message: *const c_char,
    )>,
    pub delete_id: Option<unsafe extern "C" fn(
        data: *mut c_void,
        wl_display: *mut wl_display,
        id: u32,
    )>,
}

#[repr(C)]
pub struct wl_fixes {
    _data: (),
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[repr(C)]
pub struct wl_keyboard {
    _data: (),
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct wl_keyboard_listener {
    pub keymap: Option<unsafe extern "C" fn(
        data: *mut c_void,
        wl_keyboard: *mut wl_keyboard,
        format: u32,
        fd: i32,
        size: u32,
    )>,
    pub enter: Option<unsafe extern "C" fn(
        data: *mut c_void,
        wl_keyboard: *mut wl_keyboard,
        serial: u32,
        surface: *mut wl_surface,
        keys: *mut wl_array,
    )>,
    pub leave: Option<unsafe extern "C" fn(
        data: *mut c_void,
        wl_keyboard: *mut wl_keyboard,
        serial: u32,
        surface: *mut wl_surface,
    )>,
    pub key: Option<unsafe extern "C" fn(
        data: *mut c_void,
        wl_keyboard: *mut wl_keyboard,
        serial: u32,
        time: u32,
        key: u32,
        state: u32,
    )>,
    pub modifiers: Option<unsafe extern "C" fn(
        data: *mut c_void,
        wl_keyboard: *mut wl_keyboard,
        serial: u32,
        mods_depressed: u32,
        mods_latched: u32,
        mods_locked: u32,
        group: u32,
    )>,
    pub repeat_info: Option<unsafe extern "C" fn(
        data: *mut c_void,
        wl_keyboard: *mut wl_keyboard,
        rate: i32,
        delay: i32,
    )>,
}

#[repr(C)]
pub struct wl_output {
    _data: (),
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct wl_output_listener {
    pub geometry: Option<unsafe extern "C" fn(
        data: *mut c_void,
        wl_output: *mut wl_output,
        x: i32,
        y: i32,
        physical_width: i32,
        physical_height: i32,
        subpixel: i32,
        make: *const c_char,
        model: *const c_char,
        transform: i32,
    )>,
    pub mode: Option<unsafe extern "C" fn(
        data: *mut c_void,
        wl_output: *mut wl_output,
        flags: u32,
        width: i32,
        height: i32,
        refresh: i32,
    )>,
    pub done: Option<unsafe extern "C" fn(
        data: *mut c_void,
        wl_output: *mut wl_output,
    )>,
    pub scale: Option<unsafe extern "C" fn(
        data: *mut c_void,
        wl_output: *mut wl_output,
        factor: i32,
    )>,
    pub name: Option<unsafe extern "C" fn(
        data: *mut c_void,
        wl_output: *mut wl_output,
        name: *const c_char,
    )>,
    pub description: Option<unsafe extern "C" fn(
        data: *mut c_void,
        wl_output: *mut wl_output,
        description: *const c_char,
    )>,
}

#[repr(C)]
pub struct wl_pointer {
    _data: (),
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct wl_pointer_listener {
    pub enter: Option<unsafe extern "C" fn(
        data: *mut c_void,
        wl_pointer: *mut wl_pointer,
        serial: u32,
        surface: *mut wl_surface,
        surface_x: wl_fixed_t,
        surface_y: wl_fixed_t,
    )>,
    pub leave: Option<unsafe extern "C" fn(
        data: *mut c_void,
        wl_pointer: *mut wl_pointer,
        serial: u32,
        surface: *mut wl_surface,
    )>,
    pub motion: Option<unsafe extern "C" fn(
        data: *mut c_void,
        wl_pointer: *mut wl_pointer,
        time: u32,
        surface_x: wl_fixed_t,
        surface_y: wl_fixed_t,
    )>,
    pub button: Option<unsafe extern "C" fn(
        data: *mut c_void,
        wl_pointer: *mut wl_pointer,
        serial: u32,
        time: u32,
        button: u32,
        state: u32,
    )>,
    pub axis: Option<unsafe extern "C" fn(
        data: *mut c_void,
        wl_pointer: *mut wl_pointer,
        time: u32,
        axis: u32,
        value: wl_fixed_t,
    )>,
    pub frame: Option<unsafe extern "C" fn(
        data: *mut c_void,
        wl_pointer: *mut wl_pointer,
    )>,
    pub axis_source: Option<unsafe extern "C" fn(
        data: *mut c_void,
        wl_pointer: *mut wl_pointer,
        axis_source: u32,
    )>,
    pub axis_stop: Option<unsafe extern "C" fn(
        data: *mut c_void,
        wl_pointer: *mut wl_pointer,
        time: u32,
        axis: u32,
    )>,
    pub axis_discrete: Option<unsafe extern "C" fn(
        data: *mut c_void,
        wl_pointer: *mut wl_pointer,
        axis: u32,
        discrete: i32,
    )>,
    pub axis_value120: Option<unsafe extern "C" fn(
        data: *mut c_void,
        wl_pointer: *mut wl_pointer,
        axis: u32,
        value120: i32,
    )>,
    pub axis_relative_direction: Option<unsafe extern "C" fn(
        data: *mut c_void,
        wl_pointer: *mut wl_pointer,
        axis: u32,
        direction: u32,
    )>,
}

#[repr(C)]
pub struct wl_region {
    _data: (),
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[repr(C)]
pub struct wl_registry {
    _data: (),
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct wl_registry_listener {
    pub global: Option<unsafe extern "C" fn(
        data: *mut c_void,
        wl_registry: *mut wl_registry,
        name: u32,
        interface: *const c_char,
        version: u32,
    )>,
    pub global_remove: Option<unsafe extern "C" fn(
        data: *mut c_void,
        wl_registry: *mut wl_registry,
        name: u32,
    )>,
}

#[repr(C)]
pub struct wl_seat {
    _data: (),
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct wl_seat_listener {
    pub capabilities: Option<unsafe extern "C" fn(
        data: *mut c_void,
        wl_seat: *mut wl_seat,
        capabilities: u32,
    )>,
    pub name: Option<unsafe extern "C" fn(
        data: *mut c_void,
        wl_seat: *mut wl_seat,
        name: *const c_char,
    )>,
}

#[repr(C)]
pub struct wl_shell {
    _data: (),
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[repr(C)]
pub struct wl_shell_surface {
    _data: (),
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct wl_shell_surface_listener {
    pub ping: Option<unsafe extern "C" fn(
        data: *mut c_void,
        wl_shell_surface: *mut wl_shell_surface,
        serial: u32,
    )>,
    pub configure: Option<unsafe extern "C" fn(
        data: *mut c_void,
        wl_shell_surface: *mut wl_shell_surface,
        edges: u32,
        width: i32,
        height: i32,
    )>,
    pub popup_done: Option<unsafe extern "C" fn(
        data: *mut c_void,
        wl_shell_surface: *mut wl_shell_surface,
    )>,
}

#[repr(C)]
pub struct wl_shm {
    _data: (),
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct wl_shm_listener {
    pub format: Option<unsafe extern "C" fn(
        data: *mut c_void,
        wl_shm: *mut wl_shm,
        format: u32,
    )>,
}

#[repr(C)]
pub struct wl_shm_pool {
    _data: (),
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[repr(C)]
pub struct wl_subcompositor {
    _data: (),
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[repr(C)]
pub struct wl_subsurface {
    _data: (),
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[repr(C)]
pub struct wl_surface {
    _data: (),
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct wl_surface_listener {
    pub enter: Option<unsafe extern "C" fn(
        data: *mut c_void,
        wl_surface: *mut wl_surface,
        output: *mut wl_output,
    )>,
    pub leave: Option<unsafe extern "C" fn(
        data: *mut c_void,
        wl_surface: *mut wl_surface,
        output: *mut wl_output,
    )>,
    pub preferred_buffer_scale: Option<unsafe extern "C" fn(
        data: *mut c_void,
        wl_surface: *mut wl_surface,
        factor: i32,
    )>,
    pub preferred_buffer_transform: Option<unsafe extern "C" fn(
        data: *mut c_void,
        wl_surface: *mut wl_surface,
        transform: u32,
    )>,
}

#[repr(C)]
pub struct wl_touch {
    _data: (),
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct wl_touch_listener {
    pub down: Option<unsafe extern "C" fn(
        data: *mut c_void,
        wl_touch: *mut wl_touch,
        serial: u32,
        time: u32,
        surface: *mut wl_surface,
        id: i32,
        x: wl_fixed_t,
        y: wl_fixed_t,
    )>,
    pub up: Option<unsafe extern "C" fn(
        data: *mut c_void,
        wl_touch: *mut wl_touch,
        serial: u32,
        time: u32,
        id: i32,
    )>,
    pub motion: Option<unsafe extern "C" fn(
        data: *mut c_void,
        wl_touch: *mut wl_touch,
        time: u32,
        id: i32,
        x: wl_fixed_t,
        y: wl_fixed_t,
    )>,
    pub frame: Option<unsafe extern "C" fn(
        data: *mut c_void,
        wl_touch: *mut wl_touch,
    )>,
    pub cancel: Option<unsafe extern "C" fn(
        data: *mut c_void,
        wl_touch: *mut wl_touch,
    )>,
    pub shape: Option<unsafe extern "C" fn(
        data: *mut c_void,
        wl_touch: *mut wl_touch,
        id: i32,
        major: wl_fixed_t,
        minor: wl_fixed_t,
    )>,
    pub orientation: Option<unsafe extern "C" fn(
        data: *mut c_void,
        wl_touch: *mut wl_touch,
        id: i32,
        orientation: wl_fixed_t,
    )>,
}

pub const WL_BUFFER_DESTROY: u32 = 0;
pub const WL_BUFFER_DESTROY_SINCE_VERSION: u32 = 1;
pub const WL_BUFFER_RELEASE_SINCE_VERSION: u32 = 1;
pub const WL_CALLBACK_DONE_SINCE_VERSION: u32 = 1;
pub const WL_COMPOSITOR_CREATE_REGION: u32 = 1;
pub const WL_COMPOSITOR_CREATE_REGION_SINCE_VERSION: u32 = 1;
pub const WL_COMPOSITOR_CREATE_SURFACE: u32 = 0;
pub const WL_COMPOSITOR_CREATE_SURFACE_SINCE_VERSION: u32 = 1;
pub const WL_DATA_DEVICE_DATA_OFFER_SINCE_VERSION: u32 = 1;
pub const WL_DATA_DEVICE_DROP_SINCE_VERSION: u32 = 1;
pub const WL_DATA_DEVICE_ENTER_SINCE_VERSION: u32 = 1;
pub const WL_DATA_DEVICE_ERROR_ROLE: u32 = 0;
pub const WL_DATA_DEVICE_ERROR_USED_SOURCE: u32 = 1;
pub const WL_DATA_DEVICE_LEAVE_SINCE_VERSION: u32 = 1;
pub const WL_DATA_DEVICE_MANAGER_CREATE_DATA_SOURCE: u32 = 0;
pub const WL_DATA_DEVICE_MANAGER_CREATE_DATA_SOURCE_SINCE_VERSION: u32 = 1;
pub const WL_DATA_DEVICE_MANAGER_DND_ACTION_ASK: u32 = 4;
pub const WL_DATA_DEVICE_MANAGER_DND_ACTION_COPY: u32 = 1;
pub const WL_DATA_DEVICE_MANAGER_DND_ACTION_MOVE: u32 = 2;
pub const WL_DATA_DEVICE_MANAGER_DND_ACTION_NONE: u32 = 0;
pub const WL_DATA_DEVICE_MANAGER_GET_DATA_DEVICE: u32 = 1;
pub const WL_DATA_DEVICE_MANAGER_GET_DATA_DEVICE_SINCE_VERSION: u32 = 1;
pub const WL_DATA_DEVICE_MOTION_SINCE_VERSION: u32 = 1;
pub const WL_DATA_DEVICE_RELEASE: u32 = 2;
pub const WL_DATA_DEVICE_RELEASE_SINCE_VERSION: u32 = 2;
pub const WL_DATA_DEVICE_SELECTION_SINCE_VERSION: u32 = 1;
pub const WL_DATA_DEVICE_SET_SELECTION: u32 = 1;
pub const WL_DATA_DEVICE_SET_SELECTION_SINCE_VERSION: u32 = 1;
pub const WL_DATA_DEVICE_START_DRAG: u32 = 0;
pub const WL_DATA_DEVICE_START_DRAG_SINCE_VERSION: u32 = 1;
pub const WL_DATA_OFFER_ACCEPT: u32 = 0;
pub const WL_DATA_OFFER_ACCEPT_SINCE_VERSION: u32 = 1;
pub const WL_DATA_OFFER_ACTION_SINCE_VERSION: u32 = 3;
pub const WL_DATA_OFFER_DESTROY: u32 = 2;
pub const WL_DATA_OFFER_DESTROY_SINCE_VERSION: u32 = 1;
pub const WL_DATA_OFFER_ERROR_INVALID_ACTION: u32 = 2;
pub const WL_DATA_OFFER_ERROR_INVALID_ACTION_MASK: u32 = 1;
pub const WL_DATA_OFFER_ERROR_INVALID_FINISH: u32 = 0;
pub const WL_DATA_OFFER_ERROR_INVALID_OFFER: u32 = 3;
pub const WL_DATA_OFFER_FINISH: u32 = 3;
pub const WL_DATA_OFFER_FINISH_SINCE_VERSION: u32 = 3;
pub const WL_DATA_OFFER_OFFER_SINCE_VERSION: u32 = 1;
pub const WL_DATA_OFFER_RECEIVE: u32 = 1;
pub const WL_DATA_OFFER_RECEIVE_SINCE_VERSION: u32 = 1;
pub const WL_DATA_OFFER_SET_ACTIONS: u32 = 4;
pub const WL_DATA_OFFER_SET_ACTIONS_SINCE_VERSION: u32 = 3;
pub const WL_DATA_OFFER_SOURCE_ACTIONS_SINCE_VERSION: u32 = 3;
pub const WL_DATA_SOURCE_ACTION_SINCE_VERSION: u32 = 3;
pub const WL_DATA_SOURCE_CANCELLED_SINCE_VERSION: u32 = 1;
pub const WL_DATA_SOURCE_DESTROY: u32 = 1;
pub const WL_DATA_SOURCE_DESTROY_SINCE_VERSION: u32 = 1;
pub const WL_DATA_SOURCE_DND_DROP_PERFORMED_SINCE_VERSION: u32 = 3;
pub const WL_DATA_SOURCE_DND_FINISHED_SINCE_VERSION: u32 = 3;
pub const WL_DATA_SOURCE_ERROR_INVALID_ACTION_MASK: u32 = 0;
pub const WL_DATA_SOURCE_ERROR_INVALID_SOURCE: u32 = 1;
pub const WL_DATA_SOURCE_OFFER: u32 = 0;
pub const WL_DATA_SOURCE_OFFER_SINCE_VERSION: u32 = 1;
pub const WL_DATA_SOURCE_SEND_SINCE_VERSION: u32 = 1;
pub const WL_DATA_SOURCE_SET_ACTIONS: u32 = 2;
pub const WL_DATA_SOURCE_SET_ACTIONS_SINCE_VERSION: u32 = 3;
pub const WL_DATA_SOURCE_TARGET_SINCE_VERSION: u32 = 1;
pub const WL_DISPLAY_DELETE_ID_SINCE_VERSION: u32 = 1;
pub const WL_DISPLAY_ERROR_IMPLEMENTATION: u32 = 3;
pub const WL_DISPLAY_ERROR_INVALID_METHOD: u32 = 1;
pub const WL_DISPLAY_ERROR_INVALID_OBJECT: u32 = 0;
pub const WL_DISPLAY_ERROR_NO_MEMORY: u32 = 2;
pub const WL_DISPLAY_ERROR_SINCE_VERSION: u32 = 1;
pub const WL_DISPLAY_GET_REGISTRY: u32 = 1;
pub const WL_DISPLAY_GET_REGISTRY_SINCE_VERSION: u32 = 1;
pub const WL_DISPLAY_SYNC: u32 = 0;
pub const WL_DISPLAY_SYNC_SINCE_VERSION: u32 = 1;
pub const WL_FIXES_DESTROY: u32 = 0;
pub const WL_FIXES_DESTROY_REGISTRY: u32 = 1;
pub const WL_FIXES_DESTROY_REGISTRY_SINCE_VERSION: u32 = 1;
pub const WL_FIXES_DESTROY_SINCE_VERSION: u32 = 1;
pub const WL_KEYBOARD_ENTER_SINCE_VERSION: u32 = 1;
pub const WL_KEYBOARD_KEYMAP_FORMAT_NO_KEYMAP: u32 = 0;
pub const WL_KEYBOARD_KEYMAP_FORMAT_XKB_V1: u32 = 1;
pub const WL_KEYBOARD_KEYMAP_SINCE_VERSION: u32 = 1;
pub const WL_KEYBOARD_KEY_SINCE_VERSION: u32 = 1;
pub const WL_KEYBOARD_KEY_STATE_PRESSED: u32 = 1;
pub const WL_KEYBOARD_KEY_STATE_RELEASED: u32 = 0;
pub const WL_KEYBOARD_KEY_STATE_REPEATED: u32 = 2;
pub const WL_KEYBOARD_KEY_STATE_REPEATED_SINCE_VERSION: u32 = 10;
pub const WL_KEYBOARD_LEAVE_SINCE_VERSION: u32 = 1;
pub const WL_KEYBOARD_MODIFIERS_SINCE_VERSION: u32 = 1;
pub const WL_KEYBOARD_RELEASE: u32 = 0;
pub const WL_KEYBOARD_RELEASE_SINCE_VERSION: u32 = 3;
pub const WL_KEYBOARD_REPEAT_INFO_SINCE_VERSION: u32 = 4;
pub const WL_OUTPUT_DESCRIPTION_SINCE_VERSION: u32 = 4;
pub const WL_OUTPUT_DONE_SINCE_VERSION: u32 = 2;
pub const WL_OUTPUT_GEOMETRY_SINCE_VERSION: u32 = 1;
pub const WL_OUTPUT_MODE_CURRENT: u32 = 0x1;
pub const WL_OUTPUT_MODE_PREFERRED: u32 = 0x2;
pub const WL_OUTPUT_MODE_SINCE_VERSION: u32 = 1;
pub const WL_OUTPUT_NAME_SINCE_VERSION: u32 = 4;
pub const WL_OUTPUT_RELEASE: u32 = 0;
pub const WL_OUTPUT_RELEASE_SINCE_VERSION: u32 = 3;
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
pub const WL_POINTER_AXIS_DISCRETE_SINCE_VERSION: u32 = 5;
pub const WL_POINTER_AXIS_HORIZONTAL_SCROLL: u32 = 1;
pub const WL_POINTER_AXIS_RELATIVE_DIRECTION_IDENTICAL: u32 = 0;
pub const WL_POINTER_AXIS_RELATIVE_DIRECTION_INVERTED: u32 = 1;
pub const WL_POINTER_AXIS_RELATIVE_DIRECTION_SINCE_VERSION: u32 = 9;
pub const WL_POINTER_AXIS_SINCE_VERSION: u32 = 1;
pub const WL_POINTER_AXIS_SOURCE_CONTINUOUS: u32 = 2;
pub const WL_POINTER_AXIS_SOURCE_FINGER: u32 = 1;
pub const WL_POINTER_AXIS_SOURCE_SINCE_VERSION: u32 = 5;
pub const WL_POINTER_AXIS_SOURCE_WHEEL: u32 = 0;
pub const WL_POINTER_AXIS_SOURCE_WHEEL_TILT: u32 = 3;
pub const WL_POINTER_AXIS_SOURCE_WHEEL_TILT_SINCE_VERSION: u32 = 6;
pub const WL_POINTER_AXIS_STOP_SINCE_VERSION: u32 = 5;
pub const WL_POINTER_AXIS_VALUE120_SINCE_VERSION: u32 = 8;
pub const WL_POINTER_AXIS_VERTICAL_SCROLL: u32 = 0;
pub const WL_POINTER_BUTTON_SINCE_VERSION: u32 = 1;
pub const WL_POINTER_BUTTON_STATE_PRESSED: u32 = 1;
pub const WL_POINTER_BUTTON_STATE_RELEASED: u32 = 0;
pub const WL_POINTER_ENTER_SINCE_VERSION: u32 = 1;
pub const WL_POINTER_ERROR_ROLE: u32 = 0;
pub const WL_POINTER_FRAME_SINCE_VERSION: u32 = 5;
pub const WL_POINTER_LEAVE_SINCE_VERSION: u32 = 1;
pub const WL_POINTER_MOTION_SINCE_VERSION: u32 = 1;
pub const WL_POINTER_RELEASE: u32 = 1;
pub const WL_POINTER_RELEASE_SINCE_VERSION: u32 = 3;
pub const WL_POINTER_SET_CURSOR: u32 = 0;
pub const WL_POINTER_SET_CURSOR_SINCE_VERSION: u32 = 1;
pub const WL_REGION_ADD: u32 = 1;
pub const WL_REGION_ADD_SINCE_VERSION: u32 = 1;
pub const WL_REGION_DESTROY: u32 = 0;
pub const WL_REGION_DESTROY_SINCE_VERSION: u32 = 1;
pub const WL_REGION_SUBTRACT: u32 = 2;
pub const WL_REGION_SUBTRACT_SINCE_VERSION: u32 = 1;
pub const WL_REGISTRY_BIND: u32 = 0;
pub const WL_REGISTRY_BIND_SINCE_VERSION: u32 = 1;
pub const WL_REGISTRY_GLOBAL_REMOVE_SINCE_VERSION: u32 = 1;
pub const WL_REGISTRY_GLOBAL_SINCE_VERSION: u32 = 1;
pub const WL_SEAT_CAPABILITIES_SINCE_VERSION: u32 = 1;
pub const WL_SEAT_CAPABILITY_KEYBOARD: u32 = 2;
pub const WL_SEAT_CAPABILITY_POINTER: u32 = 1;
pub const WL_SEAT_CAPABILITY_TOUCH: u32 = 4;
pub const WL_SEAT_ERROR_MISSING_CAPABILITY: u32 = 0;
pub const WL_SEAT_GET_KEYBOARD: u32 = 1;
pub const WL_SEAT_GET_KEYBOARD_SINCE_VERSION: u32 = 1;
pub const WL_SEAT_GET_POINTER: u32 = 0;
pub const WL_SEAT_GET_POINTER_SINCE_VERSION: u32 = 1;
pub const WL_SEAT_GET_TOUCH: u32 = 2;
pub const WL_SEAT_GET_TOUCH_SINCE_VERSION: u32 = 1;
pub const WL_SEAT_NAME_SINCE_VERSION: u32 = 2;
pub const WL_SEAT_RELEASE: u32 = 3;
pub const WL_SEAT_RELEASE_SINCE_VERSION: u32 = 5;
pub const WL_SHELL_ERROR_ROLE: u32 = 0;
pub const WL_SHELL_GET_SHELL_SURFACE: u32 = 0;
pub const WL_SHELL_GET_SHELL_SURFACE_SINCE_VERSION: u32 = 1;
pub const WL_SHELL_SURFACE_CONFIGURE_SINCE_VERSION: u32 = 1;
pub const WL_SHELL_SURFACE_FULLSCREEN_METHOD_DEFAULT: u32 = 0;
pub const WL_SHELL_SURFACE_FULLSCREEN_METHOD_DRIVER: u32 = 2;
pub const WL_SHELL_SURFACE_FULLSCREEN_METHOD_FILL: u32 = 3;
pub const WL_SHELL_SURFACE_FULLSCREEN_METHOD_SCALE: u32 = 1;
pub const WL_SHELL_SURFACE_MOVE: u32 = 1;
pub const WL_SHELL_SURFACE_MOVE_SINCE_VERSION: u32 = 1;
pub const WL_SHELL_SURFACE_PING_SINCE_VERSION: u32 = 1;
pub const WL_SHELL_SURFACE_PONG: u32 = 0;
pub const WL_SHELL_SURFACE_PONG_SINCE_VERSION: u32 = 1;
pub const WL_SHELL_SURFACE_POPUP_DONE_SINCE_VERSION: u32 = 1;
pub const WL_SHELL_SURFACE_RESIZE: u32 = 2;
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
pub const WL_SHELL_SURFACE_SET_CLASS: u32 = 9;
pub const WL_SHELL_SURFACE_SET_CLASS_SINCE_VERSION: u32 = 1;
pub const WL_SHELL_SURFACE_SET_FULLSCREEN: u32 = 5;
pub const WL_SHELL_SURFACE_SET_FULLSCREEN_SINCE_VERSION: u32 = 1;
pub const WL_SHELL_SURFACE_SET_MAXIMIZED: u32 = 7;
pub const WL_SHELL_SURFACE_SET_MAXIMIZED_SINCE_VERSION: u32 = 1;
pub const WL_SHELL_SURFACE_SET_POPUP: u32 = 6;
pub const WL_SHELL_SURFACE_SET_POPUP_SINCE_VERSION: u32 = 1;
pub const WL_SHELL_SURFACE_SET_TITLE: u32 = 8;
pub const WL_SHELL_SURFACE_SET_TITLE_SINCE_VERSION: u32 = 1;
pub const WL_SHELL_SURFACE_SET_TOPLEVEL: u32 = 3;
pub const WL_SHELL_SURFACE_SET_TOPLEVEL_SINCE_VERSION: u32 = 1;
pub const WL_SHELL_SURFACE_SET_TRANSIENT: u32 = 4;
pub const WL_SHELL_SURFACE_SET_TRANSIENT_SINCE_VERSION: u32 = 1;
pub const WL_SHELL_SURFACE_TRANSIENT_INACTIVE: u32 = 0x1;
pub const WL_SHM_CREATE_POOL: u32 = 0;
pub const WL_SHM_CREATE_POOL_SINCE_VERSION: u32 = 1;
pub const WL_SHM_ERROR_INVALID_FD: u32 = 2;
pub const WL_SHM_ERROR_INVALID_FORMAT: u32 = 0;
pub const WL_SHM_ERROR_INVALID_STRIDE: u32 = 1;
pub const WL_SHM_FORMAT_ABGR1555: u32 = 0x35314241;
pub const WL_SHM_FORMAT_ABGR16161616: u32 = 0x38344241;
pub const WL_SHM_FORMAT_ABGR16161616F: u32 = 0x48344241;
pub const WL_SHM_FORMAT_ABGR2101010: u32 = 0x30334241;
pub const WL_SHM_FORMAT_ABGR4444: u32 = 0x32314241;
pub const WL_SHM_FORMAT_ABGR8888: u32 = 0x34324241;
pub const WL_SHM_FORMAT_ARGB1555: u32 = 0x35315241;
pub const WL_SHM_FORMAT_ARGB16161616: u32 = 0x38345241;
pub const WL_SHM_FORMAT_ARGB16161616F: u32 = 0x48345241;
pub const WL_SHM_FORMAT_ARGB2101010: u32 = 0x30335241;
pub const WL_SHM_FORMAT_ARGB4444: u32 = 0x32315241;
pub const WL_SHM_FORMAT_ARGB8888: u32 = 0;
pub const WL_SHM_FORMAT_AVUY8888: u32 = 0x59555641;
pub const WL_SHM_FORMAT_AXBXGXRX106106106106: u32 = 0x30314241;
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
pub const WL_SHM_FORMAT_C1: u32 = 0x20203143;
pub const WL_SHM_FORMAT_C2: u32 = 0x20203243;
pub const WL_SHM_FORMAT_C4: u32 = 0x20203443;
pub const WL_SHM_FORMAT_C8: u32 = 0x20203843;
pub const WL_SHM_FORMAT_D1: u32 = 0x20203144;
pub const WL_SHM_FORMAT_D2: u32 = 0x20203244;
pub const WL_SHM_FORMAT_D4: u32 = 0x20203444;
pub const WL_SHM_FORMAT_D8: u32 = 0x20203844;
pub const WL_SHM_FORMAT_GR1616: u32 = 0x32335247;
pub const WL_SHM_FORMAT_GR88: u32 = 0x38385247;
pub const WL_SHM_FORMAT_NV12: u32 = 0x3231564e;
pub const WL_SHM_FORMAT_NV15: u32 = 0x3531564e;
pub const WL_SHM_FORMAT_NV16: u32 = 0x3631564e;
pub const WL_SHM_FORMAT_NV21: u32 = 0x3132564e;
pub const WL_SHM_FORMAT_NV24: u32 = 0x3432564e;
pub const WL_SHM_FORMAT_NV42: u32 = 0x3234564e;
pub const WL_SHM_FORMAT_NV61: u32 = 0x3136564e;
pub const WL_SHM_FORMAT_P010: u32 = 0x30313050;
pub const WL_SHM_FORMAT_P012: u32 = 0x32313050;
pub const WL_SHM_FORMAT_P016: u32 = 0x36313050;
pub const WL_SHM_FORMAT_P030: u32 = 0x30333050;
pub const WL_SHM_FORMAT_P210: u32 = 0x30313250;
pub const WL_SHM_FORMAT_Q401: u32 = 0x31303451;
pub const WL_SHM_FORMAT_Q410: u32 = 0x30313451;
pub const WL_SHM_FORMAT_R1: u32 = 0x20203152;
pub const WL_SHM_FORMAT_R10: u32 = 0x20303152;
pub const WL_SHM_FORMAT_R12: u32 = 0x20323152;
pub const WL_SHM_FORMAT_R16: u32 = 0x20363152;
pub const WL_SHM_FORMAT_R2: u32 = 0x20203252;
pub const WL_SHM_FORMAT_R4: u32 = 0x20203452;
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
pub const WL_SHM_FORMAT_XBGR16161616: u32 = 0x38344258;
pub const WL_SHM_FORMAT_XBGR16161616F: u32 = 0x48344258;
pub const WL_SHM_FORMAT_XBGR2101010: u32 = 0x30334258;
pub const WL_SHM_FORMAT_XBGR4444: u32 = 0x32314258;
pub const WL_SHM_FORMAT_XBGR8888: u32 = 0x34324258;
pub const WL_SHM_FORMAT_XBGR8888_A8: u32 = 0x38414258;
pub const WL_SHM_FORMAT_XRGB1555: u32 = 0x35315258;
pub const WL_SHM_FORMAT_XRGB16161616: u32 = 0x38345258;
pub const WL_SHM_FORMAT_XRGB16161616F: u32 = 0x48345258;
pub const WL_SHM_FORMAT_XRGB2101010: u32 = 0x30335258;
pub const WL_SHM_FORMAT_XRGB4444: u32 = 0x32315258;
pub const WL_SHM_FORMAT_XRGB8888: u32 = 1;
pub const WL_SHM_FORMAT_XRGB8888_A8: u32 = 0x38415258;
pub const WL_SHM_FORMAT_XVUY8888: u32 = 0x59555658;
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
pub const WL_SHM_POOL_CREATE_BUFFER: u32 = 0;
pub const WL_SHM_POOL_CREATE_BUFFER_SINCE_VERSION: u32 = 1;
pub const WL_SHM_POOL_DESTROY: u32 = 1;
pub const WL_SHM_POOL_DESTROY_SINCE_VERSION: u32 = 1;
pub const WL_SHM_POOL_RESIZE: u32 = 2;
pub const WL_SHM_POOL_RESIZE_SINCE_VERSION: u32 = 1;
pub const WL_SHM_RELEASE: u32 = 1;
pub const WL_SHM_RELEASE_SINCE_VERSION: u32 = 2;
pub const WL_SUBCOMPOSITOR_DESTROY: u32 = 0;
pub const WL_SUBCOMPOSITOR_DESTROY_SINCE_VERSION: u32 = 1;
pub const WL_SUBCOMPOSITOR_ERROR_BAD_PARENT: u32 = 1;
pub const WL_SUBCOMPOSITOR_ERROR_BAD_SURFACE: u32 = 0;
pub const WL_SUBCOMPOSITOR_GET_SUBSURFACE: u32 = 1;
pub const WL_SUBCOMPOSITOR_GET_SUBSURFACE_SINCE_VERSION: u32 = 1;
pub const WL_SUBSURFACE_DESTROY: u32 = 0;
pub const WL_SUBSURFACE_DESTROY_SINCE_VERSION: u32 = 1;
pub const WL_SUBSURFACE_ERROR_BAD_SURFACE: u32 = 0;
pub const WL_SUBSURFACE_PLACE_ABOVE: u32 = 2;
pub const WL_SUBSURFACE_PLACE_ABOVE_SINCE_VERSION: u32 = 1;
pub const WL_SUBSURFACE_PLACE_BELOW: u32 = 3;
pub const WL_SUBSURFACE_PLACE_BELOW_SINCE_VERSION: u32 = 1;
pub const WL_SUBSURFACE_SET_DESYNC: u32 = 5;
pub const WL_SUBSURFACE_SET_DESYNC_SINCE_VERSION: u32 = 1;
pub const WL_SUBSURFACE_SET_POSITION: u32 = 1;
pub const WL_SUBSURFACE_SET_POSITION_SINCE_VERSION: u32 = 1;
pub const WL_SUBSURFACE_SET_SYNC: u32 = 4;
pub const WL_SUBSURFACE_SET_SYNC_SINCE_VERSION: u32 = 1;
pub const WL_SURFACE_ATTACH: u32 = 1;
pub const WL_SURFACE_ATTACH_SINCE_VERSION: u32 = 1;
pub const WL_SURFACE_COMMIT: u32 = 6;
pub const WL_SURFACE_COMMIT_SINCE_VERSION: u32 = 1;
pub const WL_SURFACE_DAMAGE: u32 = 2;
pub const WL_SURFACE_DAMAGE_BUFFER: u32 = 9;
pub const WL_SURFACE_DAMAGE_BUFFER_SINCE_VERSION: u32 = 4;
pub const WL_SURFACE_DAMAGE_SINCE_VERSION: u32 = 1;
pub const WL_SURFACE_DESTROY: u32 = 0;
pub const WL_SURFACE_DESTROY_SINCE_VERSION: u32 = 1;
pub const WL_SURFACE_ENTER_SINCE_VERSION: u32 = 1;
pub const WL_SURFACE_ERROR_DEFUNCT_ROLE_OBJECT: u32 = 4;
pub const WL_SURFACE_ERROR_INVALID_OFFSET: u32 = 3;
pub const WL_SURFACE_ERROR_INVALID_SCALE: u32 = 0;
pub const WL_SURFACE_ERROR_INVALID_SIZE: u32 = 2;
pub const WL_SURFACE_ERROR_INVALID_TRANSFORM: u32 = 1;
pub const WL_SURFACE_FRAME: u32 = 3;
pub const WL_SURFACE_FRAME_SINCE_VERSION: u32 = 1;
pub const WL_SURFACE_LEAVE_SINCE_VERSION: u32 = 1;
pub const WL_SURFACE_OFFSET: u32 = 10;
pub const WL_SURFACE_OFFSET_SINCE_VERSION: u32 = 5;
pub const WL_SURFACE_PREFERRED_BUFFER_SCALE_SINCE_VERSION: u32 = 6;
pub const WL_SURFACE_PREFERRED_BUFFER_TRANSFORM_SINCE_VERSION: u32 = 6;
pub const WL_SURFACE_SET_BUFFER_SCALE: u32 = 8;
pub const WL_SURFACE_SET_BUFFER_SCALE_SINCE_VERSION: u32 = 3;
pub const WL_SURFACE_SET_BUFFER_TRANSFORM: u32 = 7;
pub const WL_SURFACE_SET_BUFFER_TRANSFORM_SINCE_VERSION: u32 = 2;
pub const WL_SURFACE_SET_INPUT_REGION: u32 = 5;
pub const WL_SURFACE_SET_INPUT_REGION_SINCE_VERSION: u32 = 1;
pub const WL_SURFACE_SET_OPAQUE_REGION: u32 = 4;
pub const WL_SURFACE_SET_OPAQUE_REGION_SINCE_VERSION: u32 = 1;
pub const WL_TOUCH_CANCEL_SINCE_VERSION: u32 = 1;
pub const WL_TOUCH_DOWN_SINCE_VERSION: u32 = 1;
pub const WL_TOUCH_FRAME_SINCE_VERSION: u32 = 1;
pub const WL_TOUCH_MOTION_SINCE_VERSION: u32 = 1;
pub const WL_TOUCH_ORIENTATION_SINCE_VERSION: u32 = 6;
pub const WL_TOUCH_RELEASE: u32 = 0;
pub const WL_TOUCH_RELEASE_SINCE_VERSION: u32 = 3;
pub const WL_TOUCH_SHAPE_SINCE_VERSION: u32 = 6;
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
    pub static wl_fixes_interface: wl_interface;
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
pub unsafe extern "C" fn wl_buffer_add_listener(
    wl_buffer: *mut wl_buffer,
    listener: *const wl_buffer_listener,
    data: *mut c_void,
) -> c_int {
    unsafe {
        wl_proxy_add_listener(
            wl_buffer.cast(),
            listener.cast_mut().cast(),
            data,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_buffer_destroy(
    wl_buffer: *mut wl_buffer,
) {
    unsafe {
        wl_proxy_marshal_flags(
            wl_buffer.cast(),
            WL_BUFFER_DESTROY,
            null(),
            wl_proxy_get_version(wl_buffer.cast()),
            WL_MARSHAL_FLAG_DESTROY,
        );
    }
}

#[inline]
pub unsafe extern "C" fn wl_buffer_get_user_data(
    wl_buffer: *mut wl_buffer,
) -> *mut c_void {
    unsafe { wl_proxy_get_user_data(wl_buffer.cast()) }
}

#[inline]
pub unsafe extern "C" fn wl_buffer_get_version(
    wl_buffer: *mut wl_buffer,
) -> u32 {
    unsafe { wl_proxy_get_version(wl_buffer.cast()) }
}

#[inline]
pub unsafe extern "C" fn wl_buffer_set_user_data(
    wl_buffer: *mut wl_buffer,
    user_data: *mut c_void,
) {
    unsafe {
        wl_proxy_set_user_data(
            wl_buffer.cast(),
            user_data,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_callback_add_listener(
    wl_callback: *mut wl_callback,
    listener: *const wl_callback_listener,
    data: *mut c_void,
) -> c_int {
    unsafe {
        wl_proxy_add_listener(
            wl_callback.cast(),
            listener.cast_mut().cast(),
            data,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_callback_destroy(
    wl_callback: *mut wl_callback,
) {
    unsafe { wl_proxy_destroy(wl_callback.cast()) }
}

#[inline]
pub unsafe extern "C" fn wl_callback_get_user_data(
    wl_callback: *mut wl_callback,
) -> *mut c_void {
    unsafe { wl_proxy_get_user_data(wl_callback.cast()) }
}

#[inline]
pub unsafe extern "C" fn wl_callback_get_version(
    wl_callback: *mut wl_callback,
) -> u32 {
    unsafe { wl_proxy_get_version(wl_callback.cast()) }
}

#[inline]
pub unsafe extern "C" fn wl_callback_set_user_data(
    wl_callback: *mut wl_callback,
    user_data: *mut c_void,
) {
    unsafe {
        wl_proxy_set_user_data(
            wl_callback.cast(),
            user_data,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_compositor_create_region(
    wl_compositor: *mut wl_compositor,
) -> *mut wl_region {
    unsafe {
        wl_proxy_marshal_flags(
            wl_compositor.cast(),
            WL_COMPOSITOR_CREATE_REGION,
            &wl_region_interface,
            wl_proxy_get_version(wl_compositor.cast()),
            0,
            null_mut::<c_void>(),
        ).cast()
    }
}

#[inline]
pub unsafe extern "C" fn wl_compositor_create_surface(
    wl_compositor: *mut wl_compositor,
) -> *mut wl_surface {
    unsafe {
        wl_proxy_marshal_flags(
            wl_compositor.cast(),
            WL_COMPOSITOR_CREATE_SURFACE,
            &wl_surface_interface,
            wl_proxy_get_version(wl_compositor.cast()),
            0,
            null_mut::<c_void>(),
        ).cast()
    }
}

#[inline]
pub unsafe extern "C" fn wl_compositor_destroy(
    wl_compositor: *mut wl_compositor,
) {
    unsafe { wl_proxy_destroy(wl_compositor.cast()) }
}

#[inline]
pub unsafe extern "C" fn wl_compositor_get_user_data(
    wl_compositor: *mut wl_compositor,
) -> *mut c_void {
    unsafe { wl_proxy_get_user_data(wl_compositor.cast()) }
}

#[inline]
pub unsafe extern "C" fn wl_compositor_get_version(
    wl_compositor: *mut wl_compositor,
) -> u32 {
    unsafe { wl_proxy_get_version(wl_compositor.cast()) }
}

#[inline]
pub unsafe extern "C" fn wl_compositor_set_user_data(
    wl_compositor: *mut wl_compositor,
    user_data: *mut c_void,
) {
    unsafe {
        wl_proxy_set_user_data(
            wl_compositor.cast(),
            user_data,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_data_device_add_listener(
    wl_data_device: *mut wl_data_device,
    listener: *const wl_data_device_listener,
    data: *mut c_void,
) -> c_int {
    unsafe {
        wl_proxy_add_listener(
            wl_data_device.cast(),
            listener.cast_mut().cast(),
            data,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_data_device_destroy(
    wl_data_device: *mut wl_data_device,
) {
    unsafe { wl_proxy_destroy(wl_data_device.cast()) }
}

#[inline]
pub unsafe extern "C" fn wl_data_device_get_user_data(
    wl_data_device: *mut wl_data_device,
) -> *mut c_void {
    unsafe { wl_proxy_get_user_data(wl_data_device.cast()) }
}

#[inline]
pub unsafe extern "C" fn wl_data_device_get_version(
    wl_data_device: *mut wl_data_device,
) -> u32 {
    unsafe { wl_proxy_get_version(wl_data_device.cast()) }
}

#[inline]
pub unsafe extern "C" fn wl_data_device_manager_create_data_source(
    wl_data_device_manager: *mut wl_data_device_manager,
) -> *mut wl_data_source {
    unsafe {
        wl_proxy_marshal_flags(
            wl_data_device_manager.cast(),
            WL_DATA_DEVICE_MANAGER_CREATE_DATA_SOURCE,
            &wl_data_source_interface,
            wl_proxy_get_version(wl_data_device_manager.cast()),
            0,
            null_mut::<c_void>(),
        ).cast()
    }
}

#[inline]
pub unsafe extern "C" fn wl_data_device_manager_destroy(
    wl_data_device_manager: *mut wl_data_device_manager,
) {
    unsafe { wl_proxy_destroy(wl_data_device_manager.cast()) }
}

#[inline]
pub unsafe extern "C" fn wl_data_device_manager_get_data_device(
    wl_data_device_manager: *mut wl_data_device_manager,
    seat: *mut wl_seat,
) -> *mut wl_data_device {
    unsafe {
        wl_proxy_marshal_flags(
            wl_data_device_manager.cast(),
            WL_DATA_DEVICE_MANAGER_GET_DATA_DEVICE,
            &wl_data_device_interface,
            wl_proxy_get_version(wl_data_device_manager.cast()),
            0,
            null_mut::<c_void>(),
            seat,
        ).cast()
    }
}

#[inline]
pub unsafe extern "C" fn wl_data_device_manager_get_user_data(
    wl_data_device_manager: *mut wl_data_device_manager,
) -> *mut c_void {
    unsafe { wl_proxy_get_user_data(wl_data_device_manager.cast()) }
}

#[inline]
pub unsafe extern "C" fn wl_data_device_manager_get_version(
    wl_data_device_manager: *mut wl_data_device_manager,
) -> u32 {
    unsafe { wl_proxy_get_version(wl_data_device_manager.cast()) }
}

#[inline]
pub unsafe extern "C" fn wl_data_device_manager_set_user_data(
    wl_data_device_manager: *mut wl_data_device_manager,
    user_data: *mut c_void,
) {
    unsafe {
        wl_proxy_set_user_data(
            wl_data_device_manager.cast(),
            user_data,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_data_device_release(
    wl_data_device: *mut wl_data_device,
) {
    unsafe {
        wl_proxy_marshal_flags(
            wl_data_device.cast(),
            WL_DATA_DEVICE_RELEASE,
            null(),
            wl_proxy_get_version(wl_data_device.cast()),
            WL_MARSHAL_FLAG_DESTROY,
        );
    }
}

#[inline]
pub unsafe extern "C" fn wl_data_device_set_selection(
    wl_data_device: *mut wl_data_device,
    source: *mut wl_data_source,
    serial: u32,
) {
    unsafe {
        wl_proxy_marshal_flags(
            wl_data_device.cast(),
            WL_DATA_DEVICE_SET_SELECTION,
            null(),
            wl_proxy_get_version(wl_data_device.cast()),
            0,
            source,
            serial,
        );
    }
}

#[inline]
pub unsafe extern "C" fn wl_data_device_set_user_data(
    wl_data_device: *mut wl_data_device,
    user_data: *mut c_void,
) {
    unsafe {
        wl_proxy_set_user_data(
            wl_data_device.cast(),
            user_data,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_data_device_start_drag(
    wl_data_device: *mut wl_data_device,
    source: *mut wl_data_source,
    origin: *mut wl_surface,
    icon: *mut wl_surface,
    serial: u32,
) {
    unsafe {
        wl_proxy_marshal_flags(
            wl_data_device.cast(),
            WL_DATA_DEVICE_START_DRAG,
            null(),
            wl_proxy_get_version(wl_data_device.cast()),
            0,
            source,
            origin,
            icon,
            serial,
        );
    }
}

#[inline]
pub unsafe extern "C" fn wl_data_offer_accept(
    wl_data_offer: *mut wl_data_offer,
    serial: u32,
    mime_type: *const c_char,
) {
    unsafe {
        wl_proxy_marshal_flags(
            wl_data_offer.cast(),
            WL_DATA_OFFER_ACCEPT,
            null(),
            wl_proxy_get_version(wl_data_offer.cast()),
            0,
            serial,
            mime_type,
        );
    }
}

#[inline]
pub unsafe extern "C" fn wl_data_offer_add_listener(
    wl_data_offer: *mut wl_data_offer,
    listener: *const wl_data_offer_listener,
    data: *mut c_void,
) -> c_int {
    unsafe {
        wl_proxy_add_listener(
            wl_data_offer.cast(),
            listener.cast_mut().cast(),
            data,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_data_offer_destroy(
    wl_data_offer: *mut wl_data_offer,
) {
    unsafe {
        wl_proxy_marshal_flags(
            wl_data_offer.cast(),
            WL_DATA_OFFER_DESTROY,
            null(),
            wl_proxy_get_version(wl_data_offer.cast()),
            WL_MARSHAL_FLAG_DESTROY,
        );
    }
}

#[inline]
pub unsafe extern "C" fn wl_data_offer_finish(
    wl_data_offer: *mut wl_data_offer,
) {
    unsafe {
        wl_proxy_marshal_flags(
            wl_data_offer.cast(),
            WL_DATA_OFFER_FINISH,
            null(),
            wl_proxy_get_version(wl_data_offer.cast()),
            0,
        );
    }
}

#[inline]
pub unsafe extern "C" fn wl_data_offer_get_user_data(
    wl_data_offer: *mut wl_data_offer,
) -> *mut c_void {
    unsafe { wl_proxy_get_user_data(wl_data_offer.cast()) }
}

#[inline]
pub unsafe extern "C" fn wl_data_offer_get_version(
    wl_data_offer: *mut wl_data_offer,
) -> u32 {
    unsafe { wl_proxy_get_version(wl_data_offer.cast()) }
}

#[inline]
pub unsafe extern "C" fn wl_data_offer_receive(
    wl_data_offer: *mut wl_data_offer,
    mime_type: *const c_char,
    fd: i32,
) {
    unsafe {
        wl_proxy_marshal_flags(
            wl_data_offer.cast(),
            WL_DATA_OFFER_RECEIVE,
            null(),
            wl_proxy_get_version(wl_data_offer.cast()),
            0,
            mime_type,
            fd,
        );
    }
}

#[inline]
pub unsafe extern "C" fn wl_data_offer_set_actions(
    wl_data_offer: *mut wl_data_offer,
    dnd_actions: u32,
    preferred_action: u32,
) {
    unsafe {
        wl_proxy_marshal_flags(
            wl_data_offer.cast(),
            WL_DATA_OFFER_SET_ACTIONS,
            null(),
            wl_proxy_get_version(wl_data_offer.cast()),
            0,
            dnd_actions,
            preferred_action,
        );
    }
}

#[inline]
pub unsafe extern "C" fn wl_data_offer_set_user_data(
    wl_data_offer: *mut wl_data_offer,
    user_data: *mut c_void,
) {
    unsafe {
        wl_proxy_set_user_data(
            wl_data_offer.cast(),
            user_data,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_data_source_add_listener(
    wl_data_source: *mut wl_data_source,
    listener: *const wl_data_source_listener,
    data: *mut c_void,
) -> c_int {
    unsafe {
        wl_proxy_add_listener(
            wl_data_source.cast(),
            listener.cast_mut().cast(),
            data,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_data_source_destroy(
    wl_data_source: *mut wl_data_source,
) {
    unsafe {
        wl_proxy_marshal_flags(
            wl_data_source.cast(),
            WL_DATA_SOURCE_DESTROY,
            null(),
            wl_proxy_get_version(wl_data_source.cast()),
            WL_MARSHAL_FLAG_DESTROY,
        );
    }
}

#[inline]
pub unsafe extern "C" fn wl_data_source_get_user_data(
    wl_data_source: *mut wl_data_source,
) -> *mut c_void {
    unsafe { wl_proxy_get_user_data(wl_data_source.cast()) }
}

#[inline]
pub unsafe extern "C" fn wl_data_source_get_version(
    wl_data_source: *mut wl_data_source,
) -> u32 {
    unsafe { wl_proxy_get_version(wl_data_source.cast()) }
}

#[inline]
pub unsafe extern "C" fn wl_data_source_offer(
    wl_data_source: *mut wl_data_source,
    mime_type: *const c_char,
) {
    unsafe {
        wl_proxy_marshal_flags(
            wl_data_source.cast(),
            WL_DATA_SOURCE_OFFER,
            null(),
            wl_proxy_get_version(wl_data_source.cast()),
            0,
            mime_type,
        );
    }
}

#[inline]
pub unsafe extern "C" fn wl_data_source_set_actions(
    wl_data_source: *mut wl_data_source,
    dnd_actions: u32,
) {
    unsafe {
        wl_proxy_marshal_flags(
            wl_data_source.cast(),
            WL_DATA_SOURCE_SET_ACTIONS,
            null(),
            wl_proxy_get_version(wl_data_source.cast()),
            0,
            dnd_actions,
        );
    }
}

#[inline]
pub unsafe extern "C" fn wl_data_source_set_user_data(
    wl_data_source: *mut wl_data_source,
    user_data: *mut c_void,
) {
    unsafe {
        wl_proxy_set_user_data(
            wl_data_source.cast(),
            user_data,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_display_add_listener(
    wl_display: *mut wl_display,
    listener: *const wl_display_listener,
    data: *mut c_void,
) -> c_int {
    unsafe {
        wl_proxy_add_listener(
            wl_display.cast(),
            listener.cast_mut().cast(),
            data,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_display_get_registry(
    wl_display: *mut wl_display,
) -> *mut wl_registry {
    unsafe {
        wl_proxy_marshal_flags(
            wl_display.cast(),
            WL_DISPLAY_GET_REGISTRY,
            &wl_registry_interface,
            wl_proxy_get_version(wl_display.cast()),
            0,
            null_mut::<c_void>(),
        ).cast()
    }
}

#[inline]
pub unsafe extern "C" fn wl_display_get_user_data(
    wl_display: *mut wl_display,
) -> *mut c_void {
    unsafe { wl_proxy_get_user_data(wl_display.cast()) }
}

#[inline]
pub unsafe extern "C" fn wl_display_get_version(
    wl_display: *mut wl_display,
) -> u32 {
    unsafe { wl_proxy_get_version(wl_display.cast()) }
}

#[inline]
pub unsafe extern "C" fn wl_display_set_user_data(
    wl_display: *mut wl_display,
    user_data: *mut c_void,
) {
    unsafe {
        wl_proxy_set_user_data(
            wl_display.cast(),
            user_data,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_display_sync(
    wl_display: *mut wl_display,
) -> *mut wl_callback {
    unsafe {
        wl_proxy_marshal_flags(
            wl_display.cast(),
            WL_DISPLAY_SYNC,
            &wl_callback_interface,
            wl_proxy_get_version(wl_display.cast()),
            0,
            null_mut::<c_void>(),
        ).cast()
    }
}

#[inline]
pub unsafe extern "C" fn wl_fixes_destroy(
    wl_fixes: *mut wl_fixes,
) {
    unsafe {
        wl_proxy_marshal_flags(
            wl_fixes.cast(),
            WL_FIXES_DESTROY,
            null(),
            wl_proxy_get_version(wl_fixes.cast()),
            WL_MARSHAL_FLAG_DESTROY,
        );
    }
}

#[inline]
pub unsafe extern "C" fn wl_fixes_destroy_registry(
    wl_fixes: *mut wl_fixes,
    registry: *mut wl_registry,
) {
    unsafe {
        wl_proxy_marshal_flags(
            wl_fixes.cast(),
            WL_FIXES_DESTROY_REGISTRY,
            null(),
            wl_proxy_get_version(wl_fixes.cast()),
            0,
            registry,
        );
    }
}

#[inline]
pub unsafe extern "C" fn wl_fixes_get_user_data(
    wl_fixes: *mut wl_fixes,
) -> *mut c_void {
    unsafe { wl_proxy_get_user_data(wl_fixes.cast()) }
}

#[inline]
pub unsafe extern "C" fn wl_fixes_get_version(
    wl_fixes: *mut wl_fixes,
) -> u32 {
    unsafe { wl_proxy_get_version(wl_fixes.cast()) }
}

#[inline]
pub unsafe extern "C" fn wl_fixes_set_user_data(
    wl_fixes: *mut wl_fixes,
    user_data: *mut c_void,
) {
    unsafe {
        wl_proxy_set_user_data(
            wl_fixes.cast(),
            user_data,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_keyboard_add_listener(
    wl_keyboard: *mut wl_keyboard,
    listener: *const wl_keyboard_listener,
    data: *mut c_void,
) -> c_int {
    unsafe {
        wl_proxy_add_listener(
            wl_keyboard.cast(),
            listener.cast_mut().cast(),
            data,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_keyboard_destroy(
    wl_keyboard: *mut wl_keyboard,
) {
    unsafe { wl_proxy_destroy(wl_keyboard.cast()) }
}

#[inline]
pub unsafe extern "C" fn wl_keyboard_get_user_data(
    wl_keyboard: *mut wl_keyboard,
) -> *mut c_void {
    unsafe { wl_proxy_get_user_data(wl_keyboard.cast()) }
}

#[inline]
pub unsafe extern "C" fn wl_keyboard_get_version(
    wl_keyboard: *mut wl_keyboard,
) -> u32 {
    unsafe { wl_proxy_get_version(wl_keyboard.cast()) }
}

#[inline]
pub unsafe extern "C" fn wl_keyboard_release(
    wl_keyboard: *mut wl_keyboard,
) {
    unsafe {
        wl_proxy_marshal_flags(
            wl_keyboard.cast(),
            WL_KEYBOARD_RELEASE,
            null(),
            wl_proxy_get_version(wl_keyboard.cast()),
            WL_MARSHAL_FLAG_DESTROY,
        );
    }
}

#[inline]
pub unsafe extern "C" fn wl_keyboard_set_user_data(
    wl_keyboard: *mut wl_keyboard,
    user_data: *mut c_void,
) {
    unsafe {
        wl_proxy_set_user_data(
            wl_keyboard.cast(),
            user_data,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_output_add_listener(
    wl_output: *mut wl_output,
    listener: *const wl_output_listener,
    data: *mut c_void,
) -> c_int {
    unsafe {
        wl_proxy_add_listener(
            wl_output.cast(),
            listener.cast_mut().cast(),
            data,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_output_destroy(
    wl_output: *mut wl_output,
) {
    unsafe { wl_proxy_destroy(wl_output.cast()) }
}

#[inline]
pub unsafe extern "C" fn wl_output_get_user_data(
    wl_output: *mut wl_output,
) -> *mut c_void {
    unsafe { wl_proxy_get_user_data(wl_output.cast()) }
}

#[inline]
pub unsafe extern "C" fn wl_output_get_version(
    wl_output: *mut wl_output,
) -> u32 {
    unsafe { wl_proxy_get_version(wl_output.cast()) }
}

#[inline]
pub unsafe extern "C" fn wl_output_release(
    wl_output: *mut wl_output,
) {
    unsafe {
        wl_proxy_marshal_flags(
            wl_output.cast(),
            WL_OUTPUT_RELEASE,
            null(),
            wl_proxy_get_version(wl_output.cast()),
            WL_MARSHAL_FLAG_DESTROY,
        );
    }
}

#[inline]
pub unsafe extern "C" fn wl_output_set_user_data(
    wl_output: *mut wl_output,
    user_data: *mut c_void,
) {
    unsafe {
        wl_proxy_set_user_data(
            wl_output.cast(),
            user_data,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_pointer_add_listener(
    wl_pointer: *mut wl_pointer,
    listener: *const wl_pointer_listener,
    data: *mut c_void,
) -> c_int {
    unsafe {
        wl_proxy_add_listener(
            wl_pointer.cast(),
            listener.cast_mut().cast(),
            data,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_pointer_destroy(
    wl_pointer: *mut wl_pointer,
) {
    unsafe { wl_proxy_destroy(wl_pointer.cast()) }
}

#[inline]
pub unsafe extern "C" fn wl_pointer_get_user_data(
    wl_pointer: *mut wl_pointer,
) -> *mut c_void {
    unsafe { wl_proxy_get_user_data(wl_pointer.cast()) }
}

#[inline]
pub unsafe extern "C" fn wl_pointer_get_version(
    wl_pointer: *mut wl_pointer,
) -> u32 {
    unsafe { wl_proxy_get_version(wl_pointer.cast()) }
}

#[inline]
pub unsafe extern "C" fn wl_pointer_release(
    wl_pointer: *mut wl_pointer,
) {
    unsafe {
        wl_proxy_marshal_flags(
            wl_pointer.cast(),
            WL_POINTER_RELEASE,
            null(),
            wl_proxy_get_version(wl_pointer.cast()),
            WL_MARSHAL_FLAG_DESTROY,
        );
    }
}

#[inline]
pub unsafe extern "C" fn wl_pointer_set_cursor(
    wl_pointer: *mut wl_pointer,
    serial: u32,
    surface: *mut wl_surface,
    hotspot_x: i32,
    hotspot_y: i32,
) {
    unsafe {
        wl_proxy_marshal_flags(
            wl_pointer.cast(),
            WL_POINTER_SET_CURSOR,
            null(),
            wl_proxy_get_version(wl_pointer.cast()),
            0,
            serial,
            surface,
            hotspot_x,
            hotspot_y,
        );
    }
}

#[inline]
pub unsafe extern "C" fn wl_pointer_set_user_data(
    wl_pointer: *mut wl_pointer,
    user_data: *mut c_void,
) {
    unsafe {
        wl_proxy_set_user_data(
            wl_pointer.cast(),
            user_data,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_region_add(
    wl_region: *mut wl_region,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
) {
    unsafe {
        wl_proxy_marshal_flags(
            wl_region.cast(),
            WL_REGION_ADD,
            null(),
            wl_proxy_get_version(wl_region.cast()),
            0,
            x,
            y,
            width,
            height,
        );
    }
}

#[inline]
pub unsafe extern "C" fn wl_region_destroy(
    wl_region: *mut wl_region,
) {
    unsafe {
        wl_proxy_marshal_flags(
            wl_region.cast(),
            WL_REGION_DESTROY,
            null(),
            wl_proxy_get_version(wl_region.cast()),
            WL_MARSHAL_FLAG_DESTROY,
        );
    }
}

#[inline]
pub unsafe extern "C" fn wl_region_get_user_data(
    wl_region: *mut wl_region,
) -> *mut c_void {
    unsafe { wl_proxy_get_user_data(wl_region.cast()) }
}

#[inline]
pub unsafe extern "C" fn wl_region_get_version(
    wl_region: *mut wl_region,
) -> u32 {
    unsafe { wl_proxy_get_version(wl_region.cast()) }
}

#[inline]
pub unsafe extern "C" fn wl_region_set_user_data(
    wl_region: *mut wl_region,
    user_data: *mut c_void,
) {
    unsafe {
        wl_proxy_set_user_data(
            wl_region.cast(),
            user_data,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_region_subtract(
    wl_region: *mut wl_region,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
) {
    unsafe {
        wl_proxy_marshal_flags(
            wl_region.cast(),
            WL_REGION_SUBTRACT,
            null(),
            wl_proxy_get_version(wl_region.cast()),
            0,
            x,
            y,
            width,
            height,
        );
    }
}

#[inline]
pub unsafe extern "C" fn wl_registry_add_listener(
    wl_registry: *mut wl_registry,
    listener: *const wl_registry_listener,
    data: *mut c_void,
) -> c_int {
    unsafe {
        wl_proxy_add_listener(
            wl_registry.cast(),
            listener.cast_mut().cast(),
            data,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_registry_bind(
    wl_registry: *mut wl_registry,
    name: u32,
    interface: *const wl_interface,
    version: u32,
) -> *mut c_void {
    unsafe {
        wl_proxy_marshal_flags(
            wl_registry.cast(),
            WL_REGISTRY_BIND,
            interface,
            version,
            0,
            name,
            (*interface).name,
            version,
            null_mut::<c_void>(),
        ).cast()
    }
}

#[inline]
pub unsafe extern "C" fn wl_registry_destroy(
    wl_registry: *mut wl_registry,
) {
    unsafe { wl_proxy_destroy(wl_registry.cast()) }
}

#[inline]
pub unsafe extern "C" fn wl_registry_get_user_data(
    wl_registry: *mut wl_registry,
) -> *mut c_void {
    unsafe { wl_proxy_get_user_data(wl_registry.cast()) }
}

#[inline]
pub unsafe extern "C" fn wl_registry_get_version(
    wl_registry: *mut wl_registry,
) -> u32 {
    unsafe { wl_proxy_get_version(wl_registry.cast()) }
}

#[inline]
pub unsafe extern "C" fn wl_registry_set_user_data(
    wl_registry: *mut wl_registry,
    user_data: *mut c_void,
) {
    unsafe {
        wl_proxy_set_user_data(
            wl_registry.cast(),
            user_data,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_seat_add_listener(
    wl_seat: *mut wl_seat,
    listener: *const wl_seat_listener,
    data: *mut c_void,
) -> c_int {
    unsafe {
        wl_proxy_add_listener(
            wl_seat.cast(),
            listener.cast_mut().cast(),
            data,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_seat_destroy(
    wl_seat: *mut wl_seat,
) {
    unsafe { wl_proxy_destroy(wl_seat.cast()) }
}

#[inline]
pub unsafe extern "C" fn wl_seat_get_keyboard(
    wl_seat: *mut wl_seat,
) -> *mut wl_keyboard {
    unsafe {
        wl_proxy_marshal_flags(
            wl_seat.cast(),
            WL_SEAT_GET_KEYBOARD,
            &wl_keyboard_interface,
            wl_proxy_get_version(wl_seat.cast()),
            0,
            null_mut::<c_void>(),
        ).cast()
    }
}

#[inline]
pub unsafe extern "C" fn wl_seat_get_pointer(
    wl_seat: *mut wl_seat,
) -> *mut wl_pointer {
    unsafe {
        wl_proxy_marshal_flags(
            wl_seat.cast(),
            WL_SEAT_GET_POINTER,
            &wl_pointer_interface,
            wl_proxy_get_version(wl_seat.cast()),
            0,
            null_mut::<c_void>(),
        ).cast()
    }
}

#[inline]
pub unsafe extern "C" fn wl_seat_get_touch(
    wl_seat: *mut wl_seat,
) -> *mut wl_touch {
    unsafe {
        wl_proxy_marshal_flags(
            wl_seat.cast(),
            WL_SEAT_GET_TOUCH,
            &wl_touch_interface,
            wl_proxy_get_version(wl_seat.cast()),
            0,
            null_mut::<c_void>(),
        ).cast()
    }
}

#[inline]
pub unsafe extern "C" fn wl_seat_get_user_data(
    wl_seat: *mut wl_seat,
) -> *mut c_void {
    unsafe { wl_proxy_get_user_data(wl_seat.cast()) }
}

#[inline]
pub unsafe extern "C" fn wl_seat_get_version(
    wl_seat: *mut wl_seat,
) -> u32 {
    unsafe { wl_proxy_get_version(wl_seat.cast()) }
}

#[inline]
pub unsafe extern "C" fn wl_seat_release(
    wl_seat: *mut wl_seat,
) {
    unsafe {
        wl_proxy_marshal_flags(
            wl_seat.cast(),
            WL_SEAT_RELEASE,
            null(),
            wl_proxy_get_version(wl_seat.cast()),
            WL_MARSHAL_FLAG_DESTROY,
        );
    }
}

#[inline]
pub unsafe extern "C" fn wl_seat_set_user_data(
    wl_seat: *mut wl_seat,
    user_data: *mut c_void,
) {
    unsafe {
        wl_proxy_set_user_data(
            wl_seat.cast(),
            user_data,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_shell_destroy(
    wl_shell: *mut wl_shell,
) {
    unsafe { wl_proxy_destroy(wl_shell.cast()) }
}

#[inline]
pub unsafe extern "C" fn wl_shell_get_shell_surface(
    wl_shell: *mut wl_shell,
    surface: *mut wl_surface,
) -> *mut wl_shell_surface {
    unsafe {
        wl_proxy_marshal_flags(
            wl_shell.cast(),
            WL_SHELL_GET_SHELL_SURFACE,
            &wl_shell_surface_interface,
            wl_proxy_get_version(wl_shell.cast()),
            0,
            null_mut::<c_void>(),
            surface,
        ).cast()
    }
}

#[inline]
pub unsafe extern "C" fn wl_shell_get_user_data(
    wl_shell: *mut wl_shell,
) -> *mut c_void {
    unsafe { wl_proxy_get_user_data(wl_shell.cast()) }
}

#[inline]
pub unsafe extern "C" fn wl_shell_get_version(
    wl_shell: *mut wl_shell,
) -> u32 {
    unsafe { wl_proxy_get_version(wl_shell.cast()) }
}

#[inline]
pub unsafe extern "C" fn wl_shell_set_user_data(
    wl_shell: *mut wl_shell,
    user_data: *mut c_void,
) {
    unsafe {
        wl_proxy_set_user_data(
            wl_shell.cast(),
            user_data,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_shell_surface_add_listener(
    wl_shell_surface: *mut wl_shell_surface,
    listener: *const wl_shell_surface_listener,
    data: *mut c_void,
) -> c_int {
    unsafe {
        wl_proxy_add_listener(
            wl_shell_surface.cast(),
            listener.cast_mut().cast(),
            data,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_shell_surface_destroy(
    wl_shell_surface: *mut wl_shell_surface,
) {
    unsafe { wl_proxy_destroy(wl_shell_surface.cast()) }
}

#[inline]
pub unsafe extern "C" fn wl_shell_surface_get_user_data(
    wl_shell_surface: *mut wl_shell_surface,
) -> *mut c_void {
    unsafe { wl_proxy_get_user_data(wl_shell_surface.cast()) }
}

#[inline]
pub unsafe extern "C" fn wl_shell_surface_get_version(
    wl_shell_surface: *mut wl_shell_surface,
) -> u32 {
    unsafe { wl_proxy_get_version(wl_shell_surface.cast()) }
}

#[inline]
pub unsafe extern "C" fn wl_shell_surface_move(
    wl_shell_surface: *mut wl_shell_surface,
    seat: *mut wl_seat,
    serial: u32,
) {
    unsafe {
        wl_proxy_marshal_flags(
            wl_shell_surface.cast(),
            WL_SHELL_SURFACE_MOVE,
            null(),
            wl_proxy_get_version(wl_shell_surface.cast()),
            0,
            seat,
            serial,
        );
    }
}

#[inline]
pub unsafe extern "C" fn wl_shell_surface_pong(
    wl_shell_surface: *mut wl_shell_surface,
    serial: u32,
) {
    unsafe {
        wl_proxy_marshal_flags(
            wl_shell_surface.cast(),
            WL_SHELL_SURFACE_PONG,
            null(),
            wl_proxy_get_version(wl_shell_surface.cast()),
            0,
            serial,
        );
    }
}

#[inline]
pub unsafe extern "C" fn wl_shell_surface_resize(
    wl_shell_surface: *mut wl_shell_surface,
    seat: *mut wl_seat,
    serial: u32,
    edges: u32,
) {
    unsafe {
        wl_proxy_marshal_flags(
            wl_shell_surface.cast(),
            WL_SHELL_SURFACE_RESIZE,
            null(),
            wl_proxy_get_version(wl_shell_surface.cast()),
            0,
            seat,
            serial,
            edges,
        );
    }
}

#[inline]
pub unsafe extern "C" fn wl_shell_surface_set_class(
    wl_shell_surface: *mut wl_shell_surface,
    class_: *const c_char,
) {
    unsafe {
        wl_proxy_marshal_flags(
            wl_shell_surface.cast(),
            WL_SHELL_SURFACE_SET_CLASS,
            null(),
            wl_proxy_get_version(wl_shell_surface.cast()),
            0,
            class_,
        );
    }
}

#[inline]
pub unsafe extern "C" fn wl_shell_surface_set_fullscreen(
    wl_shell_surface: *mut wl_shell_surface,
    method: u32,
    framerate: u32,
    output: *mut wl_output,
) {
    unsafe {
        wl_proxy_marshal_flags(
            wl_shell_surface.cast(),
            WL_SHELL_SURFACE_SET_FULLSCREEN,
            null(),
            wl_proxy_get_version(wl_shell_surface.cast()),
            0,
            method,
            framerate,
            output,
        );
    }
}

#[inline]
pub unsafe extern "C" fn wl_shell_surface_set_maximized(
    wl_shell_surface: *mut wl_shell_surface,
    output: *mut wl_output,
) {
    unsafe {
        wl_proxy_marshal_flags(
            wl_shell_surface.cast(),
            WL_SHELL_SURFACE_SET_MAXIMIZED,
            null(),
            wl_proxy_get_version(wl_shell_surface.cast()),
            0,
            output,
        );
    }
}

#[inline]
pub unsafe extern "C" fn wl_shell_surface_set_popup(
    wl_shell_surface: *mut wl_shell_surface,
    seat: *mut wl_seat,
    serial: u32,
    parent: *mut wl_surface,
    x: i32,
    y: i32,
    flags: u32,
) {
    unsafe {
        wl_proxy_marshal_flags(
            wl_shell_surface.cast(),
            WL_SHELL_SURFACE_SET_POPUP,
            null(),
            wl_proxy_get_version(wl_shell_surface.cast()),
            0,
            seat,
            serial,
            parent,
            x,
            y,
            flags,
        );
    }
}

#[inline]
pub unsafe extern "C" fn wl_shell_surface_set_title(
    wl_shell_surface: *mut wl_shell_surface,
    title: *const c_char,
) {
    unsafe {
        wl_proxy_marshal_flags(
            wl_shell_surface.cast(),
            WL_SHELL_SURFACE_SET_TITLE,
            null(),
            wl_proxy_get_version(wl_shell_surface.cast()),
            0,
            title,
        );
    }
}

#[inline]
pub unsafe extern "C" fn wl_shell_surface_set_toplevel(
    wl_shell_surface: *mut wl_shell_surface,
) {
    unsafe {
        wl_proxy_marshal_flags(
            wl_shell_surface.cast(),
            WL_SHELL_SURFACE_SET_TOPLEVEL,
            null(),
            wl_proxy_get_version(wl_shell_surface.cast()),
            0,
        );
    }
}

#[inline]
pub unsafe extern "C" fn wl_shell_surface_set_transient(
    wl_shell_surface: *mut wl_shell_surface,
    parent: *mut wl_surface,
    x: i32,
    y: i32,
    flags: u32,
) {
    unsafe {
        wl_proxy_marshal_flags(
            wl_shell_surface.cast(),
            WL_SHELL_SURFACE_SET_TRANSIENT,
            null(),
            wl_proxy_get_version(wl_shell_surface.cast()),
            0,
            parent,
            x,
            y,
            flags,
        );
    }
}

#[inline]
pub unsafe extern "C" fn wl_shell_surface_set_user_data(
    wl_shell_surface: *mut wl_shell_surface,
    user_data: *mut c_void,
) {
    unsafe {
        wl_proxy_set_user_data(
            wl_shell_surface.cast(),
            user_data,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_shm_add_listener(
    wl_shm: *mut wl_shm,
    listener: *const wl_shm_listener,
    data: *mut c_void,
) -> c_int {
    unsafe {
        wl_proxy_add_listener(
            wl_shm.cast(),
            listener.cast_mut().cast(),
            data,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_shm_create_pool(
    wl_shm: *mut wl_shm,
    fd: i32,
    size: i32,
) -> *mut wl_shm_pool {
    unsafe {
        wl_proxy_marshal_flags(
            wl_shm.cast(),
            WL_SHM_CREATE_POOL,
            &wl_shm_pool_interface,
            wl_proxy_get_version(wl_shm.cast()),
            0,
            null_mut::<c_void>(),
            fd,
            size,
        ).cast()
    }
}

#[inline]
pub unsafe extern "C" fn wl_shm_destroy(
    wl_shm: *mut wl_shm,
) {
    unsafe { wl_proxy_destroy(wl_shm.cast()) }
}

#[inline]
pub unsafe extern "C" fn wl_shm_get_user_data(
    wl_shm: *mut wl_shm,
) -> *mut c_void {
    unsafe { wl_proxy_get_user_data(wl_shm.cast()) }
}

#[inline]
pub unsafe extern "C" fn wl_shm_get_version(
    wl_shm: *mut wl_shm,
) -> u32 {
    unsafe { wl_proxy_get_version(wl_shm.cast()) }
}

#[inline]
pub unsafe extern "C" fn wl_shm_pool_create_buffer(
    wl_shm_pool: *mut wl_shm_pool,
    offset: i32,
    width: i32,
    height: i32,
    stride: i32,
    format: u32,
) -> *mut wl_buffer {
    unsafe {
        wl_proxy_marshal_flags(
            wl_shm_pool.cast(),
            WL_SHM_POOL_CREATE_BUFFER,
            &wl_buffer_interface,
            wl_proxy_get_version(wl_shm_pool.cast()),
            0,
            null_mut::<c_void>(),
            offset,
            width,
            height,
            stride,
            format,
        ).cast()
    }
}

#[inline]
pub unsafe extern "C" fn wl_shm_pool_destroy(
    wl_shm_pool: *mut wl_shm_pool,
) {
    unsafe {
        wl_proxy_marshal_flags(
            wl_shm_pool.cast(),
            WL_SHM_POOL_DESTROY,
            null(),
            wl_proxy_get_version(wl_shm_pool.cast()),
            WL_MARSHAL_FLAG_DESTROY,
        );
    }
}

#[inline]
pub unsafe extern "C" fn wl_shm_pool_get_user_data(
    wl_shm_pool: *mut wl_shm_pool,
) -> *mut c_void {
    unsafe { wl_proxy_get_user_data(wl_shm_pool.cast()) }
}

#[inline]
pub unsafe extern "C" fn wl_shm_pool_get_version(
    wl_shm_pool: *mut wl_shm_pool,
) -> u32 {
    unsafe { wl_proxy_get_version(wl_shm_pool.cast()) }
}

#[inline]
pub unsafe extern "C" fn wl_shm_pool_resize(
    wl_shm_pool: *mut wl_shm_pool,
    size: i32,
) {
    unsafe {
        wl_proxy_marshal_flags(
            wl_shm_pool.cast(),
            WL_SHM_POOL_RESIZE,
            null(),
            wl_proxy_get_version(wl_shm_pool.cast()),
            0,
            size,
        );
    }
}

#[inline]
pub unsafe extern "C" fn wl_shm_pool_set_user_data(
    wl_shm_pool: *mut wl_shm_pool,
    user_data: *mut c_void,
) {
    unsafe {
        wl_proxy_set_user_data(
            wl_shm_pool.cast(),
            user_data,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_shm_release(
    wl_shm: *mut wl_shm,
) {
    unsafe {
        wl_proxy_marshal_flags(
            wl_shm.cast(),
            WL_SHM_RELEASE,
            null(),
            wl_proxy_get_version(wl_shm.cast()),
            WL_MARSHAL_FLAG_DESTROY,
        );
    }
}

#[inline]
pub unsafe extern "C" fn wl_shm_set_user_data(
    wl_shm: *mut wl_shm,
    user_data: *mut c_void,
) {
    unsafe {
        wl_proxy_set_user_data(
            wl_shm.cast(),
            user_data,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_subcompositor_destroy(
    wl_subcompositor: *mut wl_subcompositor,
) {
    unsafe {
        wl_proxy_marshal_flags(
            wl_subcompositor.cast(),
            WL_SUBCOMPOSITOR_DESTROY,
            null(),
            wl_proxy_get_version(wl_subcompositor.cast()),
            WL_MARSHAL_FLAG_DESTROY,
        );
    }
}

#[inline]
pub unsafe extern "C" fn wl_subcompositor_get_subsurface(
    wl_subcompositor: *mut wl_subcompositor,
    surface: *mut wl_surface,
    parent: *mut wl_surface,
) -> *mut wl_subsurface {
    unsafe {
        wl_proxy_marshal_flags(
            wl_subcompositor.cast(),
            WL_SUBCOMPOSITOR_GET_SUBSURFACE,
            &wl_subsurface_interface,
            wl_proxy_get_version(wl_subcompositor.cast()),
            0,
            null_mut::<c_void>(),
            surface,
            parent,
        ).cast()
    }
}

#[inline]
pub unsafe extern "C" fn wl_subcompositor_get_user_data(
    wl_subcompositor: *mut wl_subcompositor,
) -> *mut c_void {
    unsafe { wl_proxy_get_user_data(wl_subcompositor.cast()) }
}

#[inline]
pub unsafe extern "C" fn wl_subcompositor_get_version(
    wl_subcompositor: *mut wl_subcompositor,
) -> u32 {
    unsafe { wl_proxy_get_version(wl_subcompositor.cast()) }
}

#[inline]
pub unsafe extern "C" fn wl_subcompositor_set_user_data(
    wl_subcompositor: *mut wl_subcompositor,
    user_data: *mut c_void,
) {
    unsafe {
        wl_proxy_set_user_data(
            wl_subcompositor.cast(),
            user_data,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_subsurface_destroy(
    wl_subsurface: *mut wl_subsurface,
) {
    unsafe {
        wl_proxy_marshal_flags(
            wl_subsurface.cast(),
            WL_SUBSURFACE_DESTROY,
            null(),
            wl_proxy_get_version(wl_subsurface.cast()),
            WL_MARSHAL_FLAG_DESTROY,
        );
    }
}

#[inline]
pub unsafe extern "C" fn wl_subsurface_get_user_data(
    wl_subsurface: *mut wl_subsurface,
) -> *mut c_void {
    unsafe { wl_proxy_get_user_data(wl_subsurface.cast()) }
}

#[inline]
pub unsafe extern "C" fn wl_subsurface_get_version(
    wl_subsurface: *mut wl_subsurface,
) -> u32 {
    unsafe { wl_proxy_get_version(wl_subsurface.cast()) }
}

#[inline]
pub unsafe extern "C" fn wl_subsurface_place_above(
    wl_subsurface: *mut wl_subsurface,
    sibling: *mut wl_surface,
) {
    unsafe {
        wl_proxy_marshal_flags(
            wl_subsurface.cast(),
            WL_SUBSURFACE_PLACE_ABOVE,
            null(),
            wl_proxy_get_version(wl_subsurface.cast()),
            0,
            sibling,
        );
    }
}

#[inline]
pub unsafe extern "C" fn wl_subsurface_place_below(
    wl_subsurface: *mut wl_subsurface,
    sibling: *mut wl_surface,
) {
    unsafe {
        wl_proxy_marshal_flags(
            wl_subsurface.cast(),
            WL_SUBSURFACE_PLACE_BELOW,
            null(),
            wl_proxy_get_version(wl_subsurface.cast()),
            0,
            sibling,
        );
    }
}

#[inline]
pub unsafe extern "C" fn wl_subsurface_set_desync(
    wl_subsurface: *mut wl_subsurface,
) {
    unsafe {
        wl_proxy_marshal_flags(
            wl_subsurface.cast(),
            WL_SUBSURFACE_SET_DESYNC,
            null(),
            wl_proxy_get_version(wl_subsurface.cast()),
            0,
        );
    }
}

#[inline]
pub unsafe extern "C" fn wl_subsurface_set_position(
    wl_subsurface: *mut wl_subsurface,
    x: i32,
    y: i32,
) {
    unsafe {
        wl_proxy_marshal_flags(
            wl_subsurface.cast(),
            WL_SUBSURFACE_SET_POSITION,
            null(),
            wl_proxy_get_version(wl_subsurface.cast()),
            0,
            x,
            y,
        );
    }
}

#[inline]
pub unsafe extern "C" fn wl_subsurface_set_sync(
    wl_subsurface: *mut wl_subsurface,
) {
    unsafe {
        wl_proxy_marshal_flags(
            wl_subsurface.cast(),
            WL_SUBSURFACE_SET_SYNC,
            null(),
            wl_proxy_get_version(wl_subsurface.cast()),
            0,
        );
    }
}

#[inline]
pub unsafe extern "C" fn wl_subsurface_set_user_data(
    wl_subsurface: *mut wl_subsurface,
    user_data: *mut c_void,
) {
    unsafe {
        wl_proxy_set_user_data(
            wl_subsurface.cast(),
            user_data,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_surface_add_listener(
    wl_surface: *mut wl_surface,
    listener: *const wl_surface_listener,
    data: *mut c_void,
) -> c_int {
    unsafe {
        wl_proxy_add_listener(
            wl_surface.cast(),
            listener.cast_mut().cast(),
            data,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_surface_attach(
    wl_surface: *mut wl_surface,
    buffer: *mut wl_buffer,
    x: i32,
    y: i32,
) {
    unsafe {
        wl_proxy_marshal_flags(
            wl_surface.cast(),
            WL_SURFACE_ATTACH,
            null(),
            wl_proxy_get_version(wl_surface.cast()),
            0,
            buffer,
            x,
            y,
        );
    }
}

#[inline]
pub unsafe extern "C" fn wl_surface_commit(
    wl_surface: *mut wl_surface,
) {
    unsafe {
        wl_proxy_marshal_flags(
            wl_surface.cast(),
            WL_SURFACE_COMMIT,
            null(),
            wl_proxy_get_version(wl_surface.cast()),
            0,
        );
    }
}

#[inline]
pub unsafe extern "C" fn wl_surface_damage(
    wl_surface: *mut wl_surface,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
) {
    unsafe {
        wl_proxy_marshal_flags(
            wl_surface.cast(),
            WL_SURFACE_DAMAGE,
            null(),
            wl_proxy_get_version(wl_surface.cast()),
            0,
            x,
            y,
            width,
            height,
        );
    }
}

#[inline]
pub unsafe extern "C" fn wl_surface_damage_buffer(
    wl_surface: *mut wl_surface,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
) {
    unsafe {
        wl_proxy_marshal_flags(
            wl_surface.cast(),
            WL_SURFACE_DAMAGE_BUFFER,
            null(),
            wl_proxy_get_version(wl_surface.cast()),
            0,
            x,
            y,
            width,
            height,
        );
    }
}

#[inline]
pub unsafe extern "C" fn wl_surface_destroy(
    wl_surface: *mut wl_surface,
) {
    unsafe {
        wl_proxy_marshal_flags(
            wl_surface.cast(),
            WL_SURFACE_DESTROY,
            null(),
            wl_proxy_get_version(wl_surface.cast()),
            WL_MARSHAL_FLAG_DESTROY,
        );
    }
}

#[inline]
pub unsafe extern "C" fn wl_surface_frame(
    wl_surface: *mut wl_surface,
) -> *mut wl_callback {
    unsafe {
        wl_proxy_marshal_flags(
            wl_surface.cast(),
            WL_SURFACE_FRAME,
            &wl_callback_interface,
            wl_proxy_get_version(wl_surface.cast()),
            0,
            null_mut::<c_void>(),
        ).cast()
    }
}

#[inline]
pub unsafe extern "C" fn wl_surface_get_user_data(
    wl_surface: *mut wl_surface,
) -> *mut c_void {
    unsafe { wl_proxy_get_user_data(wl_surface.cast()) }
}

#[inline]
pub unsafe extern "C" fn wl_surface_get_version(
    wl_surface: *mut wl_surface,
) -> u32 {
    unsafe { wl_proxy_get_version(wl_surface.cast()) }
}

#[inline]
pub unsafe extern "C" fn wl_surface_offset(
    wl_surface: *mut wl_surface,
    x: i32,
    y: i32,
) {
    unsafe {
        wl_proxy_marshal_flags(
            wl_surface.cast(),
            WL_SURFACE_OFFSET,
            null(),
            wl_proxy_get_version(wl_surface.cast()),
            0,
            x,
            y,
        );
    }
}

#[inline]
pub unsafe extern "C" fn wl_surface_set_buffer_scale(
    wl_surface: *mut wl_surface,
    scale: i32,
) {
    unsafe {
        wl_proxy_marshal_flags(
            wl_surface.cast(),
            WL_SURFACE_SET_BUFFER_SCALE,
            null(),
            wl_proxy_get_version(wl_surface.cast()),
            0,
            scale,
        );
    }
}

#[inline]
pub unsafe extern "C" fn wl_surface_set_buffer_transform(
    wl_surface: *mut wl_surface,
    transform: i32,
) {
    unsafe {
        wl_proxy_marshal_flags(
            wl_surface.cast(),
            WL_SURFACE_SET_BUFFER_TRANSFORM,
            null(),
            wl_proxy_get_version(wl_surface.cast()),
            0,
            transform,
        );
    }
}

#[inline]
pub unsafe extern "C" fn wl_surface_set_input_region(
    wl_surface: *mut wl_surface,
    region: *mut wl_region,
) {
    unsafe {
        wl_proxy_marshal_flags(
            wl_surface.cast(),
            WL_SURFACE_SET_INPUT_REGION,
            null(),
            wl_proxy_get_version(wl_surface.cast()),
            0,
            region,
        );
    }
}

#[inline]
pub unsafe extern "C" fn wl_surface_set_opaque_region(
    wl_surface: *mut wl_surface,
    region: *mut wl_region,
) {
    unsafe {
        wl_proxy_marshal_flags(
            wl_surface.cast(),
            WL_SURFACE_SET_OPAQUE_REGION,
            null(),
            wl_proxy_get_version(wl_surface.cast()),
            0,
            region,
        );
    }
}

#[inline]
pub unsafe extern "C" fn wl_surface_set_user_data(
    wl_surface: *mut wl_surface,
    user_data: *mut c_void,
) {
    unsafe {
        wl_proxy_set_user_data(
            wl_surface.cast(),
            user_data,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_touch_add_listener(
    wl_touch: *mut wl_touch,
    listener: *const wl_touch_listener,
    data: *mut c_void,
) -> c_int {
    unsafe {
        wl_proxy_add_listener(
            wl_touch.cast(),
            listener.cast_mut().cast(),
            data,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wl_touch_destroy(
    wl_touch: *mut wl_touch,
) {
    unsafe { wl_proxy_destroy(wl_touch.cast()) }
}

#[inline]
pub unsafe extern "C" fn wl_touch_get_user_data(
    wl_touch: *mut wl_touch,
) -> *mut c_void {
    unsafe { wl_proxy_get_user_data(wl_touch.cast()) }
}

#[inline]
pub unsafe extern "C" fn wl_touch_get_version(
    wl_touch: *mut wl_touch,
) -> u32 {
    unsafe { wl_proxy_get_version(wl_touch.cast()) }
}

#[inline]
pub unsafe extern "C" fn wl_touch_release(
    wl_touch: *mut wl_touch,
) {
    unsafe {
        wl_proxy_marshal_flags(
            wl_touch.cast(),
            WL_TOUCH_RELEASE,
            null(),
            wl_proxy_get_version(wl_touch.cast()),
            WL_MARSHAL_FLAG_DESTROY,
        );
    }
}

#[inline]
pub unsafe extern "C" fn wl_touch_set_user_data(
    wl_touch: *mut wl_touch,
    user_data: *mut c_void,
) {
    unsafe {
        wl_proxy_set_user_data(
            wl_touch.cast(),
            user_data,
        )
    }
}
