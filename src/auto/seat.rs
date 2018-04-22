// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v3_20", feature = "dox"))]
use Device;
#[cfg(any(feature = "v3_22", feature = "dox"))]
use DeviceTool;
#[cfg(any(feature = "v3_20", feature = "dox"))]
use Display;
#[cfg(any(feature = "v3_20", feature = "dox"))]
use SeatCapabilities;
use ffi;
use glib;
#[cfg(any(feature = "v3_20", feature = "dox"))]
use glib::object::Downcast;
use glib::object::IsA;
#[cfg(any(feature = "v3_20", feature = "dox"))]
use glib::signal::SignalHandlerId;
#[cfg(any(feature = "v3_20", feature = "dox"))]
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
#[cfg(any(feature = "v3_20", feature = "dox"))]
use std::boxed::Box as Box_;
use std::mem;
#[cfg(any(feature = "v3_20", feature = "dox"))]
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Seat(Object<ffi::GdkSeat>);

    match fn {
        get_type => || ffi::gdk_seat_get_type(),
    }
}

pub trait SeatExt {
    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn get_capabilities(&self) -> SeatCapabilities;

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn get_display(&self) -> Option<Display>;

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn get_keyboard(&self) -> Option<Device>;

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn get_pointer(&self) -> Option<Device>;

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn get_slaves(&self, capabilities: SeatCapabilities) -> Vec<Device>;

    //#[cfg(any(feature = "v3_20", feature = "dox"))]
    //fn grab<'a, 'b, 'c, P: Into<Option<&'a Cursor>>, Q: Into<Option<&'b Event>>, R: Into<Option<&'c /*Unimplemented*/SeatGrabPrepareFunc>>, S: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, window: &Window, capabilities: SeatCapabilities, owner_events: bool, cursor: P, event: Q, prepare_func: R, prepare_func_data: S) -> GrabStatus;

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn ungrab(&self);

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn connect_device_added<F: Fn(&Self, &Device) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn connect_device_removed<F: Fn(&Self, &Device) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn connect_tool_added<F: Fn(&Self, &DeviceTool) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn connect_tool_removed<F: Fn(&Self, &DeviceTool) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn connect_property_display_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Seat> + IsA<glib::object::Object>> SeatExt for O {
    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn get_capabilities(&self) -> SeatCapabilities {
        unsafe {
            from_glib(ffi::gdk_seat_get_capabilities(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn get_display(&self) -> Option<Display> {
        unsafe {
            from_glib_none(ffi::gdk_seat_get_display(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn get_keyboard(&self) -> Option<Device> {
        unsafe {
            from_glib_none(ffi::gdk_seat_get_keyboard(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn get_pointer(&self) -> Option<Device> {
        unsafe {
            from_glib_none(ffi::gdk_seat_get_pointer(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn get_slaves(&self, capabilities: SeatCapabilities) -> Vec<Device> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gdk_seat_get_slaves(self.to_glib_none().0, capabilities.to_glib()))
        }
    }

    //#[cfg(any(feature = "v3_20", feature = "dox"))]
    //fn grab<'a, 'b, 'c, P: Into<Option<&'a Cursor>>, Q: Into<Option<&'b Event>>, R: Into<Option<&'c /*Unimplemented*/SeatGrabPrepareFunc>>, S: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, window: &Window, capabilities: SeatCapabilities, owner_events: bool, cursor: P, event: Q, prepare_func: R, prepare_func_data: S) -> GrabStatus {
    //    unsafe { TODO: call ffi::gdk_seat_grab() }
    //}

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn ungrab(&self) {
        unsafe {
            ffi::gdk_seat_ungrab(self.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn connect_device_added<F: Fn(&Self, &Device) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &Device) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "device-added",
                transmute(device_added_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn connect_device_removed<F: Fn(&Self, &Device) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &Device) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "device-removed",
                transmute(device_removed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn connect_tool_added<F: Fn(&Self, &DeviceTool) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &DeviceTool) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "tool-added",
                transmute(tool_added_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn connect_tool_removed<F: Fn(&Self, &DeviceTool) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &DeviceTool) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "tool-removed",
                transmute(tool_removed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn connect_property_display_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::display",
                transmute(notify_display_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(any(feature = "v3_20", feature = "dox"))]
unsafe extern "C" fn device_added_trampoline<P>(this: *mut ffi::GdkSeat, device: *mut ffi::GdkDevice, f: glib_ffi::gpointer)
where P: IsA<Seat> {
    callback_guard!();
    let f: &&(Fn(&P, &Device) + 'static) = transmute(f);
    f(&Seat::from_glib_borrow(this).downcast_unchecked(), &from_glib_borrow(device))
}

#[cfg(any(feature = "v3_20", feature = "dox"))]
unsafe extern "C" fn device_removed_trampoline<P>(this: *mut ffi::GdkSeat, device: *mut ffi::GdkDevice, f: glib_ffi::gpointer)
where P: IsA<Seat> {
    callback_guard!();
    let f: &&(Fn(&P, &Device) + 'static) = transmute(f);
    f(&Seat::from_glib_borrow(this).downcast_unchecked(), &from_glib_borrow(device))
}

#[cfg(any(feature = "v3_22", feature = "dox"))]
unsafe extern "C" fn tool_added_trampoline<P>(this: *mut ffi::GdkSeat, tool: *mut ffi::GdkDeviceTool, f: glib_ffi::gpointer)
where P: IsA<Seat> {
    callback_guard!();
    let f: &&(Fn(&P, &DeviceTool) + 'static) = transmute(f);
    f(&Seat::from_glib_borrow(this).downcast_unchecked(), &from_glib_borrow(tool))
}

#[cfg(any(feature = "v3_22", feature = "dox"))]
unsafe extern "C" fn tool_removed_trampoline<P>(this: *mut ffi::GdkSeat, tool: *mut ffi::GdkDeviceTool, f: glib_ffi::gpointer)
where P: IsA<Seat> {
    callback_guard!();
    let f: &&(Fn(&P, &DeviceTool) + 'static) = transmute(f);
    f(&Seat::from_glib_borrow(this).downcast_unchecked(), &from_glib_borrow(tool))
}

#[cfg(any(feature = "v3_20", feature = "dox"))]
unsafe extern "C" fn notify_display_trampoline<P>(this: *mut ffi::GdkSeat, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Seat> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Seat::from_glib_borrow(this).downcast_unchecked())
}
