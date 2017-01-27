// This file was generated by gir (8a0b5e5) from gir-files (71d73f0)
// DO NOT EDIT

use ffi;
use glib;
use glib::object::IsA;
use glib::translate::*;

glib_wrapper! {
    pub struct Action(Object<ffi::GAction>);

    match fn {
        get_type => || ffi::g_action_get_type(),
    }
}

pub trait ActionExt {
    fn activate(&self, parameter: Option<&glib::Variant>);

    fn change_state(&self, value: &glib::Variant);

    fn get_enabled(&self) -> bool;

    fn get_name(&self) -> Option<String>;

    fn get_parameter_type(&self) -> Option<glib::VariantType>;

    fn get_state(&self) -> Option<glib::Variant>;

    fn get_state_hint(&self) -> Option<glib::Variant>;

    fn get_state_type(&self) -> Option<glib::VariantType>;
}

impl<O: IsA<Action>> ActionExt for O {
    fn activate(&self, parameter: Option<&glib::Variant>) {
        unsafe {
            ffi::g_action_activate(self.to_glib_none().0, parameter.to_glib_none().0);
        }
    }

    fn change_state(&self, value: &glib::Variant) {
        unsafe {
            ffi::g_action_change_state(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn get_enabled(&self) -> bool {
        unsafe {
            from_glib(ffi::g_action_get_enabled(self.to_glib_none().0))
        }
    }

    fn get_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_action_get_name(self.to_glib_none().0))
        }
    }

    fn get_parameter_type(&self) -> Option<glib::VariantType> {
        unsafe {
            from_glib_none(ffi::g_action_get_parameter_type(self.to_glib_none().0))
        }
    }

    fn get_state(&self) -> Option<glib::Variant> {
        unsafe {
            from_glib_full(ffi::g_action_get_state(self.to_glib_none().0))
        }
    }

    fn get_state_hint(&self) -> Option<glib::Variant> {
        unsafe {
            from_glib_full(ffi::g_action_get_state_hint(self.to_glib_none().0))
        }
    }

    fn get_state_type(&self) -> Option<glib::VariantType> {
        unsafe {
            from_glib_none(ffi::g_action_get_state_type(self.to_glib_none().0))
        }
    }
}
