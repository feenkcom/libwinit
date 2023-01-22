use geometry_box::{PointBox, SizeBox, U128Box};
use raw_window_handle::{
    HasRawDisplayHandle, HasRawWindowHandle, RawDisplayHandle, RawWindowHandle,
};
use string_box::StringBox;
use value_box::{ReturnBoxerResult, ValueBox, ValueBoxPointer};
use winit::dpi::{PhysicalPosition, PhysicalSize};
#[cfg(target_os = "macos")]
use winit::platform::macos::WindowExtMacOS;
#[cfg(target_os = "windows")]
use winit::platform::windows::WindowExtWindows;
use winit::window::Window;
use winit::window::WindowBuilder;

use crate::enums::WinitCursorIcon;
use crate::event_loop::WinitEventLoop;
use crate::{winit_convert_window_id, WinitError};

#[no_mangle]
pub extern "C" fn winit_create_window(
    event_loop: *mut ValueBox<WinitEventLoop>,
    window_builder: *mut ValueBox<WindowBuilder>,
) -> *mut ValueBox<Window> {
    event_loop
        .to_ref()
        .and_then(|event_loop| {
            window_builder.take_value().and_then(|window_builder| {
                debug!("Window builder: {:?}", &window_builder);
                window_builder
                    .build(&event_loop)
                    .map_err(|error| Into::<WinitError>::into(error).into())
            })
        })
        .into_raw()
}

///////////////////////////////////////////////////////////////////////////////////////
///////////////////////////// W I N D O W   A C C E S S O R S /////////////////////////
///////////////////////////////////////////////////////////////////////////////////////

#[no_mangle]
pub extern "C" fn winit_window_raw_window_handle(
    window: *mut ValueBox<Window>,
) -> *mut ValueBox<RawWindowHandle> {
    window.with_ref(Window::raw_window_handle).into_raw()
}

#[no_mangle]
pub extern "C" fn winit_window_raw_display_handle(
    window: *mut ValueBox<Window>,
) -> *mut ValueBox<RawDisplayHandle> {
    window.with_ref(Window::raw_display_handle).into_raw()
}

#[no_mangle]
pub extern "C" fn winit_window_request_redraw(window: *mut ValueBox<Window>) {
    window.with_ref(Window::request_redraw).log();
}

#[no_mangle]
pub extern "C" fn winit_window_get_scale_factor(window: *mut ValueBox<Window>) -> f64 {
    window.with_ref(Window::scale_factor).or_log(1.0)
}

#[no_mangle]
pub extern "C" fn winit_window_get_inner_size(
    window: *mut ValueBox<Window>,
    size: *mut ValueBox<SizeBox<u32>>,
) {
    window
        .to_ref()
        .and_then(|window| {
            size.with_mut(|size| {
                let window_size: PhysicalSize<u32> = window.inner_size();
                size.width = window_size.width;
                size.height = window_size.height;
            })
        })
        .log()
}

#[no_mangle]
pub extern "C" fn winit_window_set_inner_size(
    window: *mut ValueBox<Window>,
    width: u32,
    height: u32,
) {
    window
        .with_ref(|window| window.set_inner_size(PhysicalSize::new(width, height)))
        .log();
}

#[no_mangle]
pub extern "C" fn winit_window_get_position(
    window: *mut ValueBox<Window>,
    position: *mut ValueBox<PointBox<i32>>,
) {
    window
        .to_ref()
        .and_then(|window| {
            position.to_ref().and_then(|mut position| {
                window
                    .outer_position()
                    .map_err(|error| Into::<WinitError>::into(error).into())
                    .map(|outer_position| {
                        position.x = outer_position.x;
                        position.y = outer_position.y;
                    })
            })
        })
        .log();
}

#[no_mangle]
pub extern "C" fn winit_window_set_position(window: *mut ValueBox<Window>, x: i32, y: i32) {
    window
        .with_ref(|window| window.set_outer_position(PhysicalPosition::new(x, y)))
        .log();
}

#[no_mangle]
pub extern "C" fn winit_window_get_id(
    window: *mut ValueBox<Window>,
    window_id: *mut ValueBox<U128Box>,
) {
    window
        .to_ref()
        .and_then(|window| {
            window_id.with_mut(|window_id| {
                let id: U128Box = winit_convert_window_id(window.id());
                window_id.low = id.low;
                window_id.high = id.high
            })
        })
        .log();
}

#[no_mangle]
pub extern "C" fn winit_window_set_title(
    window: *mut ValueBox<Window>,
    window_title: *mut ValueBox<StringBox>,
) {
    window
        .with_ref_ref(window_title, |window, window_title| {
            window.set_title(window_title.as_str())
        })
        .log();
}

#[no_mangle]
pub extern "C" fn winit_window_set_cursor_icon(
    window: *mut ValueBox<Window>,
    cursor_icon: WinitCursorIcon,
) {
    window
        .with_ref(|window| window.set_cursor_icon(cursor_icon.into()))
        .log();
}

#[no_mangle]
pub extern "C" fn winit_window_set_maximized(window: *mut ValueBox<Window>, maximized: bool) {
    window
        .with_ref(|window| window.set_maximized(maximized))
        .log();
}

#[cfg(target_os = "windows")]
#[no_mangle]
pub extern "C" fn winit_window_get_hwnd(
    window_ptr: *mut ValueBox<Window>,
) -> *mut std::ffi::c_void {
    window_ptr.with_not_null_return(std::ptr::null_mut(), |window| unsafe {
        std::mem::transmute(window.hwnd())
    })
}

#[cfg(target_os = "macos")]
#[no_mangle]
pub extern "C" fn winit_window_get_ns_view(window_ptr: *mut ValueBox<Window>) -> cocoa::base::id {
    window_ptr
        .with_ref(|window| window.ns_view() as cocoa::base::id)
        .or_log(cocoa::base::nil)
}

#[no_mangle]
pub extern "C" fn winit_window_drop(ptr: *mut ValueBox<Window>) {
    ptr.release();
}
