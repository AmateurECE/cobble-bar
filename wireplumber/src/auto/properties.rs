// This file was generated by gir (https://github.com/gtk-rs/gir)
// from /usr/share/gir-1.0
// from ../gir-1.0
// DO NOT EDIT

use crate::SpaJson;
use glib::translate::*;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Properties(Shared<ffi::WpProperties>);

    match fn {
        ref => |ptr| ffi::wp_properties_ref(ptr),
        unref => |ptr| ffi::wp_properties_unref(ptr),
        type_ => || ffi::wp_properties_get_type(),
    }
}

impl Properties {
    //#[doc(alias = "wp_properties_new")]
    //pub fn new(key: &str, : /*Unknown conversion*//*Unimplemented*/Basic: VarArgs) -> Properties {
    //    unsafe { TODO: call ffi:wp_properties_new() }
    //}

    //#[doc(alias = "wp_properties_new_copy")]
    //pub fn new_copy(props: /*Ignored*/Option<&pipewire::pw_properties>) -> Properties {
    //    unsafe { TODO: call ffi:wp_properties_new_copy() }
    //}

    //#[doc(alias = "wp_properties_new_copy_dict")]
    //pub fn new_copy_dict(dict: /*Ignored*/Option<&libspa::spa_dict>) -> Properties {
    //    unsafe { TODO: call ffi:wp_properties_new_copy_dict() }
    //}

    #[doc(alias = "wp_properties_new_empty")]
    pub fn new_empty() -> Properties {
        unsafe { from_glib_full(ffi::wp_properties_new_empty()) }
    }

    #[doc(alias = "wp_properties_new_json")]
    pub fn new_json(json: &SpaJson) -> Properties {
        unsafe { from_glib_full(ffi::wp_properties_new_json(json.to_glib_none().0)) }
    }

    #[doc(alias = "wp_properties_new_string")]
    pub fn new_string(str: &str) -> Properties {
        unsafe { from_glib_full(ffi::wp_properties_new_string(str.to_glib_none().0)) }
    }

    //#[doc(alias = "wp_properties_new_take")]
    //pub fn new_take(props: /*Ignored*/Option<&mut pipewire::pw_properties>) -> Properties {
    //    unsafe { TODO: call ffi:wp_properties_new_take() }
    //}

    //#[doc(alias = "wp_properties_new_valist")]
    //pub fn new_valist(key: &str, args: /*Unknown conversion*//*Unimplemented*/Unsupported) -> Properties {
    //    unsafe { TODO: call ffi:wp_properties_new_valist() }
    //}

    //#[doc(alias = "wp_properties_new_wrap")]
    //pub fn new_wrap(props: /*Ignored*/Option<&pipewire::pw_properties>) -> Properties {
    //    unsafe { TODO: call ffi:wp_properties_new_wrap() }
    //}

    //#[doc(alias = "wp_properties_new_wrap_dict")]
    //pub fn new_wrap_dict(dict: /*Ignored*/Option<&libspa::spa_dict>) -> Properties {
    //    unsafe { TODO: call ffi:wp_properties_new_wrap_dict() }
    //}

    #[doc(alias = "wp_properties_add")]
    pub fn add(&self, props: &Properties) -> i32 {
        unsafe { ffi::wp_properties_add(self.to_glib_none().0, props.to_glib_none().0) }
    }

    //#[doc(alias = "wp_properties_add_from_dict")]
    //pub fn add_from_dict(&self, dict: /*Ignored*/Option<&libspa::spa_dict>) -> i32 {
    //    unsafe { TODO: call ffi:wp_properties_add_from_dict() }
    //}

    //#[doc(alias = "wp_properties_add_keys")]
    //pub fn add_keys(&self, props: &Properties, key1: &str, : /*Unknown conversion*//*Unimplemented*/Basic: VarArgs) -> i32 {
    //    unsafe { TODO: call ffi:wp_properties_add_keys() }
    //}

    #[doc(alias = "wp_properties_add_keys_array")]
    pub fn add_keys_array(&self, props: &Properties, keys: &[&str]) -> i32 {
        unsafe {
            ffi::wp_properties_add_keys_array(
                self.to_glib_none().0,
                props.to_glib_none().0,
                keys.to_glib_none().0,
            )
        }
    }

    //#[doc(alias = "wp_properties_add_keys_from_dict")]
    //pub fn add_keys_from_dict(&self, dict: /*Ignored*/Option<&libspa::spa_dict>, key1: &str, : /*Unknown conversion*//*Unimplemented*/Basic: VarArgs) -> i32 {
    //    unsafe { TODO: call ffi:wp_properties_add_keys_from_dict() }
    //}

    #[doc(alias = "wp_properties_copy")]
    #[must_use]
    pub fn copy(&self) -> Option<Properties> {
        unsafe { from_glib_full(ffi::wp_properties_copy(self.to_glib_none().0)) }
    }

    #[doc(alias = "wp_properties_ensure_unique_owner")]
    #[must_use]
    pub fn ensure_unique_owner(self) -> Option<Properties> {
        unsafe { from_glib_full(ffi::wp_properties_ensure_unique_owner(self.into_glib_ptr())) }
    }

    #[doc(alias = "wp_properties_get")]
    pub fn get(&self, key: &str) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::wp_properties_get(
                self.to_glib_none().0,
                key.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "wp_properties_get_count")]
    #[doc(alias = "get_count")]
    pub fn count(&self) -> u32 {
        unsafe { ffi::wp_properties_get_count(self.to_glib_none().0) }
    }

    #[doc(alias = "wp_properties_matches")]
    pub fn matches(&self, other: &Properties) -> bool {
        unsafe {
            from_glib(ffi::wp_properties_matches(
                self.to_glib_none().0,
                other.to_glib_none().0,
            ))
        }
    }

    //#[doc(alias = "wp_properties_new_iterator")]
    //pub fn new_iterator(&self) -> /*Ignored*/Option<Iterator> {
    //    unsafe { TODO: call ffi:wp_properties_new_iterator() }
    //}

    //#[doc(alias = "wp_properties_peek_dict")]
    //pub fn peek_dict(&self) -> /*Ignored*/Option<libspa::spa_dict> {
    //    unsafe { TODO: call ffi:wp_properties_peek_dict() }
    //}

    #[doc(alias = "wp_properties_set")]
    pub fn set(&self, key: &str, value: Option<&str>) -> i32 {
        unsafe {
            ffi::wp_properties_set(
                self.to_glib_none().0,
                key.to_glib_none().0,
                value.to_glib_none().0,
            )
        }
    }

    //#[doc(alias = "wp_properties_setf")]
    //pub fn setf(&self, key: &str, format: &str, : /*Unknown conversion*//*Unimplemented*/Basic: VarArgs) -> i32 {
    //    unsafe { TODO: call ffi:wp_properties_setf() }
    //}

    //#[doc(alias = "wp_properties_setf_valist")]
    //pub fn setf_valist(&self, key: &str, format: &str, args: /*Unknown conversion*//*Unimplemented*/Unsupported) -> i32 {
    //    unsafe { TODO: call ffi:wp_properties_setf_valist() }
    //}

    #[doc(alias = "wp_properties_sort")]
    pub fn sort(&self) {
        unsafe {
            ffi::wp_properties_sort(self.to_glib_none().0);
        }
    }

    //#[doc(alias = "wp_properties_to_pw_properties")]
    //pub fn to_pw_properties(&self) -> /*Ignored*/Option<pipewire::pw_properties> {
    //    unsafe { TODO: call ffi:wp_properties_to_pw_properties() }
    //}

    //#[doc(alias = "wp_properties_unref_and_take_pw_properties")]
    //pub fn unref_and_take_pw_properties(self) -> /*Ignored*/Option<pipewire::pw_properties> {
    //    unsafe { TODO: call ffi:wp_properties_unref_and_take_pw_properties() }
    //}

    #[doc(alias = "wp_properties_update")]
    pub fn update(&self, props: &Properties) -> i32 {
        unsafe { ffi::wp_properties_update(self.to_glib_none().0, props.to_glib_none().0) }
    }

    //#[doc(alias = "wp_properties_update_from_dict")]
    //pub fn update_from_dict(&self, dict: /*Ignored*/Option<&libspa::spa_dict>) -> i32 {
    //    unsafe { TODO: call ffi:wp_properties_update_from_dict() }
    //}

    #[doc(alias = "wp_properties_update_from_json")]
    pub fn update_from_json(&self, json: &SpaJson) -> i32 {
        unsafe { ffi::wp_properties_update_from_json(self.to_glib_none().0, json.to_glib_none().0) }
    }

    //#[doc(alias = "wp_properties_update_keys")]
    //pub fn update_keys(&self, props: &Properties, key1: &str, : /*Unknown conversion*//*Unimplemented*/Basic: VarArgs) -> i32 {
    //    unsafe { TODO: call ffi:wp_properties_update_keys() }
    //}

    #[doc(alias = "wp_properties_update_keys_array")]
    pub fn update_keys_array(&self, props: &Properties, keys: &[&str]) -> i32 {
        unsafe {
            ffi::wp_properties_update_keys_array(
                self.to_glib_none().0,
                props.to_glib_none().0,
                keys.to_glib_none().0,
            )
        }
    }

    //#[doc(alias = "wp_properties_update_keys_from_dict")]
    //pub fn update_keys_from_dict(&self, dict: /*Ignored*/Option<&libspa::spa_dict>, key1: &str, : /*Unknown conversion*//*Unimplemented*/Basic: VarArgs) -> i32 {
    //    unsafe { TODO: call ffi:wp_properties_update_keys_from_dict() }
    //}
}