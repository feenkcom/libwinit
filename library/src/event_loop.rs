use winit::event_loop::{EventLoop, EventLoopBuilder, EventLoopProxy, EventLoopWindowTarget};
use winit::monitor::MonitorHandle;

use crate::WinitUserEvent;
use value_box::{BoxerError, ReturnBoxerResult, ValueBox, ValueBoxPointer};

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
    event_loop: *mut ValueBox<WinitEventLoop>,
) -> WinitEventLoopType {
    event_loop
        .with_ref_ok(|event_loop| get_event_loop_type(event_loop))
        .or_log(WinitEventLoopType::Unknown)
}

#[no_mangle]
pub extern "C" fn winit_event_loop_create_proxy(
    event_loop: *mut ValueBox<WinitEventLoop>,
) -> *mut ValueBox<WinitEventLoopProxy> {
    event_loop
        .with_ref_ok(|event_loop| event_loop.create_proxy())
        .into_raw()
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
    event_loop: *mut ValueBox<WinitEventLoop>,
) -> *mut ValueBox<MonitorHandle> {
    event_loop
        .with_ref(|event_loop| {
            event_loop.primary_monitor().ok_or_else(|| {
                BoxerError::AnyError("There is no monitor, or it is not supported".into())
            })
        })
        .into_raw()
}

#[no_mangle]
pub extern "C" fn winit_primary_monitor_get_hidpi_factor(
    monitor_handle: *mut ValueBox<MonitorHandle>,
) -> f64 {
    monitor_handle
        .with_ref_ok(|monitor_handle| monitor_handle.scale_factor())
        .or_log(1.0)
}

#[no_mangle]
pub extern "C" fn winit_primary_monitor_drop(ptr: *mut ValueBox<MonitorHandle>) {
    ptr.release();
}
