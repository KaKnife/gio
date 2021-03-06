// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct FilenameCompleter(Object<ffi::GFilenameCompleter, ffi::GFilenameCompleterClass>);

    match fn {
        get_type => || ffi::g_filename_completer_get_type(),
    }
}

impl FilenameCompleter {
    pub fn new() -> FilenameCompleter {
        unsafe {
            from_glib_full(ffi::g_filename_completer_new())
        }
    }
}

impl Default for FilenameCompleter {
    fn default() -> Self {
        Self::new()
    }
}

pub trait FilenameCompleterExt {
    fn get_completion_suffix(&self, initial_text: &str) -> Option<String>;

    fn get_completions(&self, initial_text: &str) -> Vec<String>;

    fn set_dirs_only(&self, dirs_only: bool);

    fn connect_got_completion_data<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<FilenameCompleter> + IsA<glib::object::Object>> FilenameCompleterExt for O {
    fn get_completion_suffix(&self, initial_text: &str) -> Option<String> {
        unsafe {
            from_glib_full(ffi::g_filename_completer_get_completion_suffix(self.to_glib_none().0, initial_text.to_glib_none().0))
        }
    }

    fn get_completions(&self, initial_text: &str) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::g_filename_completer_get_completions(self.to_glib_none().0, initial_text.to_glib_none().0))
        }
    }

    fn set_dirs_only(&self, dirs_only: bool) {
        unsafe {
            ffi::g_filename_completer_set_dirs_only(self.to_glib_none().0, dirs_only.to_glib());
        }
    }

    fn connect_got_completion_data<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "got-completion-data",
                transmute(got_completion_data_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn got_completion_data_trampoline<P>(this: *mut ffi::GFilenameCompleter, f: glib_ffi::gpointer)
where P: IsA<FilenameCompleter> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&FilenameCompleter::from_glib_borrow(this).downcast_unchecked())
}
