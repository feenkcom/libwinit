use std::ffi::c_void;
use std::mem::transmute;
use std::ops::Deref;

use winit::window::{WindowBuilder, WindowId};

use crate::event_loop::WinitEventLoopType;
use crate::events::WinitEvent;
use crate::{
    PollingEventLoop, WindowRedrawRequestedListener, WindowRef, WindowResizedListener,
    WinitEventLoopWaker, WinitUserEvent,
};
use value_box::{ReturnBoxerResult, ValueBox, ValueBoxIntoRaw, ValueBoxPointer};

#[no_mangle]
pub extern "C" fn winit_waker_wake(waker: *const c_void, event: WinitUserEvent) -> bool {
    let waker = waker as *mut ValueBox<WinitEventLoopWaker>;
    waker
        .with_ref_ok(|waker| match waker.wake(event) {
            Ok(_) => true,
            Err(_) => false,
        })
        .or_log(false)
}

#[no_mangle]
pub extern "C" fn winit_event_loop_waker_create(
    event_loop: *mut ValueBox<PollingEventLoop>,
) -> *mut ValueBox<WinitEventLoopWaker> {
    event_loop
        .with_ref_ok(|event_loop| value_box!(event_loop.event_loop_waker.clone()))
        .into_raw()
}

#[no_mangle]
pub extern "C" fn winit_event_loop_waker_function(
) -> extern "C" fn(*const c_void, WinitUserEvent) -> bool {
    winit_waker_wake
}

#[no_mangle]
pub extern "C" fn winit_event_loop_waker_drop(
    event_loop_waker: *mut ValueBox<WinitEventLoopWaker>,
) {
    event_loop_waker.release();
}

#[no_mangle]
pub extern "C" fn winit_polling_event_loop_new() -> *mut ValueBox<PollingEventLoop> {
    ValueBox::new(PollingEventLoop::new()).into_raw()
}

#[no_mangle]
pub extern "C" fn winit_polling_event_loop_wake(
    event_loop: *mut ValueBox<PollingEventLoop>,
    event: WinitUserEvent,
) -> bool {
    event_loop
        .with_ref_ok(|event_loop| event_loop.wake(event))
        .map(|_| true)
        .or_log(false)
}

#[no_mangle]
pub extern "C" fn winit_polling_event_loop_create_window(
    event_loop: *mut ValueBox<PollingEventLoop>,
    window_builder: *mut ValueBox<WindowBuilder>,
) -> *mut ValueBox<WindowRef> {
    event_loop
        .with_mut(|event_loop| {
            window_builder.take_value().and_then(|window_builder| {
                event_loop
                    .create_window(window_builder)
                    .map_err(|err| err.boxed().into())
            })
        })
        .map(|window| ValueBox::new(window).into_raw())
        .or_log(std::ptr::null_mut())
}

#[no_mangle]
pub extern "C" fn winit_polling_event_loop_new_with_semaphore_and_main_events_signaller(
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
pub extern "C" fn winit_polling_event_loop_add_resize_listener(
    event_loop: *mut ValueBox<PollingEventLoop>,
    window_id: *mut ValueBox<WindowId>,
    callback: unsafe extern "C" fn(*const c_void, u32, u32),
    thunk: *const c_void,
) {
    event_loop
        .with_mut(|event_loop| {
            window_id.with_ref(|window_id| {
                Ok(event_loop.add_resize_listener(
                    window_id.deref(),
                    WindowResizedListener::new(callback, thunk),
                ))
            })
        })
        .log();
}

#[no_mangle]
pub extern "C" fn winit_polling_event_loop_count_resize_listeners(
    event_loop: *mut ValueBox<PollingEventLoop>,
) -> usize {
    event_loop
        .with_ref_ok(PollingEventLoop::count_resize_listeners)
        .or_log(0)
}

#[no_mangle]
pub extern "C" fn winit_polling_event_loop_add_redraw_listener(
    event_loop: *mut ValueBox<PollingEventLoop>,
    window_id: *mut ValueBox<WindowId>,
    callback: unsafe extern "C" fn(*const c_void),
    thunk: *const c_void,
) {
    event_loop
        .with_mut(|event_loop| {
            window_id.with_ref(|window_id| {
                Ok(event_loop.add_redraw_listener(
                    window_id.deref(),
                    WindowRedrawRequestedListener::new(callback, thunk),
                ))
            })
        })
        .log();
}

#[no_mangle]
pub extern "C" fn winit_polling_event_loop_count_redraw_listeners(
    event_loop: *mut ValueBox<PollingEventLoop>,
) -> usize {
    event_loop
        .with_ref_ok(PollingEventLoop::count_redraw_listeners)
        .or_log(0)
}

#[no_mangle]
#[cfg(target_os = "android")]
pub extern "C" fn winit_polling_event_loop_set_android_app(
    event_loop: *mut ValueBox<PollingEventLoop>,
    android_app: *mut winit::platform::android::activity::AndroidApp,
) {
    if android_app.is_null() {
        Err::<(), value_box::BoxerError>(value_box::BoxerError::NullPointer(
            std::any::type_name::<winit::platform::android::activity::AndroidApp>().to_string(),
        ))
        .log();
        return;
    }

    event_loop
        .with_mut_ok(|event_loop| {
            event_loop.set_android_app(*unsafe { Box::from_raw(android_app) });
        })
        .log();
}

#[no_mangle]
pub extern "C" fn winit_polling_event_loop_poll(
    event_loop: *mut ValueBox<PollingEventLoop>,
) -> *mut WinitEvent {
    event_loop
        .with_mut_ok(|event_loop| event_loop.poll())
        .map(|event| {
            event
                .map(|event| Box::into_raw(Box::new(event)))
                .unwrap_or(std::ptr::null_mut())
        })
        .or_log(std::ptr::null_mut())
}

#[no_mangle]
pub extern "C" fn winit_polling_event_loop_run(event_loop: *mut ValueBox<PollingEventLoop>) {
    event_loop
        .with_mut_ok(|polling_event_loop| {
            let event_loop: &'static mut PollingEventLoop =
                unsafe { transmute(polling_event_loop) };
            event_loop.run();
        })
        .log();
}

/// Must be called from the inside of the `run` method of the [`PollingEventLoop`].
#[no_mangle]
pub extern "C" fn winit_polling_event_loop_get_type(
    event_loop: *mut ValueBox<PollingEventLoop>,
) -> WinitEventLoopType {
    event_loop
        .with_ref_ok(|event_loop| event_loop.get_type())
        .or_log(WinitEventLoopType::Unknown)
}

#[no_mangle]
pub extern "C" fn winit_polling_event_loop_drop(event_loop: *mut ValueBox<PollingEventLoop>) {
    event_loop.release();
}
