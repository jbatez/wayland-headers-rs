use libc::{gid_t, pid_t, uid_t};

use crate::prelude::*;

#[doc(no_inline)]
pub use super::wayland_util::*;

#[doc(no_inline)]
pub use super::wayland_version::*;

// TODO: Document.
#[macro_export]
macro_rules! wl_client_for_each {
    ($client:ident: $T:ty, $list:expr, $body:block) => {
        for $client in $crate::_macro_helpers::WlListForEachIter::new($list) {
            let $client: $T = $crate::wayland_server_core::wl_client_from_link($client.as_ptr());
            $body
        }
    };
}
pub use wl_client_for_each;

// TODO: Document.
#[macro_export]
macro_rules! wl_resource_for_each {
    ($resource:ident: $T:ty, $list:expr, $body:block) => {
        for $resource in $crate::_macro_helpers::WlListForEachIter::new($list) {
            let $resource: $T =
                $crate::wayland_server_core::wl_resource_from_link($client.as_ptr());
            $body
        }
    };
}
pub use wl_resource_for_each;

// TODO: Document.
#[macro_export]
macro_rules! wl_resource_for_each_safe {
    ($resource:ident: $T:ty, $list:expr, $body:block) => {
        for $resource in $crate::_macro_helpers::WlListForEachSafeIter::new($list) {
            let $resource: $T =
                $crate::wayland_server_core::wl_resource_from_link($client.as_ptr());
            $body
        }
    };
}
pub use wl_resource_for_each_safe;

#[repr(C)]
pub struct wl_client {
    _data: (),
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[repr(C)]
pub struct wl_display {
    _data: (),
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[repr(C)]
pub struct wl_event_loop {
    _data: (),
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[repr(C)]
pub struct wl_event_source {
    _data: (),
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[repr(C)]
pub struct wl_global {
    _data: (),
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct wl_listener {
    pub link: wl_list,
    pub notify: wl_notify_func_t,
}

#[repr(C)]
pub struct wl_protocol_logger {
    _data: (),
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct wl_protocol_logger_message {
    pub resource: *mut wl_resource,
    pub message_opcode: c_int,
    pub message: *const wl_message,
    pub arguments_count: c_int,
    pub arguments: *const wl_argument,
}

#[repr(C)]
pub struct wl_resource {
    _data: (),
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[repr(C)]
pub struct wl_shm_buffer {
    _data: (),
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[repr(C)]
pub struct wl_shm_pool {
    _data: (),
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct wl_signal {
    pub listener_list: wl_list,
}

pub const WL_EVENT_ERROR: u32 = 0x08;
pub const WL_EVENT_HANGUP: u32 = 0x04;
pub const WL_EVENT_READABLE: u32 = 0x01;
pub const WL_EVENT_WRITABLE: u32 = 0x02;
pub const WL_PROTOCOL_LOGGER_EVENT: wl_protocol_logger_type = 1;
pub const WL_PROTOCOL_LOGGER_REQUEST: wl_protocol_logger_type = 0;

unsafe extern "C" {
    pub fn wl_client_add_destroy_late_listener(client: *mut wl_client, listener: *mut wl_listener);
    pub fn wl_client_add_destroy_listener(client: *mut wl_client, listener: *mut wl_listener);

    pub fn wl_client_add_resource_created_listener(
        client: *mut wl_client,
        listener: *mut wl_listener,
    );

    pub fn wl_client_create(display: *mut wl_display, fd: c_int) -> *mut wl_client;
    pub fn wl_client_destroy(client: *mut wl_client);
    pub fn wl_client_flush(client: *mut wl_client);

    pub fn wl_client_for_each_resource(
        client: *mut wl_client,
        iterator: wl_client_for_each_resource_iterator_func_t,
        user_data: *mut c_void,
    );

    pub fn wl_client_from_link(link: *mut wl_list) -> *mut wl_client;

    pub fn wl_client_get_credentials(
        client: *const wl_client,
        pid: *mut pid_t,
        uid: *mut uid_t,
        gid: *mut gid_t,
    );

    pub fn wl_client_get_destroy_late_listener(
        client: *mut wl_client,
        notify: wl_notify_func_t,
    ) -> *mut wl_listener;

    pub fn wl_client_get_destroy_listener(
        client: *mut wl_client,
        notify: wl_notify_func_t,
    ) -> *mut wl_listener;

    pub fn wl_client_get_display(client: *mut wl_client) -> *mut wl_display;
    pub fn wl_client_get_fd(client: *mut wl_client) -> c_int;
    pub fn wl_client_get_link(client: *mut wl_client) -> *mut wl_list;
    pub fn wl_client_get_object(client: *mut wl_client, id: u32) -> *mut wl_resource;
    pub fn wl_client_get_user_data(client: *mut wl_client) -> *mut c_void;
    pub fn wl_client_post_implementation_error(client: *mut wl_client, msg: *const c_char, ...);
    pub fn wl_client_post_no_memory(client: *mut wl_client);
    pub fn wl_client_set_max_buffer_size(client: *mut wl_client, max_buffer_size: usize);

    pub fn wl_client_set_user_data(
        client: *mut wl_client,
        data: *mut c_void,
        dtor: wl_user_data_destroy_func_t,
    );

    pub fn wl_display_add_client_created_listener(
        display: *mut wl_display,
        listener: *mut wl_listener,
    );

    pub fn wl_display_add_destroy_listener(display: *mut wl_display, listener: *mut wl_listener);

    pub fn wl_display_add_protocol_logger(
        display: *mut wl_display,
        _: wl_protocol_logger_func_t,
        user_data: *mut c_void,
    ) -> *mut wl_protocol_logger;

    pub fn wl_display_add_shm_format(display: *mut wl_display, format: u32) -> *mut u32;
    pub fn wl_display_add_socket(display: *mut wl_display, name: *const c_char) -> c_int;
    pub fn wl_display_add_socket_auto(display: *mut wl_display) -> *const c_char;
    pub fn wl_display_add_socket_fd(display: *mut wl_display, sock_fd: c_int) -> c_int;
    pub fn wl_display_create() -> *mut wl_display;
    pub fn wl_display_destroy(display: *mut wl_display);
    pub fn wl_display_destroy_clients(display: *mut wl_display);
    pub fn wl_display_flush_clients(display: *mut wl_display);
    pub fn wl_display_get_client_list(display: *mut wl_display) -> *mut wl_list;

    pub fn wl_display_get_destroy_listener(
        display: *mut wl_display,
        notify: wl_notify_func_t,
    ) -> *mut wl_listener;

    pub fn wl_display_get_event_loop(display: *mut wl_display) -> *mut wl_event_loop;
    pub fn wl_display_get_serial(display: *const wl_display) -> u32;
    pub fn wl_display_init_shm(display: *mut wl_display) -> c_int;
    pub fn wl_display_next_serial(display: *mut wl_display) -> u32;
    pub fn wl_display_run(display: *mut wl_display);
    pub fn wl_display_set_default_max_buffer_size(display: *mut wl_display, max_buffer_size: usize);

    pub fn wl_display_set_global_filter(
        display: *mut wl_display,
        filter: wl_display_global_filter_func_t,
        data: *mut c_void,
    );

    pub fn wl_display_terminate(display: *mut wl_display);

    pub fn wl_event_loop_add_destroy_listener(
        _loop: *mut wl_event_loop,
        listener: *mut wl_listener,
    );

    pub fn wl_event_loop_add_fd(
        _loop: *mut wl_event_loop,
        fd: c_int,
        mask: u32,
        func: wl_event_loop_fd_func_t,
        data: *mut c_void,
    ) -> *mut wl_event_source;

    pub fn wl_event_loop_add_idle(
        _loop: *mut wl_event_loop,
        func: wl_event_loop_idle_func_t,
        data: *mut c_void,
    ) -> *mut wl_event_source;

    pub fn wl_event_loop_add_signal(
        _loop: *mut wl_event_loop,
        signal_number: c_int,
        func: wl_event_loop_signal_func_t,
        data: *mut c_void,
    ) -> *mut wl_event_source;

    pub fn wl_event_loop_add_timer(
        _loop: *mut wl_event_loop,
        func: wl_event_loop_timer_func_t,
        data: *mut c_void,
    ) -> *mut wl_event_source;

    pub fn wl_event_loop_create() -> *mut wl_event_loop;
    pub fn wl_event_loop_destroy(_loop: *mut wl_event_loop);
    pub fn wl_event_loop_dispatch(_loop: *mut wl_event_loop, timeout: c_int) -> c_int;
    pub fn wl_event_loop_dispatch_idle(_loop: *mut wl_event_loop);

    pub fn wl_event_loop_get_destroy_listener(
        _loop: *mut wl_event_loop,
        notify: wl_notify_func_t,
    ) -> *mut wl_listener;

    pub fn wl_event_loop_get_fd(_loop: *mut wl_event_loop) -> c_int;
    pub fn wl_event_source_check(source: *mut wl_event_source);
    pub fn wl_event_source_fd_update(source: *mut wl_event_source, mask: u32) -> c_int;
    pub fn wl_event_source_remove(source: *mut wl_event_source) -> c_int;
    pub fn wl_event_source_timer_update(source: *mut wl_event_source, ms_delay: c_int) -> c_int;

    pub fn wl_global_create(
        display: *mut wl_display,
        interface: *const wl_interface,
        version: c_int,
        data: *mut c_void,
        bind: wl_global_bind_func_t,
    ) -> *mut wl_global;

    pub fn wl_global_destroy(global: *mut wl_global);
    pub fn wl_global_get_display(global: *const wl_global) -> *mut wl_display;
    pub fn wl_global_get_interface(global: *const wl_global) -> *const wl_interface;
    pub fn wl_global_get_name(global: *const wl_global, client: *const wl_client) -> u32;
    pub fn wl_global_get_user_data(global: *const wl_global) -> *mut c_void;
    pub fn wl_global_get_version(global: *const wl_global) -> u32;
    pub fn wl_global_remove(global: *mut wl_global);
    pub fn wl_global_set_user_data(global: *mut wl_global, data: *mut c_void);
    pub fn wl_log_set_handler_server(handler: wl_log_func_t);
    pub fn wl_protocol_logger_destroy(logger: *mut wl_protocol_logger);
    pub fn wl_resource_add_destroy_listener(resource: *mut wl_resource, listener: *mut wl_listener);

    pub fn wl_resource_create(
        client: *mut wl_client,
        interface: *const wl_interface,
        version: c_int,
        id: u32,
    ) -> *mut wl_resource;

    pub fn wl_resource_destroy(resource: *mut wl_resource);

    pub fn wl_resource_find_for_client(
        list: *mut wl_list,
        client: *mut wl_client,
    ) -> *mut wl_resource;

    pub fn wl_resource_from_link(resource: *mut wl_list) -> *mut wl_resource;
    pub fn wl_resource_get_class(resource: *const wl_resource) -> *const c_char;
    pub fn wl_resource_get_client(resource: *mut wl_resource) -> *mut wl_client;

    pub fn wl_resource_get_destroy_listener(
        resource: *mut wl_resource,
        notify: wl_notify_func_t,
    ) -> *mut wl_listener;

    pub fn wl_resource_get_id(resource: *const wl_resource) -> u32;
    pub fn wl_resource_get_interface(resource: *mut wl_resource) -> *const wl_interface;
    pub fn wl_resource_get_link(resource: *mut wl_resource) -> *mut wl_list;
    pub fn wl_resource_get_user_data(resource: *mut wl_resource) -> *mut c_void;
    pub fn wl_resource_get_version(resource: *const wl_resource) -> c_int;

    pub fn wl_resource_instance_of(
        resource: *mut wl_resource,
        interface: *const wl_interface,
        implementation: *const c_void,
    ) -> c_int;

    pub fn wl_resource_post_error(resource: *mut wl_resource, code: u32, msg: *const c_char, ...);

    pub fn wl_resource_post_error_vargs(
        resource: *mut wl_resource,
        code: u32,
        msg: *const c_char,
        argp: *mut c_void, // TODO: VaList
    );

    pub fn wl_resource_post_event(resource: *mut wl_resource, opcode: u32, ...);

    pub fn wl_resource_post_event_array(
        resource: *mut wl_resource,
        opcode: u32,
        args: *mut wl_argument,
    );

    pub fn wl_resource_post_no_memory(resource: *mut wl_resource);
    pub fn wl_resource_queue_event(resource: *mut wl_resource, opcode: u32, ...);

    pub fn wl_resource_queue_event_array(
        resource: *mut wl_resource,
        opcode: u32,
        args: *mut wl_argument,
    );

    pub fn wl_resource_set_destructor(
        resource: *mut wl_resource,
        destroy: wl_resource_destroy_func_t,
    );

    pub fn wl_resource_set_dispatcher(
        resource: *mut wl_resource,
        dispatcher: wl_dispatcher_func_t,
        implementation: *const c_void,
        data: *mut c_void,
        destroy: wl_resource_destroy_func_t,
    );

    pub fn wl_resource_set_implementation(
        resource: *mut wl_resource,
        implementation: *const c_void,
        data: *mut c_void,
        destroy: wl_resource_destroy_func_t,
    );

    pub fn wl_resource_set_user_data(resource: *mut wl_resource, data: *mut c_void);
    pub fn wl_shm_buffer_begin_access(buffer: *mut wl_shm_buffer);
    pub fn wl_shm_buffer_end_access(buffer: *mut wl_shm_buffer);
    pub fn wl_shm_buffer_get(resource: *mut wl_resource) -> *mut wl_shm_buffer;
    pub fn wl_shm_buffer_get_data(buffer: *mut wl_shm_buffer) -> *mut c_void;
    pub fn wl_shm_buffer_get_format(buffer: *const wl_shm_buffer) -> u32;
    pub fn wl_shm_buffer_get_height(buffer: *const wl_shm_buffer) -> i32;
    pub fn wl_shm_buffer_get_stride(buffer: *const wl_shm_buffer) -> i32;
    pub fn wl_shm_buffer_get_width(buffer: *const wl_shm_buffer) -> i32;
    pub fn wl_shm_buffer_ref(buffer: *mut wl_shm_buffer) -> *mut wl_shm_buffer;
    pub fn wl_shm_buffer_ref_pool(buffer: *mut wl_shm_buffer) -> *mut wl_shm_pool;
    pub fn wl_shm_buffer_unref(buffer: *mut wl_shm_buffer);
    pub fn wl_shm_pool_unref(pool: *mut wl_shm_pool);
}

#[inline]
pub unsafe extern "C" fn wl_signal_add(signal: *mut wl_signal, listener: *mut wl_listener) {
    unsafe { wl_list_insert((*signal).listener_list.prev, &mut (*listener).link) }
}

#[inline]
pub unsafe extern "C" fn wl_signal_emit(signal: *mut wl_signal, data: *mut c_void) {
    unsafe {
        wl_list_for_each_safe!(l: *mut wl_listener, &(*signal).listener_list, link, {
            (*l).notify.unwrap_unchecked()(l, data);
        })
    }
}

unsafe extern "C" {
    pub fn wl_signal_emit_mutable(signal: *mut wl_signal, data: *mut c_void);
}

#[inline]
pub unsafe extern "C" fn wl_signal_get(
    signal: *mut wl_signal,
    notify: wl_notify_func_t,
) -> *mut wl_listener {
    unsafe {
        wl_list_for_each!(l: *mut wl_listener, &(*signal).listener_list, link, {
            #[allow(unpredictable_function_pointer_comparisons)]
            if (*l).notify == notify {
                return l;
            }
        });
        null_mut()
    }
}

#[inline]
pub unsafe extern "C" fn wl_signal_init(signal: *mut wl_signal) {
    unsafe { wl_list_init(&mut (*signal).listener_list) }
}

pub type wl_client_for_each_resource_iterator_func_t = Option<
    unsafe extern "C" fn(resource: *mut wl_resource, user_data: *mut c_void) -> wl_iterator_result,
>;

pub type wl_display_global_filter_func_t = Option<
    unsafe extern "C" fn(
        client: *const wl_client,
        global: *const wl_global,
        data: *mut c_void,
    ) -> bool,
>;

pub type wl_event_loop_fd_func_t =
    Option<unsafe extern "C" fn(fd: c_int, mask: u32, data: *mut c_void) -> c_int>;

pub type wl_event_loop_idle_func_t = Option<unsafe extern "C" fn(data: *mut c_void)>;

pub type wl_event_loop_signal_func_t =
    Option<unsafe extern "C" fn(signal_number: c_int, data: *mut c_void) -> c_int>;

pub type wl_event_loop_timer_func_t = Option<unsafe extern "C" fn(data: *mut c_void) -> c_int>;

pub type wl_global_bind_func_t =
    Option<unsafe extern "C" fn(client: *mut wl_client, data: *mut c_void, version: u32, id: u32)>;

pub type wl_notify_func_t =
    Option<unsafe extern "C" fn(listener: *mut wl_listener, data: *mut c_void)>;

pub type wl_protocol_logger_func_t = Option<
    unsafe extern "C" fn(
        user_data: *mut c_void,
        direction: wl_protocol_logger_type,
        message: *const wl_protocol_logger_message,
    ),
>;

pub type wl_protocol_logger_type = c_int;
pub type wl_resource_destroy_func_t = Option<unsafe extern "C" fn(resource: *mut wl_resource)>;
pub type wl_user_data_destroy_func_t = Option<unsafe extern "C" fn(data: *mut c_void)>;
