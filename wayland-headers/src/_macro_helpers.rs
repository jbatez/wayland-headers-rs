use core::ptr::NonNull;

use crate::wayland_util::{wl_array, wl_list};

pub use core::mem::offset_of;

pub use u8;

pub struct WlArrayForEachIter<T> {
    pos: *mut T,
    array: NonNull<wl_array>,
}

impl<T> WlArrayForEachIter<T> {
    pub unsafe fn new(array: *const wl_array) -> Self {
        Self {
            pos: unsafe { (*array).data.cast() },
            array: unsafe { NonNull::new_unchecked(array.cast_mut()) },
        }
    }
}

impl<T> Iterator for WlArrayForEachIter<T> {
    type Item = NonNull<T>;
    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let pos = self.pos;
            let array = self.array.as_ref();
            if array.size != 0 && pos.cast::<u8>() < array.data.cast::<u8>().add(array.size) {
                self.pos = pos.add(1);
                Some(NonNull::new_unchecked(pos))
            } else {
                None
            }
        }
    }
}

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

pub struct WlListForEachReverseIter {
    pos: NonNull<wl_list>,
    head: NonNull<wl_list>,
}

impl WlListForEachReverseIter {
    pub unsafe fn new(head: *const wl_list) -> Self {
        let head = unsafe { NonNull::new_unchecked(head.cast_mut()) };
        Self { pos: head, head }
    }
}

impl Iterator for WlListForEachReverseIter {
    type Item = NonNull<wl_list>;
    fn next(&mut self) -> Option<Self::Item> {
        self.pos = unsafe { NonNull::new_unchecked(self.pos.as_ref().prev) };
        match self.pos != self.head {
            true => Some(self.pos),
            false => None,
        }
    }
}

pub struct WlListForEachReverseSafeIter {
    next: NonNull<wl_list>,
    head: NonNull<wl_list>,
}

impl WlListForEachReverseSafeIter {
    pub unsafe fn new(head: *const wl_list) -> Self {
        Self {
            next: unsafe { NonNull::new_unchecked((*head).prev) },
            head: unsafe { NonNull::new_unchecked(head.cast_mut()) },
        }
    }
}

impl Iterator for WlListForEachReverseSafeIter {
    type Item = NonNull<wl_list>;
    fn next(&mut self) -> Option<Self::Item> {
        let pos = self.next;
        self.next = unsafe { NonNull::new_unchecked(pos.as_ref().prev) };
        match pos != self.head {
            true => Some(pos),
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
