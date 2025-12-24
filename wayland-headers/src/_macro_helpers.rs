use core::ptr::NonNull;

use crate::wayland_util::wl_list;

pub use core::mem::offset_of;
pub use u8;

pub struct WlListForEachIter {
    pos: *const wl_list,
    head: *const wl_list,
}

impl WlListForEachIter {
    pub unsafe fn new(head: *const wl_list) -> Self {
        Self { pos: head, head }
    }
}

impl Iterator for WlListForEachIter {
    type Item = NonNull<wl_list>;
    fn next(&mut self) -> Option<Self::Item> {
        self.pos = unsafe { (*self.pos).next };
        if self.pos != self.head {
            NonNull::new(self.pos.cast_mut())
        } else {
            None
        }
    }
}

pub struct WlListForEachSafeIter {
    next: *const wl_list,
    head: *const wl_list,
}

impl WlListForEachSafeIter {
    pub unsafe fn new(head: *const wl_list) -> Self {
        Self {
            next: unsafe { (*head).next },
            head,
        }
    }
}

impl Iterator for WlListForEachSafeIter {
    type Item = NonNull<wl_list>;
    fn next(&mut self) -> Option<Self::Item> {
        let pos = self.next;
        self.next = unsafe { (*pos).next };
        if pos != self.head {
            NonNull::new(pos.cast_mut())
        } else {
            None
        }
    }
}
