// This file was generated by gir (https://github.com/gtk-rs/gir)
// from /usr/share/gir-1.0
// from ../gir-1.0
// DO NOT EDIT

use crate::{Core, Object, Properties};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "WpNode")]
    pub struct Node(Object<ffi::WpNode, ffi::WpNodeClass>) @extends Object;

    match fn {
        type_ => || ffi::wp_node_get_type(),
    }
}

impl Node {
    #[doc(alias = "wp_node_new_from_factory")]
    #[doc(alias = "new_from_factory")]
    pub fn from_factory(
        core: &Core,
        factory_name: &str,
        properties: Option<Properties>,
    ) -> Option<Node> {
        unsafe {
            from_glib_full(ffi::wp_node_new_from_factory(
                core.to_glib_none().0,
                factory_name.to_glib_none().0,
                properties.into_glib_ptr(),
            ))
        }
    }

    #[doc(alias = "wp_node_get_n_input_ports")]
    #[doc(alias = "get_n_input_ports")]
    pub fn n_input_ports(&self) -> (u32, u32) {
        unsafe {
            let mut max = std::mem::MaybeUninit::uninit();
            let ret = ffi::wp_node_get_n_input_ports(self.to_glib_none().0, max.as_mut_ptr());
            (ret, max.assume_init())
        }
    }

    #[doc(alias = "wp_node_get_n_output_ports")]
    #[doc(alias = "get_n_output_ports")]
    pub fn n_output_ports(&self) -> (u32, u32) {
        unsafe {
            let mut max = std::mem::MaybeUninit::uninit();
            let ret = ffi::wp_node_get_n_output_ports(self.to_glib_none().0, max.as_mut_ptr());
            (ret, max.assume_init())
        }
    }

    #[doc(alias = "wp_node_get_n_ports")]
    #[doc(alias = "get_n_ports")]
    pub fn n_ports(&self) -> u32 {
        unsafe { ffi::wp_node_get_n_ports(self.to_glib_none().0) }
    }

    //#[doc(alias = "wp_node_get_state")]
    //#[doc(alias = "get_state")]
    //pub fn state(&self) -> (/*Ignored*/NodeState, glib::GString) {
    //    unsafe { TODO: call ffi:wp_node_get_state() }
    //}

    //#[doc(alias = "wp_node_lookup_port")]
    //pub fn lookup_port(&self, : /*Unknown conversion*//*Unimplemented*/Basic: VarArgs) -> /*Ignored*/Option<Port> {
    //    unsafe { TODO: call ffi:wp_node_lookup_port() }
    //}

    //#[doc(alias = "wp_node_lookup_port_full")]
    //pub fn lookup_port_full(&self, interest: ObjectInterest) -> /*Ignored*/Option<Port> {
    //    unsafe { TODO: call ffi:wp_node_lookup_port_full() }
    //}

    //#[doc(alias = "wp_node_new_ports_filtered_iterator")]
    //pub fn new_ports_filtered_iterator(&self, : /*Unknown conversion*//*Unimplemented*/Basic: VarArgs) -> /*Ignored*/Option<Iterator> {
    //    unsafe { TODO: call ffi:wp_node_new_ports_filtered_iterator() }
    //}

    //#[doc(alias = "wp_node_new_ports_filtered_iterator_full")]
    //pub fn new_ports_filtered_iterator_full(&self, interest: ObjectInterest) -> /*Ignored*/Option<Iterator> {
    //    unsafe { TODO: call ffi:wp_node_new_ports_filtered_iterator_full() }
    //}

    //#[doc(alias = "wp_node_new_ports_iterator")]
    //pub fn new_ports_iterator(&self) -> /*Ignored*/Option<Iterator> {
    //    unsafe { TODO: call ffi:wp_node_new_ports_iterator() }
    //}

    #[doc(alias = "wp_node_send_command")]
    pub fn send_command(&self, command: &str) {
        unsafe {
            ffi::wp_node_send_command(self.to_glib_none().0, command.to_glib_none().0);
        }
    }

    #[doc(alias = "max-input-ports")]
    pub fn max_input_ports(&self) -> u32 {
        ObjectExt::property(self, "max-input-ports")
    }

    #[doc(alias = "max-output-ports")]
    pub fn max_output_ports(&self) -> u32 {
        ObjectExt::property(self, "max-output-ports")
    }

    #[doc(alias = "ports-changed")]
    pub fn connect_ports_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn ports_changed_trampoline<F: Fn(&Node) + 'static>(
            this: *mut ffi::WpNode,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"ports-changed\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    ports_changed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    //#[doc(alias = "state-changed")]
    //pub fn connect_state_changed<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Ignored object: Wp.NodeState
    //    Ignored p0: Wp.NodeState
    //}

    #[doc(alias = "max-input-ports")]
    pub fn connect_max_input_ports_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_max_input_ports_trampoline<F: Fn(&Node) + 'static>(
            this: *mut ffi::WpNode,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::max-input-ports\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_max_input_ports_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "max-output-ports")]
    pub fn connect_max_output_ports_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_max_output_ports_trampoline<F: Fn(&Node) + 'static>(
            this: *mut ffi::WpNode,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::max-output-ports\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_max_output_ports_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "n-input-ports")]
    pub fn connect_n_input_ports_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_n_input_ports_trampoline<F: Fn(&Node) + 'static>(
            this: *mut ffi::WpNode,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::n-input-ports\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_n_input_ports_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "n-output-ports")]
    pub fn connect_n_output_ports_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_n_output_ports_trampoline<F: Fn(&Node) + 'static>(
            this: *mut ffi::WpNode,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::n-output-ports\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_n_output_ports_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "state")]
    pub fn connect_state_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_state_trampoline<F: Fn(&Node) + 'static>(
            this: *mut ffi::WpNode,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::state\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    notify_state_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}