// This file was generated by gir (https://github.com/gtk-rs/gir)
// from /usr/share/gir-1.0
// from ../gir-1.0
// DO NOT EDIT

use glib::translate::*;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct SpaJson(Shared<ffi::WpSpaJson>);

    match fn {
        ref => |ptr| ffi::wp_spa_json_ref(ptr),
        unref => |ptr| ffi::wp_spa_json_unref(ptr),
        type_ => || ffi::wp_spa_json_get_type(),
    }
}

impl SpaJson {
    //#[doc(alias = "wp_spa_json_new_array")]
    //pub fn new_array(format: Option<&str>, : /*Unknown conversion*//*Unimplemented*/Basic: VarArgs) -> SpaJson {
    //    unsafe { TODO: call ffi:wp_spa_json_new_array() }
    //}

    //#[doc(alias = "wp_spa_json_new_array_valist")]
    //pub fn new_array_valist(format: Option<&str>, args: /*Unknown conversion*//*Unimplemented*/Unsupported) -> SpaJson {
    //    unsafe { TODO: call ffi:wp_spa_json_new_array_valist() }
    //}

    #[doc(alias = "wp_spa_json_new_boolean")]
    pub fn new_boolean(value: bool) -> SpaJson {
        unsafe { from_glib_full(ffi::wp_spa_json_new_boolean(value.into_glib())) }
    }

    #[doc(alias = "wp_spa_json_new_float")]
    pub fn new_float(value: f32) -> SpaJson {
        unsafe { from_glib_full(ffi::wp_spa_json_new_float(value)) }
    }

    #[doc(alias = "wp_spa_json_new_from_string")]
    #[doc(alias = "new_from_string")]
    pub fn from_string(json_str: &str) -> SpaJson {
        unsafe { from_glib_full(ffi::wp_spa_json_new_from_string(json_str.to_glib_none().0)) }
    }

    #[doc(alias = "wp_spa_json_new_from_stringn")]
    #[doc(alias = "new_from_stringn")]
    pub fn from_stringn(json_str: &str) -> SpaJson {
        let len = json_str.len() as _;
        unsafe {
            from_glib_full(ffi::wp_spa_json_new_from_stringn(
                json_str.to_glib_none().0,
                len,
            ))
        }
    }

    #[doc(alias = "wp_spa_json_new_int")]
    pub fn new_int(value: i32) -> SpaJson {
        unsafe { from_glib_full(ffi::wp_spa_json_new_int(value)) }
    }

    #[doc(alias = "wp_spa_json_new_null")]
    pub fn new_null() -> SpaJson {
        unsafe { from_glib_full(ffi::wp_spa_json_new_null()) }
    }

    //#[doc(alias = "wp_spa_json_new_object")]
    //pub fn new_object(key: Option<&str>, format: Option<&str>, : /*Unknown conversion*//*Unimplemented*/Basic: VarArgs) -> SpaJson {
    //    unsafe { TODO: call ffi:wp_spa_json_new_object() }
    //}

    //#[doc(alias = "wp_spa_json_new_object_valist")]
    //pub fn new_object_valist(key: Option<&str>, format: Option<&str>, args: /*Unknown conversion*//*Unimplemented*/Unsupported) -> SpaJson {
    //    unsafe { TODO: call ffi:wp_spa_json_new_object_valist() }
    //}

    #[doc(alias = "wp_spa_json_new_string")]
    pub fn new_string(value: &str) -> SpaJson {
        unsafe { from_glib_full(ffi::wp_spa_json_new_string(value.to_glib_none().0)) }
    }

    //#[doc(alias = "wp_spa_json_new_wrap")]
    //pub fn new_wrap(json: /*Ignored*/Option<&mut libspa::spa_json>) -> SpaJson {
    //    unsafe { TODO: call ffi:wp_spa_json_new_wrap() }
    //}

    #[doc(alias = "wp_spa_json_new_wrap_string")]
    pub fn new_wrap_string(json_str: &str) -> SpaJson {
        unsafe { from_glib_full(ffi::wp_spa_json_new_wrap_string(json_str.to_glib_none().0)) }
    }

    #[doc(alias = "wp_spa_json_new_wrap_stringn")]
    pub fn new_wrap_stringn(json_str: &str) -> SpaJson {
        let len = json_str.len() as _;
        unsafe {
            from_glib_full(ffi::wp_spa_json_new_wrap_stringn(
                json_str.to_glib_none().0,
                len,
            ))
        }
    }

    #[doc(alias = "wp_spa_json_copy")]
    #[must_use]
    pub fn copy(&self) -> Option<SpaJson> {
        unsafe { from_glib_full(ffi::wp_spa_json_copy(self.to_glib_none().0)) }
    }

    #[doc(alias = "wp_spa_json_ensure_unique_owner")]
    #[must_use]
    pub fn ensure_unique_owner(self) -> Option<SpaJson> {
        unsafe { from_glib_full(ffi::wp_spa_json_ensure_unique_owner(self.into_glib_ptr())) }
    }

    #[doc(alias = "wp_spa_json_get_data")]
    #[doc(alias = "get_data")]
    pub fn data(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::wp_spa_json_get_data(self.to_glib_none().0)) }
    }

    #[doc(alias = "wp_spa_json_get_size")]
    #[doc(alias = "get_size")]
    pub fn size(&self) -> usize {
        unsafe { ffi::wp_spa_json_get_size(self.to_glib_none().0) }
    }

    //#[doc(alias = "wp_spa_json_get_spa_json")]
    //#[doc(alias = "get_spa_json")]
    //pub fn spa_json(&self) -> /*Ignored*/Option<libspa::spa_json> {
    //    unsafe { TODO: call ffi:wp_spa_json_get_spa_json() }
    //}

    #[doc(alias = "wp_spa_json_is_array")]
    pub fn is_array(&self) -> bool {
        unsafe { from_glib(ffi::wp_spa_json_is_array(self.to_glib_none().0)) }
    }

    #[doc(alias = "wp_spa_json_is_boolean")]
    pub fn is_boolean(&self) -> bool {
        unsafe { from_glib(ffi::wp_spa_json_is_boolean(self.to_glib_none().0)) }
    }

    #[doc(alias = "wp_spa_json_is_container")]
    pub fn is_container(&self) -> bool {
        unsafe { from_glib(ffi::wp_spa_json_is_container(self.to_glib_none().0)) }
    }

    #[doc(alias = "wp_spa_json_is_float")]
    pub fn is_float(&self) -> bool {
        unsafe { from_glib(ffi::wp_spa_json_is_float(self.to_glib_none().0)) }
    }

    #[doc(alias = "wp_spa_json_is_int")]
    pub fn is_int(&self) -> bool {
        unsafe { from_glib(ffi::wp_spa_json_is_int(self.to_glib_none().0)) }
    }

    #[doc(alias = "wp_spa_json_is_null")]
    pub fn is_null(&self) -> bool {
        unsafe { from_glib(ffi::wp_spa_json_is_null(self.to_glib_none().0)) }
    }

    #[doc(alias = "wp_spa_json_is_object")]
    pub fn is_object(&self) -> bool {
        unsafe { from_glib(ffi::wp_spa_json_is_object(self.to_glib_none().0)) }
    }

    #[doc(alias = "wp_spa_json_is_string")]
    pub fn is_string(&self) -> bool {
        unsafe { from_glib(ffi::wp_spa_json_is_string(self.to_glib_none().0)) }
    }

    #[doc(alias = "wp_spa_json_is_unique_owner")]
    pub fn is_unique_owner(&self) -> bool {
        unsafe { from_glib(ffi::wp_spa_json_is_unique_owner(self.to_glib_none().0)) }
    }

    //#[doc(alias = "wp_spa_json_new_iterator")]
    //pub fn new_iterator(&self) -> /*Ignored*/Option<Iterator> {
    //    unsafe { TODO: call ffi:wp_spa_json_new_iterator() }
    //}

    //#[doc(alias = "wp_spa_json_object_get")]
    //pub fn object_get(&self, : /*Unknown conversion*//*Unimplemented*/Basic: VarArgs) -> bool {
    //    unsafe { TODO: call ffi:wp_spa_json_object_get() }
    //}

    //#[doc(alias = "wp_spa_json_object_get_valist")]
    //pub fn object_get_valist(&self, args: /*Unknown conversion*//*Unimplemented*/Unsupported) -> bool {
    //    unsafe { TODO: call ffi:wp_spa_json_object_get_valist() }
    //}

    //#[doc(alias = "wp_spa_json_parse_array")]
    //pub fn parse_array(&self, : /*Unknown conversion*//*Unimplemented*/Basic: VarArgs) -> bool {
    //    unsafe { TODO: call ffi:wp_spa_json_parse_array() }
    //}

    //#[doc(alias = "wp_spa_json_parse_array_valist")]
    //pub fn parse_array_valist(&self, args: /*Unknown conversion*//*Unimplemented*/&mut Unsupported) -> bool {
    //    unsafe { TODO: call ffi:wp_spa_json_parse_array_valist() }
    //}

    #[doc(alias = "wp_spa_json_parse_boolean")]
    pub fn parse_boolean(&self) -> Option<bool> {
        unsafe {
            let mut value = std::mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::wp_spa_json_parse_boolean(
                self.to_glib_none().0,
                value.as_mut_ptr(),
            ));
            if ret {
                Some(from_glib(value.assume_init()))
            } else {
                None
            }
        }
    }

    #[doc(alias = "wp_spa_json_parse_float")]
    pub fn parse_float(&self) -> Option<f32> {
        unsafe {
            let mut value = std::mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::wp_spa_json_parse_float(
                self.to_glib_none().0,
                value.as_mut_ptr(),
            ));
            if ret {
                Some(value.assume_init())
            } else {
                None
            }
        }
    }

    #[doc(alias = "wp_spa_json_parse_int")]
    pub fn parse_int(&self) -> Option<i32> {
        unsafe {
            let mut value = std::mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::wp_spa_json_parse_int(
                self.to_glib_none().0,
                value.as_mut_ptr(),
            ));
            if ret {
                Some(value.assume_init())
            } else {
                None
            }
        }
    }

    //#[doc(alias = "wp_spa_json_parse_object")]
    //pub fn parse_object(&self, : /*Unknown conversion*//*Unimplemented*/Basic: VarArgs) -> bool {
    //    unsafe { TODO: call ffi:wp_spa_json_parse_object() }
    //}

    //#[doc(alias = "wp_spa_json_parse_object_valist")]
    //pub fn parse_object_valist(&self, args: /*Unknown conversion*//*Unimplemented*/&mut Unsupported) -> bool {
    //    unsafe { TODO: call ffi:wp_spa_json_parse_object_valist() }
    //}

    #[doc(alias = "wp_spa_json_parse_string")]
    pub fn parse_string(&self) -> Option<glib::GString> {
        unsafe { from_glib_full(ffi::wp_spa_json_parse_string(self.to_glib_none().0)) }
    }

    #[doc(alias = "wp_spa_json_to_string")]
    #[doc(alias = "to_string")]
    pub fn to_str(&self) -> glib::GString {
        unsafe { from_glib_full(ffi::wp_spa_json_to_string(self.to_glib_none().0)) }
    }
}

impl std::fmt::Display for SpaJson {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(&self.to_str())
    }
}