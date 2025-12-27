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

pub const WL_DATA_DEVICE_ERROR_ROLE: u32 = 0;
pub const WL_DATA_DEVICE_ERROR_USED_SOURCE: u32 = 1;
pub const WL_DATA_DEVICE_MANAGER_DND_ACTION_ASK: u32 = 4;
pub const WL_DATA_DEVICE_MANAGER_DND_ACTION_COPY: u32 = 1;
pub const WL_DATA_DEVICE_MANAGER_DND_ACTION_MOVE: u32 = 2;
pub const WL_DATA_DEVICE_MANAGER_DND_ACTION_NONE: u32 = 0;
pub const WL_DATA_OFFER_ERROR_INVALID_ACTION: u32 = 2;
pub const WL_DATA_OFFER_ERROR_INVALID_ACTION_MASK: u32 = 1;
pub const WL_DATA_OFFER_ERROR_INVALID_FINISH: u32 = 0;
pub const WL_DATA_OFFER_ERROR_INVALID_OFFER: u32 = 3;
pub const WL_DATA_SOURCE_ERROR_INVALID_ACTION_MASK: u32 = 0;
pub const WL_DATA_SOURCE_ERROR_INVALID_SOURCE: u32 = 1;
pub const WL_DISPLAY_ERROR_IMPLEMENTATION: u32 = 3;
pub const WL_DISPLAY_ERROR_INVALID_METHOD: u32 = 1;
pub const WL_DISPLAY_ERROR_INVALID_OBJECT: u32 = 0;
pub const WL_DISPLAY_ERROR_NO_MEMORY: u32 = 2;
pub const WL_KEYBOARD_KEYMAP_FORMAT_NO_KEYMAP: u32 = 0;
pub const WL_KEYBOARD_KEYMAP_FORMAT_XKB_V1: u32 = 1;
pub const WL_KEYBOARD_KEY_STATE_PRESSED: u32 = 1;
pub const WL_KEYBOARD_KEY_STATE_RELEASED: u32 = 0;
pub const WL_KEYBOARD_KEY_STATE_REPEATED: u32 = 2;
pub const WL_KEYBOARD_KEY_STATE_REPEATED_SINCE_VERSION: u32 = 10;
pub const WL_OUTPUT_MODE_CURRENT: u32 = 0x1;
pub const WL_OUTPUT_MODE_PREFERRED: u32 = 0x2;
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
pub const WL_POINTER_AXIS_HORIZONTAL_SCROLL: u32 = 1;
pub const WL_POINTER_AXIS_RELATIVE_DIRECTION_IDENTICAL: u32 = 0;
pub const WL_POINTER_AXIS_RELATIVE_DIRECTION_INVERTED: u32 = 1;
pub const WL_POINTER_AXIS_SOURCE_CONTINUOUS: u32 = 2;
pub const WL_POINTER_AXIS_SOURCE_FINGER: u32 = 1;
pub const WL_POINTER_AXIS_SOURCE_WHEEL: u32 = 0;
pub const WL_POINTER_AXIS_SOURCE_WHEEL_TILT: u32 = 3;
pub const WL_POINTER_AXIS_SOURCE_WHEEL_TILT_SINCE_VERSION: u32 = 6;
pub const WL_POINTER_AXIS_VERTICAL_SCROLL: u32 = 0;
pub const WL_POINTER_BUTTON_STATE_PRESSED: u32 = 1;
pub const WL_POINTER_BUTTON_STATE_RELEASED: u32 = 0;
pub const WL_POINTER_ERROR_ROLE: u32 = 0;
pub const WL_SEAT_CAPABILITY_KEYBOARD: u32 = 2;
pub const WL_SEAT_CAPABILITY_POINTER: u32 = 1;
pub const WL_SEAT_CAPABILITY_TOUCH: u32 = 4;
pub const WL_SEAT_ERROR_MISSING_CAPABILITY: u32 = 0;
pub const WL_SHELL_ERROR_ROLE: u32 = 0;
pub const WL_SHELL_SURFACE_FULLSCREEN_METHOD_DEFAULT: u32 = 0;
pub const WL_SHELL_SURFACE_FULLSCREEN_METHOD_DRIVER: u32 = 2;
pub const WL_SHELL_SURFACE_FULLSCREEN_METHOD_FILL: u32 = 3;
pub const WL_SHELL_SURFACE_FULLSCREEN_METHOD_SCALE: u32 = 1;
pub const WL_SHELL_SURFACE_RESIZE_BOTTOM: u32 = 2;
pub const WL_SHELL_SURFACE_RESIZE_BOTTOM_LEFT: u32 = 6;
pub const WL_SHELL_SURFACE_RESIZE_BOTTOM_RIGHT: u32 = 10;
pub const WL_SHELL_SURFACE_RESIZE_LEFT: u32 = 4;
pub const WL_SHELL_SURFACE_RESIZE_NONE: u32 = 0;
pub const WL_SHELL_SURFACE_RESIZE_RIGHT: u32 = 8;
pub const WL_SHELL_SURFACE_RESIZE_TOP: u32 = 1;
pub const WL_SHELL_SURFACE_RESIZE_TOP_LEFT: u32 = 5;
pub const WL_SHELL_SURFACE_RESIZE_TOP_RIGHT: u32 = 9;
pub const WL_SHELL_SURFACE_TRANSIENT_INACTIVE: u32 = 0x1;
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
pub const WL_SUBCOMPOSITOR_ERROR_BAD_PARENT: u32 = 1;
pub const WL_SUBCOMPOSITOR_ERROR_BAD_SURFACE: u32 = 0;
pub const WL_SUBSURFACE_ERROR_BAD_SURFACE: u32 = 0;
pub const WL_SURFACE_ERROR_DEFUNCT_ROLE_OBJECT: u32 = 4;
pub const WL_SURFACE_ERROR_INVALID_OFFSET: u32 = 3;
pub const WL_SURFACE_ERROR_INVALID_SCALE: u32 = 0;
pub const WL_SURFACE_ERROR_INVALID_SIZE: u32 = 2;
pub const WL_SURFACE_ERROR_INVALID_TRANSFORM: u32 = 1;

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
