use std::collections::HashMap;
use std::mem::transmute;

use geometry_box::U128Box;
use value_box::{ValueBox, ValueBoxPointer};
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::event::*;

use crate::{winit_convert_window_id, WinitUserEvent};

#[derive(Debug, Default)]
#[repr(C)]
pub struct WinitEvent {
    pub window_id: U128Box,
    pub event_type: WinitEventType,
    pub touch: WinitTouchEvent,
    pub mouse_wheel: WinitMouseWheelEvent,
    pub mouse_input: WinitMouseInputEvent,
    pub cursor_moved: WinitCursorMovedEvent,
    pub keyboard_input: WinitEventKeyboardInput,
    pub received_character: WinitEventReceivedCharacter,
    pub window_resized: WinitWindowResizedEvent,
    pub scale_factor: WinitWindowScaleFactorChangedEvent,
    pub window_moved: WinitWindowMovedEvent,
    pub window_focused: WinitWindowFocusedEvent,
    pub modifiers: WinitEventModifiersState,
    pub user_event: WinitEventUserEvent,
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct WinitTouchEvent {
    device_id: i64,
    phase: WinitEventTouchPhase,
    x: f64,
    y: f64,
    /// unique identifier of a finger.
    id: u64,
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct WinitMouseWheelEvent {
    device_id: i64,
    phase: WinitEventTouchPhase,
    delta: WinitMouseScrollDelta,
}

#[derive(Debug, Copy, Clone, Default)]
#[repr(C)]
pub struct WinitMouseInputEvent {
    device_id: i64,
    state: WinitEventInputElementState,
    button: WinitEventMouseButton,
}

#[derive(Debug, Copy, Clone, Default)]
#[repr(C)]
pub struct WinitCursorMovedEvent {
    device_id: i64,
    x: f64,
    y: f64,
}

#[derive(Debug, Copy, Clone, Default)]
#[repr(C)]
pub struct WinitWindowResizedEvent {
    width: u32,
    height: u32,
}

#[derive(Debug, Copy, Clone, Default)]
#[repr(C)]
pub struct WinitWindowScaleFactorChangedEvent {
    scale_factor: f64,
    width: u32,
    height: u32,
}

#[derive(Debug, Copy, Clone, Default)]
#[repr(C)]
pub struct WinitWindowMovedEvent {
    x: i32,
    y: i32,
}

#[derive(Debug, Copy, Clone, Default)]
#[repr(C)]
pub struct WinitWindowFocusedEvent {
    is_focused: bool,
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct WinitEventKeyboardInput {
    device_id: i64,
    scan_code: u32,
    state: WinitEventInputElementState,
    has_virtual_keycode: bool,
    virtual_keycode: VirtualKeyCode,
    is_synthetic: bool,
}

impl Default for WinitEventKeyboardInput {
    fn default() -> Self {
        WinitEventKeyboardInput {
            device_id: Default::default(),
            scan_code: Default::default(),
            state: Default::default(),
            has_virtual_keycode: Default::default(),
            virtual_keycode: VirtualKeyCode::Unlabeled,
            is_synthetic: false,
        }
    }
}

#[derive(Debug, Copy, Clone, Default)]
#[repr(C)]
pub struct WinitEventReceivedCharacter {
    length: usize,
    byte_1: u8,
    byte_2: u8,
    byte_3: u8,
    byte_4: u8,
}

#[derive(Debug, Copy, Clone, Default)]
#[repr(C)]
pub struct WinitMouseScrollDelta {
    delta_type: WinitEventMouseScrollDeltaType,
    x: f64,
    y: f64,
}

#[derive(Default, Debug, Hash, PartialEq, Eq, Clone, Copy)]
#[repr(C)]
pub struct WinitEventModifiersState {
    /// The "shift" key
    shift: bool,
    /// The "control" key
    ctrl: bool,
    /// The "alt" key
    alt: bool,
    /// The "logo" key
    ///
    /// This is the "windows" key on PC and "command" key on Mac.
    logo: bool,
}

#[derive(Debug, Copy, Clone, Default)]
#[repr(C)]
pub struct WinitEventMouseButton {
    button_type: WinitEventMouseButtonType,
    button_code: u16,
}

#[derive(Debug, Copy, Clone, Default)]
#[repr(C)]
pub struct WinitEventUserEvent {
    event: WinitUserEvent,
}

///////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////// S T R U C T S  ////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Copy, Clone)]
#[repr(u32)]
pub enum WinitEventMouseButtonType {
    Unknown,
    Left,
    Right,
    Middle,
    Other,
}

impl Default for WinitEventMouseButtonType {
    fn default() -> Self {
        WinitEventMouseButtonType::Unknown
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u32)]
pub enum WinitEventType {
    Unknown,
    WindowEventResized,
    WindowEventMoved,
    WindowEventCloseRequested,
    WindowEventDestroyed,
    WindowEventDroppedFile,
    WindowEventHoveredFile,
    WindowEventHoveredFileCancelled,
    WindowEventReceivedCharacter,
    WindowEventFocused,
    WindowEventKeyboardInput,
    WindowEventCursorMoved,
    WindowEventCursorEntered,
    WindowEventCursorLeft,
    WindowEventMouseWheel,
    WindowEventMouseInput,
    WindowEventTouchpadPressure,
    WindowEventAxisMotion,
    WindowEventTouch,
    WindowEventScaleFactorChanged,
    NewEvents,
    MainEventsCleared,
    LoopDestroyed,
    Suspended,
    Resumed,
    RedrawRequested,
    RedrawEventsCleared,
    ModifiersChanged,
    UserEvent,
}

impl Default for WinitEventType {
    fn default() -> Self {
        WinitEventType::Unknown
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(u32)]
pub enum WinitEventTouchPhase {
    Unknown,
    Started,
    Moved,
    Ended,
    Cancelled,
}

impl Default for WinitEventTouchPhase {
    fn default() -> Self {
        WinitEventTouchPhase::Unknown
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(u32)]
pub enum WinitEventMouseScrollDeltaType {
    Unknown,
    LineDelta,
    PixelDelta,
}

impl Default for WinitEventMouseScrollDeltaType {
    fn default() -> Self {
        WinitEventMouseScrollDeltaType::Unknown
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(u32)]
pub enum WinitEventInputElementState {
    Unknown,
    Pressed,
    Released,
}

#[derive(Debug, Copy, Clone)]
#[repr(u32)]
pub enum WinitControlFlow {
    /// When the current loop iteration finishes, immediately begin a new iteration regardless of
    /// whether or not new events are available to process.
    Poll,
    /// When the current loop iteration finishes, suspend the thread until another event arrives.
    Wait,
    /// Send a `LoopDestroyed` event and stop the event loop. This variant is *sticky* - once set,
    /// `control_flow` cannot be changed from `Exit`, and any future attempts to do so will result
    /// in the `control_flow` parameter being reset to `Exit`.
    Exit,
}

impl Default for WinitEventInputElementState {
    fn default() -> Self {
        WinitEventInputElementState::Unknown
    }
}

///////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////// E V E N T S ////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////

pub struct EventProcessor {
    pub key_buffer: HashMap<ScanCode, VirtualKeyCode>,
}

impl EventProcessor {
    pub fn new() -> Self {
        Self {
            key_buffer: HashMap::new(),
        }
    }

    pub fn process(
        &mut self,
        global_event: Event<WinitUserEvent>,
        c_event: &mut WinitEvent,
    ) -> bool {
        c_event.event_type = WinitEventType::Unknown;
        let mut result = true;

        match global_event {
            Event::WindowEvent { event, window_id } => {
                let id: U128Box = winit_convert_window_id(window_id);
                c_event.window_id.clone_from(&id);

                match event {
                    WindowEvent::Resized(PhysicalSize { width, height }) => {
                        c_event.event_type = WinitEventType::WindowEventResized;
                        c_event.window_resized.width = width;
                        c_event.window_resized.height = height;
                    }
                    WindowEvent::ScaleFactorChanged {
                        scale_factor,
                        new_inner_size,
                    } => {
                        c_event.event_type = WinitEventType::WindowEventScaleFactorChanged;
                        c_event.scale_factor.scale_factor = scale_factor;
                        c_event.scale_factor.width = new_inner_size.width;
                        c_event.scale_factor.height = new_inner_size.height;
                    }
                    WindowEvent::Focused(is_focused) => {
                        c_event.event_type = WinitEventType::WindowEventFocused;
                        c_event.window_focused.is_focused = is_focused;
                    }
                    WindowEvent::Moved(PhysicalPosition { x, y }) => {
                        c_event.event_type = WinitEventType::WindowEventMoved;
                        c_event.window_moved.x = x as i32;
                        c_event.window_moved.y = y as i32;
                    }
                    WindowEvent::CloseRequested => {
                        c_event.event_type = WinitEventType::WindowEventCloseRequested;
                    }
                    WindowEvent::Destroyed => {
                        c_event.event_type = WinitEventType::WindowEventDestroyed;
                    }
                    WindowEvent::Touch(Touch {
                        device_id,
                        phase,
                        location,
                        force: _,
                        id,
                    }) => {
                        winit_event_loop_process_touch(c_event, device_id, phase, location, id);
                    }
                    WindowEvent::MouseInput {
                        device_id,
                        state,
                        button,
                        ..
                    } => {
                        winit_event_loop_process_mouse_input(c_event, device_id, state, button);
                    }
                    WindowEvent::CursorMoved {
                        device_id,
                        position,
                        ..
                    } => {
                        winit_event_loop_process_cursor_moved(c_event, device_id, position);
                    }
                    WindowEvent::CursorEntered { device_id } => {
                        winit_event_loop_process_cursor_entered(c_event, device_id);
                    }
                    WindowEvent::CursorLeft { device_id } => {
                        winit_event_loop_process_cursor_left(c_event, device_id);
                    }
                    WindowEvent::MouseWheel {
                        device_id,
                        delta,
                        phase,
                        ..
                    } => {
                        winit_event_loop_process_mouse_wheel(c_event, device_id, delta, phase);
                    }
                    WindowEvent::KeyboardInput {
                        device_id,
                        input,
                        is_synthetic,
                    } => {
                        self.process_keyboard_input(c_event, device_id, input, is_synthetic);
                    }
                    WindowEvent::ReceivedCharacter(character) => {
                        winit_event_loop_process_received_character(c_event, character);
                    }
                    WindowEvent::ModifiersChanged(modifiers) => {
                        c_event.event_type = WinitEventType::ModifiersChanged;
                        c_event.modifiers.alt = modifiers.alt();
                        c_event.modifiers.ctrl = modifiers.ctrl();
                        c_event.modifiers.logo = modifiers.logo();
                        c_event.modifiers.shift = modifiers.shift();
                    }
                    _ => result = false,
                }
            }

            Event::NewEvents(_start_cause) => {
                c_event.event_type = WinitEventType::NewEvents;
            }
            Event::MainEventsCleared => {
                c_event.event_type = WinitEventType::MainEventsCleared;
            }
            Event::RedrawEventsCleared => {
                c_event.event_type = WinitEventType::RedrawEventsCleared;
            }
            Event::LoopDestroyed => {
                c_event.event_type = WinitEventType::LoopDestroyed;
            }
            Event::RedrawRequested(window_id) => {
                c_event.event_type = WinitEventType::RedrawRequested;
                let id: U128Box = winit_convert_window_id(window_id);
                c_event.window_id.clone_from(&id);
            }
            Event::Suspended => {
                c_event.event_type = WinitEventType::Suspended;
            }
            Event::Resumed => {
                c_event.event_type = WinitEventType::Resumed;
            }
            Event::UserEvent(custom_event) => {
                c_event.event_type = WinitEventType::UserEvent;
                c_event.user_event.event = custom_event;
            }
            Event::DeviceEvent {
                device_id: _,
                event: _,
            } => result = false,
        }
        result
    }

    fn process_keyboard_input(
        &mut self,
        c_event: &mut WinitEvent,
        device_id: DeviceId,
        input: KeyboardInput,
        is_synthetic: bool,
    ) {
        c_event.event_type = WinitEventType::WindowEventKeyboardInput;
        c_event.keyboard_input.device_id = unsafe { transmute(&device_id) };
        c_event.keyboard_input.is_synthetic = is_synthetic;
        c_event.keyboard_input.scan_code = input.scancode;

        match input.state {
            ElementState::Pressed => {
                c_event.keyboard_input.state = WinitEventInputElementState::Pressed;
            }
            ElementState::Released => {
                c_event.keyboard_input.state = WinitEventInputElementState::Released;
            }
        }

        let key_code = match input.state {
            ElementState::Pressed => match input.virtual_keycode {
                None => None,
                Some(code) => match self.key_buffer.get(&input.scancode) {
                    None => {
                        (&mut self.key_buffer).insert(input.scancode, code);
                        Some(code)
                    }
                    Some(code) => Some(code.clone()),
                },
            },
            ElementState::Released => match self.key_buffer.remove(&input.scancode) {
                None => input.virtual_keycode,
                Some(code) => Some(code),
            },
        };

        match key_code {
            Some(code) => {
                c_event.keyboard_input.has_virtual_keycode = true;
                c_event.keyboard_input.virtual_keycode = code;
            }
            None => {
                c_event.keyboard_input.has_virtual_keycode = false;
            }
        }
    }
}

fn winit_event_loop_process_mouse_wheel(
    c_event: &mut WinitEvent,
    device_id: DeviceId,
    delta: MouseScrollDelta,
    phase: TouchPhase,
) {
    c_event.event_type = WinitEventType::WindowEventMouseWheel;
    c_event.mouse_wheel.device_id = unsafe { transmute(&device_id) };

    match delta {
        MouseScrollDelta::LineDelta(x, y) => {
            c_event.mouse_wheel.delta.delta_type = WinitEventMouseScrollDeltaType::LineDelta;
            c_event.mouse_wheel.delta.x = -x as f64;
            c_event.mouse_wheel.delta.y = y as f64;
        }
        MouseScrollDelta::PixelDelta(PhysicalPosition { x, y }) => {
            c_event.mouse_wheel.delta.delta_type = WinitEventMouseScrollDeltaType::PixelDelta;
            c_event.mouse_wheel.delta.x = -x;
            c_event.mouse_wheel.delta.y = y;
        }
    }

    match phase {
        TouchPhase::Started => {
            c_event.mouse_wheel.phase = WinitEventTouchPhase::Started;
        }
        TouchPhase::Moved => {
            c_event.mouse_wheel.phase = WinitEventTouchPhase::Moved;
        }
        TouchPhase::Ended => {
            c_event.mouse_wheel.phase = WinitEventTouchPhase::Ended;
        }
        TouchPhase::Cancelled => {
            c_event.mouse_wheel.phase = WinitEventTouchPhase::Cancelled;
        }
    }
}

fn winit_event_loop_process_touch(
    c_event: &mut WinitEvent,
    device_id: DeviceId,
    phase: TouchPhase,
    location: PhysicalPosition<f64>,
    id: u64,
) {
    c_event.event_type = WinitEventType::WindowEventTouch;
    c_event.touch.device_id = unsafe { transmute(&device_id) };
    c_event.touch.x = location.x;
    c_event.touch.y = location.y;
    c_event.touch.id = id;

    match phase {
        TouchPhase::Started => {
            c_event.touch.phase = WinitEventTouchPhase::Started;
        }
        TouchPhase::Moved => {
            c_event.touch.phase = WinitEventTouchPhase::Moved;
        }
        TouchPhase::Ended => {
            c_event.touch.phase = WinitEventTouchPhase::Ended;
        }
        TouchPhase::Cancelled => {
            c_event.touch.phase = WinitEventTouchPhase::Cancelled;
        }
    }
}

fn winit_event_loop_process_mouse_input(
    c_event: &mut WinitEvent,
    device_id: DeviceId,
    state: ElementState,
    button: MouseButton,
) {
    c_event.event_type = WinitEventType::WindowEventMouseInput;
    c_event.mouse_input.device_id = unsafe { transmute(&device_id) };

    match state {
        ElementState::Released => {
            c_event.mouse_input.state = WinitEventInputElementState::Released;
        }
        ElementState::Pressed => {
            c_event.mouse_input.state = WinitEventInputElementState::Pressed;
        }
    }

    match button {
        MouseButton::Left => {
            c_event.mouse_input.button.button_type = WinitEventMouseButtonType::Left;
            c_event.mouse_input.button.button_code = 0;
        }
        MouseButton::Right => {
            c_event.mouse_input.button.button_type = WinitEventMouseButtonType::Right;
            c_event.mouse_input.button.button_code = 1;
        }
        MouseButton::Middle => {
            c_event.mouse_input.button.button_type = WinitEventMouseButtonType::Middle;
            c_event.mouse_input.button.button_code = 2;
        }
        MouseButton::Other(code) => {
            c_event.mouse_input.button.button_type = WinitEventMouseButtonType::Other;
            c_event.mouse_input.button.button_code = code;
        }
    }
}

fn winit_event_loop_process_cursor_moved<T: Into<f64>>(
    c_event: &mut WinitEvent,
    device_id: DeviceId,
    position: PhysicalPosition<T>,
) {
    c_event.event_type = WinitEventType::WindowEventCursorMoved;
    c_event.cursor_moved.device_id = unsafe { transmute(&device_id) };

    c_event.cursor_moved.x = position.x.into();
    c_event.cursor_moved.y = position.y.into();
}

fn winit_event_loop_process_cursor_entered(c_event: &mut WinitEvent, _: DeviceId) {
    c_event.event_type = WinitEventType::WindowEventCursorEntered;
}

fn winit_event_loop_process_cursor_left(c_event: &mut WinitEvent, _: DeviceId) {
    c_event.event_type = WinitEventType::WindowEventCursorLeft;
}

fn winit_event_loop_process_received_character(c_event: &mut WinitEvent, character: char) {
    c_event.event_type = WinitEventType::WindowEventReceivedCharacter;

    let mut buffer = [0; 4];
    let result = character.encode_utf8(&mut buffer);
    let length = result.len();

    c_event.received_character.length = length;

    let bytes = result.as_bytes();

    if length >= 1 {
        c_event.received_character.byte_1 = bytes[0];
    }
    if length >= 2 {
        c_event.received_character.byte_2 = bytes[1];
    }
    if length >= 3 {
        c_event.received_character.byte_3 = bytes[2];
    }
    if length >= 4 {
        c_event.received_character.byte_4 = bytes[3];
    }
}

#[no_mangle]
pub fn winit_event_drop(ptr: *mut ValueBox<WinitEvent>) {
    ptr.release();
}
