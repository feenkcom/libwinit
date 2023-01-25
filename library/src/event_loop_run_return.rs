use std::ffi::c_void;
use std::time::{Duration, Instant};

use winit::event_loop::{ControlFlow, EventLoopWindowTarget};
use winit::platform::run_return::EventLoopExtRunReturn;

use crate::events::{EventProcessor, WinitControlFlow, WinitEvent};
use crate::{WinitEventLoop, WinitUserEvent};
use value_box::{ReturnBoxerResult, ValueBox, ValueBoxPointer};

#[no_mangle]
pub extern "C" fn winit_event_loop_run_return(
    event_loop: *mut ValueBox<WinitEventLoop>,
    callback: extern "C" fn(*mut WinitEvent) -> WinitControlFlow,
) {
    event_loop
        .with_mut_ok(|event_loop| {
            let mut event_processor = EventProcessor::new();
            event_loop.run_return(
                |event,
                 _events_loop: &EventLoopWindowTarget<WinitUserEvent>,
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
                                    Instant::now() + Duration::new(0, 50 * 1000000),
                                )
                            }
                            WinitControlFlow::Exit => *control_flow = ControlFlow::Exit,
                        }
                    }
                },
            );
        })
        .log();
}

/// Initializes the winit event loop.
/// Unlike EventLoop::run, this function accepts non-'static (i.e. non-move) closures
/// and returns control flow to the caller when control_flow is set to ControlFlow::Exit.
#[no_mangle]
pub extern "C" fn winit_event_loop_run_return_data(
    event_loop: *mut ValueBox<WinitEventLoop>,
    data: *mut c_void,
    callback: extern "C" fn(*mut c_void, *mut WinitEvent) -> WinitControlFlow,
) {
    event_loop
        .with_mut_ok(|event_loop| {
            let mut event_processor = EventProcessor::new();
            event_loop.run_return(
                |event,
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
            );
        })
        .log();
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
        .to_ref()
        .map(|event_loop_ref| {
            let event_loop = unsafe { std::ptr::read(event_loop_ref.as_ptr()) };
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
