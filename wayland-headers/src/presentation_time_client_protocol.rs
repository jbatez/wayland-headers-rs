use crate::prelude::*;

use super::wayland_client::*;

#[repr(C)]
pub struct wp_presentation {
    _data: (),
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[repr(C)]
pub struct wp_presentation_feedback {
    _data: (),
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct wp_presentation_feedback_listener {
    pub sync_output: Option<unsafe extern "C" fn(
        data: *mut c_void,
        wp_presentation_feedback: *mut wp_presentation_feedback,
        output: *mut wl_output,
    )>,
    pub presented: Option<unsafe extern "C" fn(
        data: *mut c_void,
        wp_presentation_feedback: *mut wp_presentation_feedback,
        tv_sec_hi: u32,
        tv_sec_lo: u32,
        tv_nsec: u32,
        refresh: u32,
        seq_hi: u32,
        seq_lo: u32,
        flags: u32,
    )>,
    pub discarded: Option<unsafe extern "C" fn(
        data: *mut c_void,
        wp_presentation_feedback: *mut wp_presentation_feedback,
    )>,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct wp_presentation_listener {
    pub clock_id: Option<unsafe extern "C" fn(
        data: *mut c_void,
        wp_presentation: *mut wp_presentation,
        clk_id: u32,
    )>,
}

pub const WP_PRESENTATION_CLOCK_ID_SINCE_VERSION: u32 = 1;
pub const WP_PRESENTATION_DESTROY: u32 = 0;
pub const WP_PRESENTATION_DESTROY_SINCE_VERSION: u32 = 1;
pub const WP_PRESENTATION_ERROR_INVALID_FLAG: u32 = 1;
pub const WP_PRESENTATION_ERROR_INVALID_TIMESTAMP: u32 = 0;
pub const WP_PRESENTATION_FEEDBACK: u32 = 1;
pub const WP_PRESENTATION_FEEDBACK_DISCARDED_SINCE_VERSION: u32 = 1;
pub const WP_PRESENTATION_FEEDBACK_KIND_HW_CLOCK: u32 = 0x2;
pub const WP_PRESENTATION_FEEDBACK_KIND_HW_COMPLETION: u32 = 0x4;
pub const WP_PRESENTATION_FEEDBACK_KIND_VSYNC: u32 = 0x1;
pub const WP_PRESENTATION_FEEDBACK_KIND_ZERO_COPY: u32 = 0x8;
pub const WP_PRESENTATION_FEEDBACK_PRESENTED_SINCE_VERSION: u32 = 1;
pub const WP_PRESENTATION_FEEDBACK_SINCE_VERSION: u32 = 1;
pub const WP_PRESENTATION_FEEDBACK_SYNC_OUTPUT_SINCE_VERSION: u32 = 1;

unsafe extern "C" {
    pub static wp_presentation_feedback_interface: wl_interface;
    pub static wp_presentation_interface: wl_interface;
}

#[inline]
pub unsafe extern "C" fn wp_presentation_add_listener(
    wp_presentation: *mut wp_presentation,
    listener: *const wp_presentation_listener,
    data: *mut c_void,
) -> c_int {
    unsafe {
        wl_proxy_add_listener(
            wp_presentation.cast(),
            listener.cast_mut().cast(),
            data,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wp_presentation_destroy(
    wp_presentation: *mut wp_presentation,
) {
    unsafe {
        wl_proxy_marshal(
            wp_presentation.cast(),
            WP_PRESENTATION_DESTROY,
        );
        wl_proxy_destroy(
            wp_presentation.cast(),
        );
    }
}

#[inline]
pub unsafe extern "C" fn wp_presentation_feedback(
    wp_presentation: *mut wp_presentation,
    surface: *mut wl_surface,
) -> *mut wp_presentation_feedback {
    unsafe {
        wl_proxy_marshal_constructor(
            wp_presentation.cast(),
            WP_PRESENTATION_FEEDBACK,
            &wp_presentation_feedback_interface,
            surface,
            null_mut::<c_void>(),
        ).cast()
    }
}

#[inline]
pub unsafe extern "C" fn wp_presentation_feedback_add_listener(
    wp_presentation_feedback: *mut wp_presentation_feedback,
    listener: *const wp_presentation_feedback_listener,
    data: *mut c_void,
) -> c_int {
    unsafe {
        wl_proxy_add_listener(
            wp_presentation_feedback.cast(),
            listener.cast_mut().cast(),
            data,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wp_presentation_feedback_destroy(
    wp_presentation_feedback: *mut wp_presentation_feedback,
) {
    unsafe { wl_proxy_destroy(wp_presentation_feedback.cast()) }
}

#[inline]
pub unsafe extern "C" fn wp_presentation_feedback_get_user_data(
    wp_presentation_feedback: *mut wp_presentation_feedback,
) -> *mut c_void {
    unsafe { wl_proxy_get_user_data(wp_presentation_feedback.cast()) }
}

#[inline]
pub unsafe extern "C" fn wp_presentation_feedback_get_version(
    wp_presentation_feedback: *mut wp_presentation_feedback,
) -> u32 {
    unsafe { wl_proxy_get_version(wp_presentation_feedback.cast()) }
}

#[inline]
pub unsafe extern "C" fn wp_presentation_feedback_set_user_data(
    wp_presentation_feedback: *mut wp_presentation_feedback,
    user_data: *mut c_void,
) {
    unsafe {
        wl_proxy_set_user_data(
            wp_presentation_feedback.cast(),
            user_data,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wp_presentation_get_user_data(
    wp_presentation: *mut wp_presentation,
) -> *mut c_void {
    unsafe { wl_proxy_get_user_data(wp_presentation.cast()) }
}

#[inline]
pub unsafe extern "C" fn wp_presentation_get_version(
    wp_presentation: *mut wp_presentation,
) -> u32 {
    unsafe { wl_proxy_get_version(wp_presentation.cast()) }
}

#[inline]
pub unsafe extern "C" fn wp_presentation_set_user_data(
    wp_presentation: *mut wp_presentation,
    user_data: *mut c_void,
) {
    unsafe {
        wl_proxy_set_user_data(
            wp_presentation.cast(),
            user_data,
        )
    }
}
