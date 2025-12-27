use crate::wayland_server_core::*;

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
