// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use MenuModel;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct MenuLinkIter(Object<ffi::GMenuLinkIter, ffi::GMenuLinkIterClass>);

    match fn {
        get_type => || ffi::g_menu_link_iter_get_type(),
    }
}

pub trait MenuLinkIterExt {
    fn get_name(&self) -> Option<String>;

    fn get_next(&self) -> Option<(String, MenuModel)>;

    fn get_value(&self) -> Option<MenuModel>;

    fn next(&self) -> bool;
}

impl<O: IsA<MenuLinkIter>> MenuLinkIterExt for O {
    fn get_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_menu_link_iter_get_name(self.to_glib_none().0))
        }
    }

    fn get_next(&self) -> Option<(String, MenuModel)> {
        unsafe {
            let mut out_link = ptr::null();
            let mut value = ptr::null_mut();
            let ret = from_glib(ffi::g_menu_link_iter_get_next(self.to_glib_none().0, &mut out_link, &mut value));
            if ret { Some((from_glib_none(out_link), from_glib_full(value))) } else { None }
        }
    }

    fn get_value(&self) -> Option<MenuModel> {
        unsafe {
            from_glib_full(ffi::g_menu_link_iter_get_value(self.to_glib_none().0))
        }
    }

    fn next(&self) -> bool {
        unsafe {
            from_glib(ffi::g_menu_link_iter_next(self.to_glib_none().0))
        }
    }
}
