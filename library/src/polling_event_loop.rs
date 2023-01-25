use std::borrow::{Borrow, BorrowMut};
use std::collections::{HashMap, VecDeque};
use std::ffi::c_void;
use std::sync::Arc;

use geometry_box::U128Box;
use parking_lot::Mutex;
use value_box::{BoxerError, ReturnBoxerResult};
use winit::dpi::PhysicalSize;
use winit::event::{Event, Ime, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop, EventLoopProxy, EventLoopWindowTarget};
#[cfg(target_os = "ios")]
use winit::platform::ios::WindowBuilderExtIOS;
use winit::window::{Window, WindowBuilder, WindowId};

use crate::event_loop::WinitEventLoopBuilder;
use crate::events::{
    winit_event_loop_process_received_character, EventProcessor, WinitEvent, WinitEventType,
};
use crate::{winit_convert_window_id, Result, WindowRef, WinitError, WinitUserEvent};

pub type WinitEventLoop = EventLoop<WinitUserEvent>;
pub type WinitEventLoopProxy = EventLoopProxy<WinitUserEvent>;

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
pub struct WindowRedrawRequestedListener {
    thunk: *const c_void,
    callback: unsafe extern "C" fn(*const c_void),
}

impl WindowRedrawRequestedListener {
    pub fn new(callback: unsafe extern "C" fn(*const c_void), thunk: *const c_void) -> Self {
        Self { callback, thunk }
    }

    fn on_redraw_requested(&self) {
        unsafe {
            (self.callback)(self.thunk);
        }
    }
}

impl Drop for WindowRedrawRequestedListener {
    fn drop(&mut self) {}
}

#[derive(Debug)]
pub struct WindowResizedListener {
    thunk: *const c_void,
    callback: unsafe extern "C" fn(*const c_void, u32, u32),
}

impl WindowResizedListener {
    pub fn new(
        callback: unsafe extern "C" fn(*const c_void, u32, u32),
        thunk: *const c_void,
    ) -> Self {
        Self { callback, thunk }
    }

    fn on_window_resized(&self, size: &PhysicalSize<u32>) {
        unsafe {
            (self.callback)(self.thunk, size.width, size.height);
        }
    }
}

#[derive(Debug)]
pub struct PollingEventLoop {
    windows: Mutex<HashMap<WindowId, (WindowRef, Window)>>,
    events: Mutex<VecDeque<WinitEvent>>,
    pub(crate) event_loop_waker: WinitEventLoopWaker,
    semaphore_signaller: Option<SemaphoreSignaller>,
    main_events_cleared_signaller: Option<MainEventClearedSignaller>,
    window_redraw_listeners: Mutex<HashMap<WindowId, WindowRedrawRequestedListener>>,
    window_resize_listeners: Mutex<HashMap<WindowId, WindowResizedListener>>,
    pub(crate) running_event_loop: *const EventLoopWindowTarget<WinitUserEvent>,
}

impl PollingEventLoop {
    pub fn new() -> Self {
        Self {
            windows: Default::default(),
            events: Mutex::new(VecDeque::new()),
            event_loop_waker: WinitEventLoopWaker::new(),
            semaphore_signaller: None,
            main_events_cleared_signaller: None,
            window_redraw_listeners: Default::default(),
            window_resize_listeners: Default::default(),
            running_event_loop: std::ptr::null(),
        }
    }

    pub fn add_redraw_listener(
        &mut self,
        window_id: &WindowId,
        listener: WindowRedrawRequestedListener,
    ) {
        self.window_redraw_listeners
            .lock()
            .insert(window_id.clone(), listener);
    }

    pub fn remove_redraw_listener(
        &mut self,
        window_id: &WindowId,
    ) -> Option<WindowRedrawRequestedListener> {
        self.window_redraw_listeners.lock().remove(window_id)
    }

    pub fn count_redraw_listeners(&self) -> usize {
        self.window_redraw_listeners.lock().len()
    }

    pub fn add_resize_listener(&mut self, window_id: &WindowId, listener: WindowResizedListener) {
        self.window_resize_listeners
            .lock()
            .insert(window_id.clone(), listener);
    }

    pub fn remove_resize_listener(
        &mut self,
        window_id: &WindowId,
    ) -> Option<WindowResizedListener> {
        self.window_resize_listeners.lock().remove(window_id)
    }

    pub fn count_resize_listeners(&self) -> usize {
        self.window_resize_listeners.lock().len()
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
        self.events.lock().pop_front()
    }

    pub fn push(&mut self, event: WinitEvent) {
        Self::push_event(&mut self.events, event);
    }

    pub fn push_event(events: &mut Mutex<VecDeque<WinitEvent>>, event: WinitEvent) {
        events.lock().push_back(event);
    }

    pub fn signal_semaphore(&self) {
        if let Some(signaller) = self.semaphore_signaller.as_ref() {
            signaller.signal()
        }
    }

    pub fn signal_main_events_cleared(&self) {
        if let Some(signaller) = self.main_events_cleared_signaller.as_ref() {
            signaller.signal()
        }
    }

    /// Is called when a window requested to be redrawn
    fn on_redraw_requested(&self, window_id: &WindowId) -> Result<()> {
        trace!("Received RedrawRequested({:?})", window_id);
        if let Some(listener) = self.window_redraw_listeners.lock().get(window_id) {
            listener.on_redraw_requested();
        }
        Ok(())
    }

    /// Is called when a window is resized
    fn on_window_resized(&mut self, window_id: &WindowId, size: &PhysicalSize<u32>) -> Result<()> {
        self.with_window_mut(window_id, |_window, window_ref| {
            window_ref.set_inner_size(size.clone())
        })?;

        if let Some(listener) = self.window_resize_listeners.lock().get(window_id) {
            listener.on_window_resized(size);
        }

        self.with_window(window_id, |window| Ok(window.request_redraw()))?;
        Ok(())
    }

    /// Is called when windows's scale changed
    fn on_window_scale_changed(
        &mut self,
        window_id: &WindowId,
        scale_factor: &f64,
        new_inner_size: &PhysicalSize<u32>,
    ) -> Result<()> {
        self.with_window_mut(window_id, |_window, window_ref| {
            window_ref.set_inner_size(new_inner_size.clone())?;
            window_ref.set_scale_factor(scale_factor.clone())
        })?;

        if let Some(listeners) = self.window_resize_listeners.lock().get(window_id) {
            listeners.on_window_resized(&new_inner_size);
        }

        self.with_window(window_id, |window| Ok(window.request_redraw()))?;
        Ok(())
    }

    /// Create and register a window in the event loop
    pub fn create_window(&mut self, window_builder: WindowBuilder) -> Result<WindowRef> {
        self.event_loop()
            .ok_or(WinitError::EventLoopNotRunning)
            .and_then(|event_loop| {
                (if let Some(_monitor) = event_loop.primary_monitor() {
                    #[cfg(target_os = "ios")]
                    {
                        window_builder
                            .with_inner_size(_monitor.size())
                            .with_scale_factor(_monitor.scale_factor())
                    }
                    #[cfg(not(target_os = "ios"))]
                    {
                        window_builder
                    }
                } else {
                    window_builder
                })
                .build(event_loop)
                .map_err(|err| err.into())
            })
            .and_then(|window| {
                let window_id = window.id();
                let window_ref = WindowRef::new(&window_id);
                window_ref.set_scale_factor(window.scale_factor())?;
                window_ref.set_inner_size(window.inner_size())?;
                if let Ok(position) = window.outer_position() {
                    window_ref.set_outer_position(position)?;
                }

                window.set_ime_allowed(true);

                self.windows
                    .lock()
                    .insert(window_id, (window_ref.clone(), window));

                Ok(window_ref)
            })
    }

    pub fn with_window<T>(
        &self,
        window_id: &WindowId,
        callback: impl FnOnce(&Window) -> Result<T>,
    ) -> Result<T> {
        self.windows
            .lock()
            .get(window_id)
            .ok_or_else(|| WinitError::WindowNotFound(window_id.clone()))
            .and_then(|(_window_ref, window)| callback(window))
    }

    pub fn with_window_mut<T>(
        &mut self,
        window_id: &WindowId,
        callback: impl FnOnce(&mut Window, &mut WindowRef) -> Result<T>,
    ) -> Result<T> {
        self.windows
            .lock()
            .get_mut(window_id)
            .ok_or_else(|| WinitError::WindowNotFound(window_id.clone()))
            .and_then(|(window_ref, window)| callback(window, window_ref))
    }

    /// Destroy a window by its id. Removes all assigned resize and redraw listeners
    pub fn destroy_window(&mut self, window_id: &WindowId) -> Result<()> {
        self.window_resize_listeners.lock().remove(window_id);
        self.window_redraw_listeners.lock().remove(window_id);

        if let Some(window) = self.windows.lock().remove(window_id) {
            drop(window);
            info!("Closed window with id {:?}", window_id);
            Ok(())
        } else {
            warn!("Could not find window to close with id {:?}", window_id);
            Err(WinitError::WindowNotFound(window_id.clone()))
        }
    }

    pub fn run(&'static mut self) {
        let mut event_processor = EventProcessor::new();
        let event_loop = WinitEventLoopBuilder::with_user_event().build();
        self.event_loop_waker.proxy(event_loop.create_proxy());

        event_loop.run(move |event, event_loop, control_flow: &mut ControlFlow| {
            self.running_event_loop = event_loop as *const EventLoopWindowTarget<WinitUserEvent>;
            *control_flow = ControlFlow::Wait;

            let result = match &event {
                Event::UserEvent(value) => Ok(trace!("Received UserEvent({:?})", value)),
                Event::RedrawRequested(window_id) => self.on_redraw_requested(window_id),
                Event::WindowEvent { window_id, event } => match event {
                    WindowEvent::Resized(size) => self.on_window_resized(window_id, size),
                    WindowEvent::ScaleFactorChanged {
                        scale_factor,
                        new_inner_size,
                    } => self.on_window_scale_changed(window_id, scale_factor, new_inner_size),
                    WindowEvent::Ime(ime) => {
                        match ime {
                            Ime::Enabled => {}
                            Ime::Preedit(_, _) => {}
                            Ime::Commit(string) => {
                                for char in string.chars() {
                                    let mut c_event = WinitEvent::default();
                                    let id: U128Box = winit_convert_window_id(window_id.clone());
                                    c_event.window_id.clone_from(&id);
                                    winit_event_loop_process_received_character(&mut c_event, char);
                                    self.push(c_event);
                                }
                            }
                            Ime::Disabled => {}
                        }
                        Ok(())
                    }
                    _ => Ok(()),
                },
                _ => Ok(()),
            };

            result.map_err(|error| BoxerError::from(error)).log();

            let mut c_event = WinitEvent::default();
            let processed = event_processor.process(event, &mut c_event);
            if processed {
                let event_type = c_event.event_type;

                if event_type != WinitEventType::MainEventsCleared
                    && event_type != WinitEventType::RedrawEventsCleared
                    && event_type != WinitEventType::NewEvents
                    && event_type != WinitEventType::RedrawRequested
                {
                    self.push(c_event);
                    self.signal_semaphore();
                }

                if event_type == WinitEventType::MainEventsCleared
                    || event_type == WinitEventType::RedrawEventsCleared
                {
                    self.signal_main_events_cleared();
                }
            }
            self.running_event_loop = std::ptr::null_mut();
        })
    }

    pub fn wake(&self, event: WinitUserEvent) -> Result<()> {
        self.event_loop_waker.wake(event)
    }

    pub fn event_loop(&self) -> Option<&EventLoopWindowTarget<WinitUserEvent>> {
        if self.running_event_loop.is_null() {
            None
        } else {
            Some(unsafe { &*self.running_event_loop })
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct WinitEventLoopWaker {
    proxy: Arc<Mutex<Option<WinitEventLoopProxy>>>,
}

impl WinitEventLoopWaker {
    pub fn new() -> Self {
        Self {
            proxy: Arc::new(Mutex::new(None)),
        }
    }

    pub fn proxy(&self, proxy: WinitEventLoopProxy) {
        self.proxy.lock().borrow_mut().replace(proxy);
    }

    pub fn wake(&self, event: WinitUserEvent) -> Result<()> {
        match self.proxy.lock().borrow().as_ref() {
            None => Ok(()),
            Some(proxy) => proxy.send_event(event).map_err(|err| err.into()),
        }
    }
}
