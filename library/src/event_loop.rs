use std::time;

use boxer::{ValueBox, ValueBoxPointer, ValueBoxPointerReference};
use winit::event_loop::{
    ControlFlow, EventLoop, EventLoopBuilder, EventLoopProxy, EventLoopWindowTarget,
};
use winit::monitor::MonitorHandle;
use winit::platform::run_return::EventLoopExtRunReturn;

use crate::events::{EventProcessor, WinitControlFlow, WinitEvent};

pub type WinitCustomEvent = u32;
pub type WinitEventLoop = EventLoop<WinitCustomEvent>;
pub type WinitEventLoopBuilder = EventLoopBuilder<WinitCustomEvent>;
pub type WinitEventLoopProxy = EventLoopProxy<WinitCustomEvent>;

#[no_mangle]
pub fn winit_event_loop_new() -> *mut ValueBox<WinitEventLoop> {
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
pub fn winit_event_loop_drop(_ptr: &mut *mut ValueBox<WinitEventLoop>) {
    _ptr.drop();
}

#[no_mangle]
pub fn winit_event_loop_run_return(
    event_loop_ptr: *mut ValueBox<WinitEventLoop>,
    callback: extern "C" fn(*mut WinitEvent) -> WinitControlFlow,
) {
    if event_loop_ptr.is_null() {
        eprintln!("[winit_events_loop_run_return] _ptr_events_loop is null");
        return;
    }

    let mut event_processor = EventProcessor::new();

    event_loop_ptr.with_not_null(|event_loop| {
        event_loop.run_return(
            |event,
             _events_loop: &EventLoopWindowTarget<WinitCustomEvent>,
             control_flow: &mut ControlFlow| {
                *control_flow = ControlFlow::Poll;
                let mut c_event: WinitEvent = Default::default();
                let processed = event_processor.process(event, &mut c_event);
                if processed {
                    let c_event_ptr = Box::into_raw(Box::new(c_event));
                    let c_control_flow = callback(c_event_ptr);
                    unsafe { Box::from_raw(c_event_ptr) };
                    match c_control_flow {
                        WinitControlFlow::Poll => *control_flow = ControlFlow::Poll,
                        WinitControlFlow::Wait => {
                            *control_flow = ControlFlow::WaitUntil(
                                time::Instant::now() + time::Duration::new(0, 50 * 1000000),
                            )
                        }
                        WinitControlFlow::Exit => *control_flow = ControlFlow::Exit,
                    }
                }
            },
        );
    });
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
    _event_loop: &EventLoopWindowTarget<WinitCustomEvent>,
) -> WinitEventLoopType {
    use winit::platform::x11::EventLoopWindowTargetExtX11;
    use winit::platform::wayland::EventLoopWindowTargetExtWayland;

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
    _event_loop: &EventLoopWindowTarget<WinitCustomEvent>,
) -> WinitEventLoopType {
    WinitEventLoopType::Windows
}

#[cfg(target_os = "macos")]
pub fn get_event_loop_type(
    _event_loop: &EventLoopWindowTarget<WinitCustomEvent>,
) -> WinitEventLoopType {
    WinitEventLoopType::MacOS
}

#[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
pub fn get_event_loop_type(
    _event_loop: &EventLoopWindowTarget<WinitCustomEvent>,
) -> WinitEventLoopType {
    WinitEventLoopType::Unknown
}

#[no_mangle]
fn winit_event_loop_get_type(_ptr_event_loop: *mut ValueBox<WinitEventLoop>) -> WinitEventLoopType {
    _ptr_event_loop.with_not_null_return(WinitEventLoopType::Unknown, |event_loop| {
        get_event_loop_type(event_loop)
    })
}

#[no_mangle]
fn winit_event_loop_create_proxy(
    _ptr_event_loop: *mut ValueBox<WinitEventLoop>,
) -> *mut ValueBox<WinitEventLoopProxy> {
    _ptr_event_loop.with_not_null_return(std::ptr::null_mut(), |event_loop| {
        ValueBox::new(event_loop.create_proxy()).into_raw()
    })
}

#[no_mangle]
fn winit_event_loop_drop_proxy(_ptr: &mut *mut ValueBox<WinitEventLoopProxy>) {
    _ptr.drop();
}

///////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////// M O N I T O R    I D /////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////

#[no_mangle]
fn winit_event_loop_get_primary_monitor(
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
fn winit_primary_monitor_get_hidpi_factor(monitor_id_ptr: *mut ValueBox<MonitorHandle>) -> f64 {
    monitor_id_ptr.with_not_null_return(1.0, |monitor_id| monitor_id.scale_factor())
}

#[no_mangle]
fn winit_primary_monitor_drop(ptr: &mut *mut ValueBox<MonitorHandle>) {
    ptr.drop();
}
