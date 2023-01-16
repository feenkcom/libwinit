use std::os::raw::c_void;
use std::time;

use value_box::{ReturnBoxerResult, ValueBox, ValueBoxPointer};
use winit::event_loop::{
    ControlFlow, EventLoop, EventLoopBuilder, EventLoopProxy, EventLoopWindowTarget,
};
use winit::monitor::MonitorHandle;

use crate::events::{EventProcessor, WinitControlFlow, WinitEvent};
use crate::WinitUserEvent;

pub type WinitEventLoop = EventLoop<WinitUserEvent>;
pub type WinitEventLoopBuilder = EventLoopBuilder<WinitUserEvent>;
pub type WinitEventLoopProxy = EventLoopProxy<WinitUserEvent>;

#[no_mangle]
pub extern "C" fn winit_event_loop_new() -> *mut ValueBox<WinitEventLoop> {
    #[cfg(target_os = "linux")]
    {
        // respect the winit backend if it is set
        if std::env::var("WINIT_UNIX_BACKEND").is_err() {
            std::env::set_var("WINIT_UNIX_BACKEND", "x11");
        }
    }
    ValueBox::new(WinitEventLoopBuilder::with_user_event().build()).into_raw()
}

#[no_mangle]
pub extern "C" fn winit_event_loop_drop(_ptr: *mut ValueBox<WinitEventLoop>) {
    _ptr.release();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum WinitEventLoopType {
    Windows,
    MacOS,
    X11,
    Wayland,
    Unknown,
}

#[cfg(target_os = "linux")]
pub fn get_event_loop_type(
    _event_loop: &EventLoopWindowTarget<WinitUserEvent>,
) -> WinitEventLoopType {
    use winit::platform::wayland::EventLoopWindowTargetExtWayland;
    use winit::platform::x11::EventLoopWindowTargetExtX11;

    if _event_loop.is_wayland() {
        return WinitEventLoopType::Wayland;
    }
    if _event_loop.is_x11() {
        return WinitEventLoopType::X11;
    }
    return WinitEventLoopType::Unknown;
}

#[cfg(target_os = "windows")]
pub fn get_event_loop_type(
    _event_loop: &EventLoopWindowTarget<WinitUserEvent>,
) -> WinitEventLoopType {
    WinitEventLoopType::Windows
}

#[cfg(target_os = "macos")]
pub fn get_event_loop_type(
    _event_loop: &EventLoopWindowTarget<WinitUserEvent>,
) -> WinitEventLoopType {
    WinitEventLoopType::MacOS
}

#[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
pub fn get_event_loop_type(
    _event_loop: &EventLoopWindowTarget<WinitUserEvent>,
) -> WinitEventLoopType {
    WinitEventLoopType::Unknown
}

#[no_mangle]
pub extern "C" fn winit_event_loop_get_type(
    _ptr_event_loop: *mut ValueBox<WinitEventLoop>,
) -> WinitEventLoopType {
    _ptr_event_loop.with_not_null_return(WinitEventLoopType::Unknown, |event_loop| {
        get_event_loop_type(event_loop)
    })
}

#[no_mangle]
pub extern "C" fn winit_event_loop_create_proxy(
    _ptr_event_loop: *mut ValueBox<WinitEventLoop>,
) -> *mut ValueBox<WinitEventLoopProxy> {
    _ptr_event_loop.with_not_null_return(std::ptr::null_mut(), |event_loop| {
        ValueBox::new(event_loop.create_proxy()).into_raw()
    })
}

#[no_mangle]
pub extern "C" fn winit_event_loop_drop_proxy(
    event_loop_proxy: *mut ValueBox<WinitEventLoopProxy>,
) {
    event_loop_proxy.release();
}

///////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////// M O N I T O R    I D /////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////

#[no_mangle]
pub extern "C" fn winit_event_loop_get_primary_monitor(
    _ptr_event_loop: *mut ValueBox<WinitEventLoop>,
) -> *mut ValueBox<MonitorHandle> {
    _ptr_event_loop.with_not_null_return(std::ptr::null_mut(), |event_loop| {
        match event_loop.primary_monitor() {
            None => std::ptr::null_mut(),
            Some(monitor) => ValueBox::new(monitor).into_raw(),
        }
    })
}

#[no_mangle]
pub extern "C" fn winit_primary_monitor_get_hidpi_factor(
    monitor_id_ptr: *mut ValueBox<MonitorHandle>,
) -> f64 {
    monitor_id_ptr.with_not_null_return(1.0, |monitor_id| monitor_id.scale_factor())
}

#[no_mangle]
pub extern "C" fn winit_primary_monitor_drop(ptr: *mut ValueBox<MonitorHandle>) {
    ptr.release();
}
