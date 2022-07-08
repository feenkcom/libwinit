use crate::event_loop::{get_event_loop_type, WinitEventLoopType};
use crate::events::WinitEvent;
use crate::{
    PollingEventLoop, WindowRedrawRequestedListener, WindowRef, WindowResizedListener,
    WinitCustomEvent, WinitEventLoopWaker,
};
use boxer::{ReturnBoxerResult, ValueBox, ValueBoxPointer, ValueBoxPointerReference};
use std::ffi::c_void;
use std::mem::transmute;
use std::ops::Deref;
use winit::window::{WindowBuilder, WindowId};

extern "C" fn winit_waker_wake(waker_ptr: *const c_void, event: WinitCustomEvent) -> bool {
    let waker_ptr = waker_ptr as *mut ValueBox<WinitEventLoopWaker>;
    waker_ptr.with_not_null_return(false, |waker| match waker.wake(event) {
        Ok(_) => true,
        Err(_) => false,
    })
}

#[no_mangle]
pub fn winit_event_loop_waker_create(
    event_loop_ptr: *mut ValueBox<PollingEventLoop>,
) -> *mut ValueBox<WinitEventLoopWaker> {
    event_loop_ptr.with_not_null_return(std::ptr::null_mut(), |event_loop| {
        ValueBox::new(event_loop.event_loop_waker.clone()).into_raw()
    })
}

#[no_mangle]
pub fn winit_event_loop_waker_function() -> extern "C" fn(*const c_void, u32) -> bool {
    winit_waker_wake
}

#[no_mangle]
pub fn winit_event_loop_waker_drop(event_loop_waker: &mut *mut ValueBox<WinitEventLoopWaker>) {
    event_loop_waker.drop();
}

#[no_mangle]
pub fn winit_polling_event_loop_new() -> *mut ValueBox<PollingEventLoop> {
    ValueBox::new(PollingEventLoop::new()).into_raw()
}

#[no_mangle]
fn winit_polling_event_loop_wake(
    events_loop: *mut ValueBox<PollingEventLoop>,
    event: WinitCustomEvent,
) -> bool {
    events_loop.with_not_null_return(false, |event_loop| match event_loop.wake(event) {
        Ok(_) => true,
        Err(_) => false,
    })
}

#[no_mangle]
pub fn winit_polling_event_loop_create_window(
    event_loop: *mut ValueBox<PollingEventLoop>,
    window_builder: *mut ValueBox<WindowBuilder>,
) -> *mut ValueBox<WindowRef> {
    event_loop
        .to_ref()
        .and_then(|mut event_loop| {
            window_builder.to_value().and_then(|window_builder| {
                event_loop
                    .create_window(window_builder)
                    .map_err(|err| err.boxed().into())
            })
        })
        .map(|window| ValueBox::new(window).into_raw())
        .or_log(std::ptr::null_mut())
}

#[no_mangle]
pub fn winit_polling_event_loop_new_with_semaphore_and_main_events_signaller(
    semaphore_callback: extern "C" fn(usize, *const c_void),
    semaphore_index: usize,
    semaphore_thunk: *const c_void,
    main_events_callback: extern "C" fn(*const c_void),
    main_events_thunk: *const c_void,
) -> *mut ValueBox<PollingEventLoop> {
    ValueBox::new(
        PollingEventLoop::new()
            .with_semaphore_signaller(semaphore_callback, semaphore_index, semaphore_thunk)
            .with_main_events_signaller(main_events_callback, main_events_thunk),
    )
    .into_raw()
}

#[no_mangle]
pub fn winit_polling_event_loop_add_resize_listener(
    event_loop: *mut ValueBox<PollingEventLoop>,
    window_id: *mut ValueBox<WindowId>,
    callback: unsafe extern "C" fn(*const c_void, u32, u32),
    thunk: *const c_void,
) {
    event_loop
        .to_ref()
        .and_then(|mut event_loop| {
            window_id.to_ref().and_then(|window_id| {
                Ok(event_loop.add_resize_listener(
                    window_id.deref(),
                    WindowResizedListener::new(callback, thunk),
                ))
            })
        })
        .log();
}

#[no_mangle]
pub fn winit_polling_event_loop_add_redraw_listener(
    event_loop: *mut ValueBox<PollingEventLoop>,
    window_id: *mut ValueBox<WindowId>,
    callback: unsafe extern "C" fn(*const c_void),
    thunk: *const c_void,
) {
    event_loop
        .to_ref()
        .and_then(|mut event_loop| {
            window_id.to_ref().and_then(|window_id| {
                Ok(event_loop.add_redraw_listener(
                    window_id.deref(),
                    WindowRedrawRequestedListener::new(callback, thunk),
                ))
            })
        })
        .log();
}

#[no_mangle]
pub fn winit_polling_event_loop_poll(
    _ptr: *mut ValueBox<PollingEventLoop>,
) -> *mut ValueBox<WinitEvent> {
    _ptr.with_not_null_return(std::ptr::null_mut(), |event_loop| match event_loop.poll() {
        None => std::ptr::null_mut(),
        Some(event) => ValueBox::new(event).into_raw(),
    })
}

#[no_mangle]
pub fn winit_polling_event_loop_run(_ptr_event_loop: *mut ValueBox<PollingEventLoop>) {
    if _ptr_event_loop.is_null() {
        eprintln!("[winit_polling_event_loop_run_return] _ptr_event_loop is null");
        return;
    }

    _ptr_event_loop.with_not_null(|polling_event_loop| {
        let event_loop: &'static mut PollingEventLoop = unsafe { transmute(polling_event_loop) };
        event_loop.run();
    });
}

#[no_mangle]
fn winit_polling_event_loop_get_type(
    event_loop: *mut ValueBox<PollingEventLoop>,
) -> WinitEventLoopType {
    event_loop.with_not_null_return(WinitEventLoopType::Unknown, |event_loop| {
        event_loop
            .event_loop()
            .map_or(WinitEventLoopType::Unknown, |event_loop| {
                get_event_loop_type(event_loop)
            })
    })
}

#[no_mangle]
pub fn winit_polling_event_loop_drop(event_loop: &mut *mut ValueBox<PollingEventLoop>) {
    event_loop.drop();
}
