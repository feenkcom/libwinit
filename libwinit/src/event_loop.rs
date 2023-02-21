use std::ffi::c_void;
use winit::event_loop::{
    ControlFlow, EventLoop, EventLoopBuilder, EventLoopProxy, EventLoopWindowTarget,
};
use winit::monitor::MonitorHandle;

use crate::events::{EventProcessor, WinitControlFlow, WinitEvent};
use crate::WinitUserEvent;
use value_box::{BoxerError, ReturnBoxerResult, ValueBox, ValueBoxIntoRaw, ValueBoxPointer};

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

/// Hijacks the calling thread and initializes the winit event loop with the provided closure.
/// Since the closure is 'static, it must be a move closure if it needs to access any data from the calling context.
/// See the ControlFlow docs for information on how changes to &mut ControlFlow impact the event loop's behavior.
/// Any values not passed to this function will not be dropped.
#[no_mangle]
pub extern "C" fn winit_event_loop_run_data(
    event_loop: *mut ValueBox<WinitEventLoop>,
    data: *mut c_void,
    callback: extern "C" fn(*mut c_void, *mut WinitEvent) -> WinitControlFlow,
) {
    event_loop
        .take_value()
        .map(|event_loop| {
            let mut event_processor = EventProcessor::new();
            event_loop.run(
                move |event,
                      _events_loop: &EventLoopWindowTarget<WinitUserEvent>,
                      control_flow: &mut ControlFlow| {
                    control_flow.set_wait();
                    let mut c_event: WinitEvent = Default::default();
                    let processed = event_processor.process(event, &mut c_event);
                    if processed {
                        let c_event_ptr = Box::into_raw(Box::new(c_event));
                        let c_control_flow = callback(data, c_event_ptr);
                        unsafe { Box::from_raw(c_event_ptr) };

                        *control_flow = c_control_flow.into();
                    }
                },
            )
        })
        .log();
}

#[no_mangle]
pub extern "C" fn winit_event_loop_create_proxy(
    event_loop: *mut ValueBox<WinitEventLoop>,
) -> *mut ValueBox<WinitEventLoopProxy> {
    event_loop
        .with_ref_ok(|event_loop| value_box!(event_loop.create_proxy()))
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
            event_loop
                .primary_monitor()
                .map(|monitor| value_box!(monitor))
                .ok_or_else(|| {
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
