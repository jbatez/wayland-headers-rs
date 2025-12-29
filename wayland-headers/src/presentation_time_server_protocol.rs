use crate::prelude::*;

use super::wayland_server::*;

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
pub struct wp_presentation_interface {
    pub destroy: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
    )>,
    pub feedback: Option<unsafe extern "C" fn(
        client: *mut wl_client,
        resource: *mut wl_resource,
        surface: *mut wl_resource,
        callback: u32,
    )>,
}

pub const WP_PRESENTATION_CLOCK_ID: u32 = 0;
pub const WP_PRESENTATION_CLOCK_ID_SINCE_VERSION: u32 = 1;
pub const WP_PRESENTATION_DESTROY_SINCE_VERSION: u32 = 1;
pub const WP_PRESENTATION_ERROR_INVALID_FLAG: u32 = 1;
pub const WP_PRESENTATION_ERROR_INVALID_TIMESTAMP: u32 = 0;
pub const WP_PRESENTATION_FEEDBACK_DISCARDED: u32 = 2;
pub const WP_PRESENTATION_FEEDBACK_DISCARDED_SINCE_VERSION: u32 = 1;
pub const WP_PRESENTATION_FEEDBACK_KIND_HW_CLOCK: u32 = 0x2;
pub const WP_PRESENTATION_FEEDBACK_KIND_HW_COMPLETION: u32 = 0x4;
pub const WP_PRESENTATION_FEEDBACK_KIND_VSYNC: u32 = 0x1;
pub const WP_PRESENTATION_FEEDBACK_KIND_ZERO_COPY: u32 = 0x8;
pub const WP_PRESENTATION_FEEDBACK_PRESENTED: u32 = 1;
pub const WP_PRESENTATION_FEEDBACK_PRESENTED_SINCE_VERSION: u32 = 1;
pub const WP_PRESENTATION_FEEDBACK_SINCE_VERSION: u32 = 1;
pub const WP_PRESENTATION_FEEDBACK_SYNC_OUTPUT: u32 = 0;
pub const WP_PRESENTATION_FEEDBACK_SYNC_OUTPUT_SINCE_VERSION: u32 = 1;

unsafe extern "C" {
    pub static wp_presentation_feedback_interface: wl_interface;
    pub static wp_presentation_interface: wl_interface;
}

#[inline]
pub unsafe extern "C" fn wp_presentation_feedback_send_discarded(
    resource_: *mut wl_resource,
) {
    unsafe {
        wl_resource_post_event(
            resource_,
            WP_PRESENTATION_FEEDBACK_DISCARDED,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wp_presentation_feedback_send_presented(
    resource_: *mut wl_resource,
    tv_sec_hi: u32,
    tv_sec_lo: u32,
    tv_nsec: u32,
    refresh: u32,
    seq_hi: u32,
    seq_lo: u32,
    flags: u32,
) {
    unsafe {
        wl_resource_post_event(
            resource_,
            WP_PRESENTATION_FEEDBACK_PRESENTED,
            tv_sec_hi,
            tv_sec_lo,
            tv_nsec,
            refresh,
            seq_hi,
            seq_lo,
            flags,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wp_presentation_feedback_send_sync_output(
    resource_: *mut wl_resource,
    output: *mut wl_resource,
) {
    unsafe {
        wl_resource_post_event(
            resource_,
            WP_PRESENTATION_FEEDBACK_SYNC_OUTPUT,
            output,
        )
    }
}

#[inline]
pub unsafe extern "C" fn wp_presentation_send_clock_id(
    resource_: *mut wl_resource,
    clk_id: u32,
) {
    unsafe {
        wl_resource_post_event(
            resource_,
            WP_PRESENTATION_CLOCK_ID,
            clk_id,
        )
    }
}
