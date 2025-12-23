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
        args: *const c_void, // TODO: VaList
    ),
>;

pub type wl_iterator_result = c_int;
pub const WL_ITERATOR_STOP: wl_iterator_result = 0;
pub const WL_ITERATOR_CONTINUE: wl_iterator_result = 1;
