use core::{
    ffi::{c_char, c_double, c_int, c_void},
    marker::{PhantomData, PhantomPinned},
};

#[repr(C)]
pub struct wl_object {
    _data: (),
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

pub const WL_MAX_MESSAGE_SIZE: usize = 4096;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct wl_message {
    pub name: *const c_char,
    pub signature: *const c_char,
    pub types: *mut *const wl_interface,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct wl_interface {
    pub name: *const c_char,
    pub version: c_int,
    pub method_count: c_int,
    pub methods: *const wl_message,
    pub event_count: c_int,
    pub events: *const wl_message,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct wl_list {
    pub prev: *mut wl_list,
    pub next: *mut wl_list,
}

unsafe extern "C" {
    pub fn wl_list_init(list: *mut wl_list);
    pub fn wl_list_insert(list: *mut wl_list, elm: *mut wl_list);
    pub fn wl_list_remove(elm: *mut wl_list);
    pub fn wl_list_length(list: *const wl_list) -> c_int;
    pub fn wl_list_empty(list: *const wl_list) -> c_int;
    pub fn wl_list_insert_list(list: *mut wl_list, other: *mut wl_list);
}

// TODO: Document.
#[macro_export]
macro_rules! wl_container_of {
    ($ptr:expr, *const $Container:ty, $member:ident) => {{
        let ptr: *const _ = $ptr;
        let ptr = ptr as *mut $crate::_macro_helpers::u8;
        let offset = $crate::_macro_helpers::offset_of!($Container, $member);
        ptr.sub(offset) as *const $Container
    }};
    ($ptr:expr, *mut $Container:ty, $member:ident) => {{
        let ptr: *const _ = $ptr;
        let ptr = ptr as *mut $crate::_macro_helpers::u8;
        let offset = $crate::_macro_helpers::offset_of!($Container, $member);
        ptr.sub(offset) as *mut $Container
    }};
}
pub use wl_container_of;

// TODO: Document.
#[macro_export]
macro_rules! wl_list_for_each {
    ($pos:ident: *const $Container:ty, $head:expr, $member:ident, $body:expr) => {{
        for $pos in $crate::_macro_helpers::WlListForEachIter::new($head) {
            let $pos = $crate::wl_container_of!($pos.as_ptr(), *const $Container, $member);
            $body
        }
    }};
    ($pos:ident: *mut $Container:ty, $head:expr, $member:ident, $body:expr) => {{
        for $pos in $crate::_macro_helpers::WlListForEachIter::new($head) {
            let $pos = $crate::wl_container_of!($pos.as_ptr(), *mut $Container, $member);
            $body
        }
    }};
}
pub use wl_list_for_each;

// TODO: wl_list_for_each_safe
// TODO: wl_list_for_each_reverse
// TODO: wl_list_for_each_reverse_safe

#[derive(Clone, Copy)]
#[repr(C)]
pub struct wl_array {
    pub size: usize,
    pub alloc: usize,
    pub data: *mut c_void,
}

unsafe extern "C" {
    pub fn wl_array_init(array: *mut wl_array);
    pub fn wl_array_release(array: *mut wl_array);
    pub fn wl_array_add(array: *mut wl_array, size: usize) -> *mut c_void;
    pub fn wl_array_copy(array: *mut wl_array, source: *mut wl_array) -> c_int;
}

// TODO: wl_array_for_each

pub type wl_fixed_t = i32;

#[inline]
pub fn wl_fixed_to_double(f: wl_fixed_t) -> c_double {
    (f as c_double) / 256.0
}

/// Available if built with `std`.
#[cfg(any(doc, feature = "std"))]
#[inline]
pub fn wl_fixed_from_double(d: c_double) -> wl_fixed_t {
    (d * 256.0).round() as wl_fixed_t
}

#[inline]
pub fn wl_fixed_to_int(f: wl_fixed_t) -> c_int {
    f / 256
}

#[inline]
pub fn wl_fixed_from_int(i: c_int) -> wl_fixed_t {
    i * 256
}

#[repr(C)]
pub union wl_argument {
    pub i: i32,
    pub u: u32,
    pub f: wl_fixed_t,
    pub s: *const c_char,
    pub o: *mut wl_object,
    pub n: u32,
    pub a: *mut wl_array,
    pub h: i32,
}

pub type wl_dispatcher_func_t = Option<
    unsafe extern "C" fn(
        user_data: *const c_void,
        target: *mut c_void,
        opcode: u32,
        msg: *const wl_message,
        args: *mut wl_argument,
    ) -> c_int,
>;

pub type wl_log_func_t = Option<
    unsafe extern "C" fn(
        fmt: *const c_char,
        args: *mut c_void, // TODO: VaList
    ),
>;

pub type wl_iterator_result = c_int;
pub const WL_ITERATOR_STOP: wl_iterator_result = 0;
pub const WL_ITERATOR_CONTINUE: wl_iterator_result = 1;

#[cfg(test)]
mod tests {
    use core::{mem::MaybeUninit, ptr::null_mut};

    use super::*;

    #[link(name = "wayland-client")]
    unsafe extern "C" {}

    #[test]
    fn test_wl_container_of() {
        struct S {
            _m1: i32,
            m2: i32,
        }

        unsafe {
            let mut s = S { _m1: 0, m2: 0 };
            assert_eq!(wl_container_of!(&s.m2, *const S, m2), &raw const s);
            assert_eq!(wl_container_of!(&s.m2, *mut S, m2), &raw mut s);
        }
    }

    #[test]
    fn test_wl_list_for_each() {
        struct Foo {
            val: i32,
            link: wl_list,
        }

        impl Foo {
            fn new(val: i32) -> Self {
                Self {
                    val,
                    link: wl_list {
                        prev: null_mut(),
                        next: null_mut(),
                    },
                }
            }
        }

        unsafe {
            let mut list = MaybeUninit::uninit();
            wl_list_init(list.as_mut_ptr());
            let mut list = list.assume_init();

            let mut e1 = Foo::new(1);
            wl_list_insert(&mut list, &mut e1.link);

            let mut e2 = Foo::new(2);
            wl_list_insert(&mut e1.link, &mut e2.link);

            let mut e3 = Foo::new(3);
            wl_list_insert(&mut e2.link, &mut e3.link);

            let mut expected = 0;
            wl_list_for_each!(foo: *const Foo, &list, link, {
                expected += 1;
                assert_eq!((*foo).val, expected);
            });

            let mut expected = 0;
            wl_list_for_each!(foo: *mut Foo, &list, link, {
                expected += 1;
                assert_eq!((*foo).val, expected);
            });
        }
    }
}
