// This file was generated by gir (https://github.com/gtk-rs/gir)
// from /usr/share/gir-1.0
// from ../gir-1.0
// DO NOT EDIT

use crate::{ConstraintType, ConstraintVerb};
use glib::translate::*;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ObjectInterest(Shared<ffi::WpObjectInterest>);

    match fn {
        ref => |ptr| ffi::wp_object_interest_ref(ptr),
        unref => |ptr| ffi::wp_object_interest_unref(ptr),
        type_ => || ffi::wp_object_interest_get_type(),
    }
}

impl ObjectInterest {
    //#[doc(alias = "wp_object_interest_new")]
    //pub fn new(gtype: glib::types::Type, : /*Unknown conversion*//*Unimplemented*/Basic: VarArgs) -> ObjectInterest {
    //    unsafe { TODO: call ffi:wp_object_interest_new() }
    //}

    #[doc(alias = "wp_object_interest_new_type")]
    pub fn new_type(gtype: glib::types::Type) -> ObjectInterest {
        unsafe { from_glib_full(ffi::wp_object_interest_new_type(gtype.into_glib())) }
    }

    //#[doc(alias = "wp_object_interest_new_valist")]
    //pub fn new_valist(gtype: glib::types::Type, args: /*Unknown conversion*//*Unimplemented*/Unsupported) -> ObjectInterest {
    //    unsafe { TODO: call ffi:wp_object_interest_new_valist() }
    //}

    #[doc(alias = "wp_object_interest_add_constraint")]
    pub fn add_constraint(
        &self,
        type_: ConstraintType,
        subject: &str,
        verb: ConstraintVerb,
        value: Option<&glib::Variant>,
    ) {
        unsafe {
            ffi::wp_object_interest_add_constraint(
                self.to_glib_none().0,
                type_.into_glib(),
                subject.to_glib_none().0,
                verb.into_glib(),
                value.to_glib_none().0,
            );
        }
    }

    //#[doc(alias = "wp_object_interest_matches")]
    //pub fn matches(&self, object: /*Unimplemented*/Option<Basic: Pointer>) -> bool {
    //    unsafe { TODO: call ffi:wp_object_interest_matches() }
    //}

    //#[doc(alias = "wp_object_interest_matches_full")]
    //pub fn matches_full(&self, flags: /*Ignored*/InterestMatchFlags, object_type: glib::types::Type, object: Option<&impl IsA<glib::Object>>, pw_props: Option<&Properties>, pw_global_props: Option<&Properties>) -> /*Ignored*/InterestMatch {
    //    unsafe { TODO: call ffi:wp_object_interest_matches_full() }
    //}

    #[doc(alias = "wp_object_interest_validate")]
    pub fn validate(&self) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::wp_object_interest_validate(self.to_glib_none().0, &mut error);
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}
