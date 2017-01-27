// This file was generated by gir (8a0b5e5) from gir-files (71d73f0)
// DO NOT EDIT

use MenuModel;
use ffi;
use glib::translate::*;

glib_wrapper! {
    pub struct MenuLinkIter(Object<ffi::GMenuLinkIter>);

    match fn {
        get_type => || ffi::g_menu_link_iter_get_type(),
    }
}

impl MenuLinkIter {
    pub fn get_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_menu_link_iter_get_name(self.to_glib_none().0))
        }
    }

    //pub fn get_next(&self, out_link: /*Unimplemented*/String) -> Option<MenuModel> {
    //    unsafe { TODO: call ffi::g_menu_link_iter_get_next() }
    //}

    pub fn get_value(&self) -> Option<MenuModel> {
        unsafe {
            from_glib_full(ffi::g_menu_link_iter_get_value(self.to_glib_none().0))
        }
    }

    pub fn next(&self) -> bool {
        unsafe {
            from_glib(ffi::g_menu_link_iter_next(self.to_glib_none().0))
        }
    }
}
