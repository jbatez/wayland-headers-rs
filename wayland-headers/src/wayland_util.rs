use crate::prelude::*;

// TODO: Document.
#[macro_export]
macro_rules! wl_array_for_each {
    ($pos:ident: *const $T:ty, $array:expr, $body:block) => {
        for $pos in $crate::_macro_helpers::WlArrayForEachIter::<$T>::new($array) {
            let $pos = $pos.as_ptr().cast_const();
            $body
        }
    };
    ($pos:ident: *mut $T:ty, $array:expr, $body:block) => {
        for $pos in $crate::_macro_helpers::WlArrayForEachIter::<$T>::new($array) {
            let $pos = $pos.as_ptr();
            $body
        }
    };
}
pub use wl_array_for_each;

// TODO: Document.
#[macro_export]
macro_rules! wl_container_of {
    ($ptr:expr, *const $Container:ty, $member:ident) => {{
        let ptr: *const _ = $ptr;
        let ptr = ptr.cast::<$crate::_macro_helpers::u8>();
        let offset = $crate::_macro_helpers::offset_of!($Container, $member);
        ptr.sub(offset).cast::<$Container>()
    }};
    ($ptr:expr, *mut $Container:ty, $member:ident) => {{
        let ptr: *const _ = $ptr;
        let ptr = ptr.cast::<$crate::_macro_helpers::u8>();
        let offset = $crate::_macro_helpers::offset_of!($Container, $member);
        ptr.sub(offset).cast::<$Container>().cast_mut()
    }};
}
pub use wl_container_of;

// TODO: Document.
#[macro_export]
macro_rules! wl_list_for_each {
    ($pos:ident: *const $T:ty, $head:expr, $member:ident, $body:block) => {
        for $pos in $crate::_macro_helpers::WlListForEachIter::new($head) {
            let $pos = $crate::wl_container_of!($pos.as_ptr(), *const $T, $member);
            $body
        }
    };
    ($pos:ident: *mut $T:ty, $head:expr, $member:ident, $body:block) => {
        for $pos in $crate::_macro_helpers::WlListForEachIter::new($head) {
            let $pos = $crate::wl_container_of!($pos.as_ptr(), *mut $T, $member);
            $body
        }
    };
}
pub use wl_list_for_each;

// TODO: Document.
#[macro_export]
macro_rules! wl_list_for_each_reverse {
    ($pos:ident: *const $T:ty, $head:expr, $member:ident, $body:block) => {
        for $pos in $crate::_macro_helpers::WlListForEachReverseIter::new($head) {
            let $pos = $crate::wl_container_of!($pos.as_ptr(), *const $T, $member);
            $body
        }
    };
    ($pos:ident: *mut $T:ty, $head:expr, $member:ident, $body:block) => {
        for $pos in $crate::_macro_helpers::WlListForEachReverseIter::new($head) {
            let $pos = $crate::wl_container_of!($pos.as_ptr(), *mut $T, $member);
            $body
        }
    };
}
pub use wl_list_for_each_reverse;

// TODO: Document.
#[macro_export]
macro_rules! wl_list_for_each_reverse_safe {
    ($pos:ident: *const $T:ty, $head:expr, $member:ident, $body:block) => {
        for $pos in $crate::_macro_helpers::WlListForEachReverseSafeIter::new($head) {
            let $pos = $crate::wl_container_of!($pos.as_ptr(), *const $T, $member);
            $body
        }
    };
    ($pos:ident: *mut $T:ty, $head:expr, $member:ident, $body:block) => {
        for $pos in $crate::_macro_helpers::WlListForEachReverseSafeIter::new($head) {
            let $pos = $crate::wl_container_of!($pos.as_ptr(), *mut $T, $member);
            $body
        }
    };
}
pub use wl_list_for_each_reverse_safe;

// TODO: Document.
#[macro_export]
macro_rules! wl_list_for_each_safe {
    ($pos:ident: *const $T:ty, $head:expr, $member:ident, $body:block) => {
        for $pos in $crate::_macro_helpers::WlListForEachSafeIter::new($head) {
            let $pos = $crate::wl_container_of!($pos.as_ptr(), *const $T, $member);
            $body
        }
    };
    ($pos:ident: *mut $T:ty, $head:expr, $member:ident, $body:block) => {
        for $pos in $crate::_macro_helpers::WlListForEachSafeIter::new($head) {
            let $pos = $crate::wl_container_of!($pos.as_ptr(), *mut $T, $member);
            $body
        }
    };
}
pub use wl_list_for_each_safe;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct wl_array {
    pub size: usize,
    pub alloc: usize,
    pub data: *mut c_void,
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

#[derive(Clone, Copy)]
#[repr(C)]
pub struct wl_message {
    pub name: *const c_char,
    pub signature: *const c_char,
    pub types: *mut *const wl_interface,
}

#[repr(C)]
pub struct wl_object {
    _data: (),
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

pub const WL_ITERATOR_CONTINUE: wl_iterator_result = 1;
pub const WL_ITERATOR_STOP: wl_iterator_result = 0;
pub const WL_MAX_MESSAGE_SIZE: usize = 4096;

unsafe extern "C" {
    pub fn wl_array_add(array: *mut wl_array, size: usize) -> *mut c_void;
    pub fn wl_array_copy(array: *mut wl_array, source: *mut wl_array) -> c_int;
    pub fn wl_array_init(array: *mut wl_array);
    pub fn wl_array_release(array: *mut wl_array);
}

#[inline]
pub extern "C" fn wl_fixed_from_double(d: c_double) -> wl_fixed_t {
    let d = d + ((3i64 << (51 - 8)) as c_double);
    d.to_bits() as wl_fixed_t
}

#[inline]
pub extern "C" fn wl_fixed_from_int(i: c_int) -> wl_fixed_t {
    i * 256
}

#[inline]
pub extern "C" fn wl_fixed_to_double(f: wl_fixed_t) -> c_double {
    let i = ((1023i64 + 44i64) << 52) + (1i64 << 51) + (f as i64);
    c_double::from_bits(i as u64) - ((3i64 << 43) as c_double)
}

#[inline]
pub extern "C" fn wl_fixed_to_int(f: wl_fixed_t) -> c_int {
    f / 256
}

unsafe extern "C" {
    pub fn wl_list_empty(list: *const wl_list) -> c_int;
    pub fn wl_list_init(list: *mut wl_list);
    pub fn wl_list_insert(list: *mut wl_list, elm: *mut wl_list);
    pub fn wl_list_insert_list(list: *mut wl_list, other: *mut wl_list);
    pub fn wl_list_length(list: *const wl_list) -> c_int;
    pub fn wl_list_remove(elm: *mut wl_list);
}

pub type wl_dispatcher_func_t = Option<
    unsafe extern "C" fn(
        _: *const c_void,
        _: *mut c_void,
        _: u32,
        _: *const wl_message,
        _: *mut wl_argument,
    ) -> c_int,
>;

pub type wl_fixed_t = i32;
pub type wl_iterator_result = c_int;

pub type wl_log_func_t = Option<
    unsafe extern "C" fn(
        _: *const c_char,
        _: *mut c_void, // TODO: VaList
    ),
>;

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

#[cfg(test)]
mod tests {
    use core::{mem::MaybeUninit, ptr::null_mut};

    use super::*;

    #[link(name = "wayland-client")]
    unsafe extern "C" {}

    struct Foo {
        _m: i32,
        link: wl_list,
    }

    impl Foo {
        fn new() -> Self {
            Self {
                _m: 0,
                link: wl_list {
                    prev: null_mut(),
                    next: null_mut(),
                },
            }
        }
    }

    #[test]
    fn test_wl_array_for_each() {
        unsafe {
            let mut array = MaybeUninit::uninit();
            wl_array_init(array.as_mut_ptr());
            let data = wl_array_add(array.as_mut_ptr(), size_of::<Foo>() * 3);

            let mut i = 0;
            wl_array_for_each!(foo: *const Foo, array.as_ptr(), {
                assert!(i < 3);
                assert_eq!(foo, data.cast::<Foo>().add(i));
                i += 1;
            });
            assert_eq!(i, 3);

            let mut i = 0;
            wl_array_for_each!(foo: *mut Foo, array.as_ptr(), {
                assert!(i < 3);
                assert_eq!(foo, data.cast::<Foo>().add(i));
                i += 1;
            });
            assert_eq!(i, 3);

            wl_array_release(array.as_mut_ptr());
        }
    }

    #[test]
    fn test_wl_container_of() {
        let mut foo = Foo::new();
        assert_eq!(
            unsafe { wl_container_of!(&foo.link, *const Foo, link) },
            &raw const foo
        );
        assert_eq!(
            unsafe { wl_container_of!(&foo.link, *mut Foo, link) },
            &raw mut foo
        );
    }

    fn with_test_list<F>(f: F)
    where
        F: FnOnce(&mut wl_list, [&mut Foo; 3]),
    {
        unsafe {
            let mut list = MaybeUninit::uninit();
            wl_list_init(list.as_mut_ptr());
            let list = &mut *list.as_mut_ptr();

            let mut e1 = Foo::new();
            wl_list_insert(list, &mut e1.link);

            let mut e2 = Foo::new();
            wl_list_insert(&mut e1.link, &mut e2.link);

            let mut e3 = Foo::new();
            wl_list_insert(&mut e2.link, &mut e3.link);

            f(list, [&mut e1, &mut e2, &mut e3]);
        }
    }

    #[test]
    fn test_wl_list_for_each() {
        with_test_list(|list, elems| unsafe {
            let mut i = 0;
            wl_list_for_each!(foo: *const Foo, list, link, {
                assert_eq!(foo, elems[i] as *const _);
                i += 1;
            });
            assert_eq!(i, 3);

            let mut i = 0;
            wl_list_for_each!(foo: *mut Foo, list, link, {
                assert_eq!(foo, elems[i] as *mut _);
                i += 1;
            });
            assert_eq!(i, 3);
        });
    }

    #[test]
    fn test_wl_list_for_each_reverse() {
        with_test_list(|list, elems| unsafe {
            let mut i = elems.len();
            wl_list_for_each_reverse!(foo: *const Foo, list, link, {
                i -= 1;
                assert_eq!(foo, elems[i] as *const _);
            });
            assert_eq!(i, 0);

            let mut i = elems.len();
            wl_list_for_each_reverse!(foo: *mut Foo, list, link, {
                i -= 1;
                assert_eq!(foo, elems[i] as *mut _);
            });
            assert_eq!(i, 0);
        });
    }

    #[test]
    fn test_wl_list_for_each_reverse_safe() {
        with_test_list(|list, elems| unsafe {
            let mut i = elems.len();
            wl_list_for_each_reverse_safe!(foo: *const Foo, list, link, {
                i -= 1;
                assert_eq!(foo, elems[i] as *const _);
            });
            assert_eq!(i, 0);

            let mut i = elems.len();
            wl_list_for_each_reverse_safe!(foo: *mut Foo, list, link, {
                i -= 1;
                assert_eq!(foo, elems[i] as *mut _);
                wl_list_remove(&raw mut (*foo).link);
            });
            assert_eq!(i, 0);
        });
    }

    #[test]
    fn test_wl_list_for_each_safe() {
        with_test_list(|list, elems| unsafe {
            let mut i = 0;
            wl_list_for_each_safe!(foo: *const Foo, list, link, {
                assert_eq!(foo, elems[i] as *const _);
                i += 1;
            });
            assert_eq!(i, 3);

            let mut i = 0;
            wl_list_for_each_safe!(foo: *mut Foo, list, link, {
                assert_eq!(foo, elems[i] as *mut _);
                wl_list_remove(&raw mut (*foo).link);
                i += 1;
            });
            assert_eq!(i, 3);
        });
    }
}
