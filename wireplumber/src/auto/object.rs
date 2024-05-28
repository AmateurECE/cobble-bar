// This file was generated by gir (https://github.com/gtk-rs/gir)
// from /usr/share/gir-1.0
// from ../gir-1.0
// DO NOT EDIT

use crate::{Core, ObjectFeatures};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, pin::Pin};

glib::wrapper! {
    #[doc(alias = "WpObject")]
    pub struct Object(Object<ffi::WpObject, ffi::WpObjectClass>);

    match fn {
        type_ => || ffi::wp_object_get_type(),
    }
}

impl Object {
    pub const NONE: Option<&'static Object> = None;
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::Object>> Sealed for T {}
}

pub trait ObjectExt: IsA<Object> + sealed::Sealed + 'static {
    #[doc(alias = "wp_object_abort_activation")]
    fn abort_activation(&self, msg: &str) {
        unsafe {
            ffi::wp_object_abort_activation(self.as_ref().to_glib_none().0, msg.to_glib_none().0);
        }
    }

    #[doc(alias = "wp_object_activate")]
    fn activate<P: FnOnce(Result<(), glib::Error>) + 'static>(
        &self,
        features: ObjectFeatures,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
        callback: P,
    ) {
        let main_context = glib::MainContext::ref_thread_default();
        let is_main_context_owner = main_context.is_owner();
        let has_acquired_main_context = (!is_main_context_owner)
            .then(|| main_context.acquire().ok())
            .flatten();
        assert!(
            is_main_context_owner || has_acquired_main_context.is_some(),
            "Async operations only allowed if the thread is owning the MainContext"
        );

        let user_data: Box_<glib::thread_guard::ThreadGuard<P>> =
            Box_::new(glib::thread_guard::ThreadGuard::new(callback));
        unsafe extern "C" fn activate_trampoline<P: FnOnce(Result<(), glib::Error>) + 'static>(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = std::ptr::null_mut();
            let _ = ffi::wp_object_activate_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = activate_trampoline::<P>;
        unsafe {
            ffi::wp_object_activate(
                self.as_ref().to_glib_none().0,
                features,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn activate_future(
        &self,
        features: ObjectFeatures,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {
        Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.activate(features, Some(cancellable), move |res| {
                send.resolve(res);
            });
        }))
    }

    #[doc(alias = "wp_object_activate_closure")]
    fn activate_closure(
        &self,
        features: ObjectFeatures,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
        closure: glib::Closure,
    ) {
        unsafe {
            ffi::wp_object_activate_closure(
                self.as_ref().to_glib_none().0,
                features,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                closure.into_glib_ptr(),
            );
        }
    }

    #[doc(alias = "wp_object_deactivate")]
    fn deactivate(&self, features: ObjectFeatures) {
        unsafe {
            ffi::wp_object_deactivate(self.as_ref().to_glib_none().0, features);
        }
    }

    #[doc(alias = "wp_object_get_active_features")]
    #[doc(alias = "get_active_features")]
    fn active_features(&self) -> ObjectFeatures {
        unsafe { ffi::wp_object_get_active_features(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "wp_object_get_core")]
    #[doc(alias = "get_core")]
    fn core(&self) -> Option<Core> {
        unsafe { from_glib_full(ffi::wp_object_get_core(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "wp_object_get_id")]
    #[doc(alias = "get_id")]
    fn id(&self) -> u32 {
        unsafe { ffi::wp_object_get_id(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "wp_object_get_supported_features")]
    #[doc(alias = "get_supported_features")]
    fn supported_features(&self) -> ObjectFeatures {
        unsafe { ffi::wp_object_get_supported_features(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "wp_object_test_active_features")]
    fn test_active_features(&self, features: ObjectFeatures) -> bool {
        unsafe {
            from_glib(ffi::wp_object_test_active_features(
                self.as_ref().to_glib_none().0,
                features,
            ))
        }
    }

    #[doc(alias = "wp_object_test_supported_features")]
    fn test_supported_features(&self, features: ObjectFeatures) -> bool {
        unsafe {
            from_glib(ffi::wp_object_test_supported_features(
                self.as_ref().to_glib_none().0,
                features,
            ))
        }
    }

    #[doc(alias = "wp_object_update_features")]
    fn update_features(&self, activated: ObjectFeatures, deactivated: ObjectFeatures) {
        unsafe {
            ffi::wp_object_update_features(self.as_ref().to_glib_none().0, activated, deactivated);
        }
    }

    #[doc(alias = "active-features")]
    fn connect_active_features_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_active_features_trampoline<
            P: IsA<Object>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WpObject,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Object::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::active-features\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_active_features_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "id")]
    fn connect_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_id_trampoline<P: IsA<Object>, F: Fn(&P) + 'static>(
            this: *mut ffi::WpObject,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Object::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::id\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_id_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "supported-features")]
    fn connect_supported_features_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_supported_features_trampoline<
            P: IsA<Object>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WpObject,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Object::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::supported-features\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_supported_features_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<Object>> ObjectExt for O {}
