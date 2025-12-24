use core::ptr::NonNull;

use crate::wayland_util::wl_list;

pub use core::mem::offset_of;
pub use u8;

pub struct WlListForEachIter {
    pos: NonNull<wl_list>,
    head: NonNull<wl_list>,
}

impl WlListForEachIter {
    pub unsafe fn new(head: *const wl_list) -> Self {
        let head = unsafe { NonNull::new_unchecked(head.cast_mut()) };
        Self { pos: head, head }
    }
}

impl Iterator for WlListForEachIter {
    type Item = NonNull<wl_list>;
    fn next(&mut self) -> Option<Self::Item> {
        self.pos = unsafe { NonNull::new_unchecked(self.pos.as_ref().next) };
        match self.pos != self.head {
            true => Some(self.pos),
            false => None,
        }
    }
}

pub struct WlListForEachSafeIter {
    next: NonNull<wl_list>,
    head: NonNull<wl_list>,
}

impl WlListForEachSafeIter {
    pub unsafe fn new(head: *const wl_list) -> Self {
        Self {
            next: unsafe { NonNull::new_unchecked((*head).next) },
            head: unsafe { NonNull::new_unchecked(head.cast_mut()) },
        }
    }
}

impl Iterator for WlListForEachSafeIter {
    type Item = NonNull<wl_list>;
    fn next(&mut self) -> Option<Self::Item> {
        let pos = self.next;
        self.next = unsafe { NonNull::new_unchecked(pos.as_ref().next) };
        match pos != self.head {
            true => Some(pos),
            false => None,
        }
    }
}
