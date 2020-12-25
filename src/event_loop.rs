use boxer::{ValueBox, ValueBoxPointer, ValueBoxPointerReference};
use events::{EventProcessor, WinitControlFlow, WinitEvent, WinitEventType};
use std::collections::VecDeque;
use std::ffi::c_void;
use std::intrinsics::transmute;
use std::sync::{Arc, Mutex};
use std::time;
use winit::event_loop::EventLoopClosed;
use winit::event_loop::{ControlFlow, EventLoop, EventLoopProxy, EventLoopWindowTarget};
use winit::monitor::MonitorHandle;
use winit::platform::desktop::EventLoopExtDesktop;

pub type WinitCustomEvent = u32;
pub type WinitEventLoop = EventLoop<WinitCustomEvent>;
pub type WinitEventLoopProxy = EventLoopProxy<WinitCustomEvent>;

pub struct SemaphoreSignaller {
    semaphore_callback: unsafe extern "C" fn(usize, *const c_void),
    semaphore_index: usize,
    semaphore_thunk: *const c_void,
}

impl SemaphoreSignaller {
    pub fn new(
        semaphore_callback: unsafe extern "C" fn(usize, *const c_void),
        semaphore_index: usize,
        semaphore_thunk: *const c_void,
    ) -> Self {
        Self {
            semaphore_callback,
            semaphore_index,
            semaphore_thunk,
        }
    }

    pub fn signal(&self) {
        let callback = self.semaphore_callback;
        unsafe { callback(self.semaphore_index, self.semaphore_thunk) };
    }
}

pub struct MainEventClearedSignaller {
    callback: unsafe extern "C" fn(*const c_void),
    thunk: *const c_void,
}

impl MainEventClearedSignaller {
    pub fn new(callback: unsafe extern "C" fn(*const c_void), thunk: *const c_void) -> Self {
        Self { callback, thunk }
    }

    pub fn signal(&self) {
        let callback = self.callback;
        unsafe { callback(self.thunk) };
    }
}

pub struct PollingEventLoop {
    events: Mutex<VecDeque<WinitEvent>>,
    semaphore_signaller: Option<SemaphoreSignaller>,
    main_events_cleared_signaller: Option<MainEventClearedSignaller>,
}

impl PollingEventLoop {
    pub fn new() -> Self {
        Self {
            events: Mutex::new(VecDeque::new()),
            semaphore_signaller: None,
            main_events_cleared_signaller: None,
        }
    }

    pub fn with_semaphore_signaller(
        mut self,
        semaphore_callback: extern "C" fn(usize, *const c_void),
        semaphore_index: usize,
        semaphore_thunk: *const c_void,
    ) -> Self {
        self.semaphore_signaller = Some(SemaphoreSignaller::new(
            semaphore_callback,
            semaphore_index,
            semaphore_thunk,
        ));
        self
    }

    pub fn with_main_events_signaller(
        mut self,
        callback: extern "C" fn(*const c_void),
        thunk: *const c_void,
    ) -> Self {
        self.main_events_cleared_signaller = Some(MainEventClearedSignaller::new(callback, thunk));
        self
    }

    pub fn poll(&mut self) -> Option<WinitEvent> {
        match self.events.lock() {
            Ok(mut guard) => guard.pop_front(),
            Err(err) => {
                println!(
                    "[PollingEventLoop::poll] Error locking the guard: {:?}",
                    err
                );
                None
            }
        }
    }

    pub fn push(&mut self, event: WinitEvent) {
        Self::push_event(&mut self.events, event);
    }

    pub fn push_event(events: &mut Mutex<VecDeque<WinitEvent>>, event: WinitEvent) {
        match events.lock() {
            Ok(mut guard) => {
                guard.push_back(event);
            }
            Err(err) => println!(
                "[PollingEventLoop::push] Error locking the guard: {:?}",
                err
            ),
        }
    }

    pub fn signal_semaphore(&self) {
        if self.semaphore_signaller.is_some() {
            self.semaphore_signaller.as_ref().unwrap().signal();
        }
    }

    pub fn signal_main_events_cleared(&self) {
        if self.main_events_cleared_signaller.is_some() {
            self.main_events_cleared_signaller
                .as_ref()
                .unwrap()
                .signal();
        }
    }

    pub fn run(&'static mut self) {
        let mut event_processor = EventProcessor::new();
        let event_loop = WinitEventLoop::with_user_event();

        event_loop.run(move |event, _, control_flow: &mut ControlFlow| {
            *control_flow = ControlFlow::Wait;

            let mut c_event = WinitEvent::default();
            let processed = event_processor.process(event, &mut c_event);
            if processed {
                let event_type = c_event.event_type;
                if event_type != WinitEventType::MainEventsCleared
                    && event_type != WinitEventType::RedrawEventsCleared
                    && event_type != WinitEventType::NewEvents
                {
                    self.push(c_event);
                    self.signal_semaphore();
                }

                if event_type == WinitEventType::MainEventsCleared {
                    self.signal_main_events_cleared();
                }
            }
        })
    }
}

#[repr(C)]
pub struct WinitEventLoopWaker {
    proxy: Arc<WinitEventLoopProxy>,
}

impl WinitEventLoopWaker {
    pub fn new(event_loop: &WinitEventLoop) -> Self {
        Self {
            proxy: Arc::new(event_loop.create_proxy()),
        }
    }

    pub fn wake(&self, event: WinitCustomEvent) -> Result<(), EventLoopClosed<WinitCustomEvent>> {
        self.proxy.send_event(event)
    }
}

extern "C" fn winit_waker_wake(waker_ptr: *const c_void, event: WinitCustomEvent) -> bool {
    let ptr = waker_ptr as *mut ValueBox<WinitEventLoopWaker>;
    ptr.with_not_null_return(false, |waker| match waker.wake(event) {
        Ok(_) => true,
        Err(_) => false,
    })
}

#[no_mangle]
pub fn winit_event_loop_waker_create(
    event_loop_ptr: *mut ValueBox<WinitEventLoop>,
) -> *mut ValueBox<WinitEventLoopWaker> {
    event_loop_ptr.with_not_null_return(std::ptr::null_mut(), |event_loop| {
        ValueBox::new(WinitEventLoopWaker::new(event_loop)).into_raw()
    })
}

#[no_mangle]
pub fn winit_event_loop_waker_function() -> extern "C" fn(*const c_void, u32) -> bool {
    winit_waker_wake
}

#[no_mangle]
pub fn winit_event_loop_waker_drop(_ptr: &mut *mut ValueBox<WinitEventLoopWaker>) {
    _ptr.drop();
}

#[no_mangle]
pub fn winit_polling_event_loop_new() -> *mut ValueBox<PollingEventLoop> {
    ValueBox::new(PollingEventLoop::new()).into_raw()
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
pub fn winit_polling_event_loop_drop(_ptr: &mut *mut ValueBox<PollingEventLoop>) {
    _ptr.drop();
}

#[no_mangle]
pub fn winit_create_events_loop() -> *mut ValueBox<WinitEventLoop> {
    ValueBox::new(WinitEventLoop::with_user_event()).into_raw()
}

#[no_mangle]
pub fn winit_destroy_events_loop(_ptr: &mut *mut ValueBox<WinitEventLoop>) {
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
fn get_event_loop_type(_event_loop: &WinitEventLoop) -> WinitEventLoopType {
    use winit::platform::unix::EventLoopWindowTargetExtUnix;
    if _event_loop.is_wayland() {
        return WinitEventLoopType::Wayland;
    }
    if _event_loop.is_x11() {
        return WinitEventLoopType::X11;
    }
    return WinitEventLoopType::Unknown;
}

#[cfg(target_os = "windows")]
fn get_event_loop_type(_event_loop: &WinitEventLoop) -> WinitEventLoopType {
    WinitEventLoopType::Windows
}

#[cfg(target_os = "macos")]
fn get_event_loop_type(_event_loop: &WinitEventLoop) -> WinitEventLoopType {
    WinitEventLoopType::MacOS
}

#[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
fn get_event_loop_type(_event_loop: &WinitEventLoop) -> WinitEventLoopType {
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