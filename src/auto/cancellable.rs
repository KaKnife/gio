// This file was generated by gir (https://github.com/gtk-rs/gir @ 8b9d0bb)
// from gir-files (https://github.com/gtk-rs/gir-files @ 77d1f70)
// DO NOT EDIT

use Error;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use libc;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Cancellable(Object<ffi::GCancellable, ffi::GCancellableClass>);

    match fn {
        get_type => || ffi::g_cancellable_get_type(),
    }
}

impl Cancellable {
    pub fn new() -> Cancellable {
        unsafe {
            from_glib_full(ffi::g_cancellable_new())
        }
    }

    pub fn get_current() -> Option<Cancellable> {
        unsafe {
            from_glib_none(ffi::g_cancellable_get_current())
        }
    }
}

impl Default for Cancellable {
    fn default() -> Self {
        Self::new()
    }
}

pub trait CancellableExt {
    fn cancel(&self);

    //fn connect<'a, P: Into<Option</*Unimplemented*/Fundamental: Pointer>>, Q: Into<Option<&'a /*Ignored*/glib::DestroyNotify>>>(&self, callback: /*Unknown conversion*//*Unimplemented*/Callback, data: P, data_destroy_func: Q) -> libc::c_ulong;

    fn disconnect(&self, handler_id: libc::c_ulong);

    fn get_fd(&self) -> i32;

    fn is_cancelled(&self) -> bool;

    //fn make_pollfd(&self, pollfd: /*Ignored*/&mut glib::PollFD) -> bool;

    fn pop_current(&self);

    fn push_current(&self);

    fn release_fd(&self);

    fn reset(&self);

    fn set_error_if_cancelled(&self) -> Result<(), Error>;

    fn source_new(&self) -> Option<glib::Source>;

    fn connect_cancelled<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Cancellable> + IsA<glib::object::Object>> CancellableExt for O {
    fn cancel(&self) {
        unsafe {
            ffi::g_cancellable_cancel(self.to_glib_none().0);
        }
    }

    //fn connect<'a, P: Into<Option</*Unimplemented*/Fundamental: Pointer>>, Q: Into<Option<&'a /*Ignored*/glib::DestroyNotify>>>(&self, callback: /*Unknown conversion*//*Unimplemented*/Callback, data: P, data_destroy_func: Q) -> libc::c_ulong {
    //    unsafe { TODO: call ffi::g_cancellable_connect() }
    //}

    fn disconnect(&self, handler_id: libc::c_ulong) {
        unsafe {
            ffi::g_cancellable_disconnect(self.to_glib_none().0, handler_id);
        }
    }

    fn get_fd(&self) -> i32 {
        unsafe {
            ffi::g_cancellable_get_fd(self.to_glib_none().0)
        }
    }

    fn is_cancelled(&self) -> bool {
        unsafe {
            from_glib(ffi::g_cancellable_is_cancelled(self.to_glib_none().0))
        }
    }

    //fn make_pollfd(&self, pollfd: /*Ignored*/&mut glib::PollFD) -> bool {
    //    unsafe { TODO: call ffi::g_cancellable_make_pollfd() }
    //}

    fn pop_current(&self) {
        unsafe {
            ffi::g_cancellable_pop_current(self.to_glib_none().0);
        }
    }

    fn push_current(&self) {
        unsafe {
            ffi::g_cancellable_push_current(self.to_glib_none().0);
        }
    }

    fn release_fd(&self) {
        unsafe {
            ffi::g_cancellable_release_fd(self.to_glib_none().0);
        }
    }

    fn reset(&self) {
        unsafe {
            ffi::g_cancellable_reset(self.to_glib_none().0);
        }
    }

    fn set_error_if_cancelled(&self) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_cancellable_set_error_if_cancelled(self.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn source_new(&self) -> Option<glib::Source> {
        unsafe {
            from_glib_full(ffi::g_cancellable_source_new(self.to_glib_none().0))
        }
    }

    fn connect_cancelled<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "cancelled",
                transmute(cancelled_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn cancelled_trampoline<P>(this: *mut ffi::GCancellable, f: glib_ffi::gpointer)
where P: IsA<Cancellable> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Cancellable::from_glib_borrow(this).downcast_unchecked())
}
