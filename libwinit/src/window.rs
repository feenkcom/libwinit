use geometry_box::{PointBox, SizeBox, U128Box};
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};
use raw_window_handle_extensions::{VeryRawDisplayHandle, VeryRawWindowHandle};
use string_box::StringBox;
use value_box::{ReturnBoxerResult, ValueBox, ValueBoxIntoRaw, ValueBoxPointer};
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::monitor::MonitorHandle;
#[cfg(target_os = "macos")]
use winit::platform::macos::WindowExtMacOS;
#[cfg(target_os = "windows")]
use winit::platform::windows::WindowExtWindows;
#[cfg(any(
    target_os = "linux",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd",
))]
use winit::platform::x11::WindowExtX11;
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
        .with_ref(|event_loop| {
            window_builder.take_value().and_then(|window_builder| {
                debug!("Window builder: {:?}", &window_builder);
                window_builder
                    .build(&event_loop)
                    .map(|window| value_box!(window))
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
) -> *mut VeryRawWindowHandle {
    window
        .with_ref_ok(Window::raw_window_handle)
        .map(|handle| VeryRawWindowHandle::from(handle).into())
        .or_log(std::ptr::null_mut())
}

#[no_mangle]
pub extern "C" fn winit_window_raw_display_handle(
    window: *mut ValueBox<Window>,
) -> *mut VeryRawDisplayHandle {
    window
        .with_ref_ok(Window::raw_display_handle)
        .map(|handle| VeryRawDisplayHandle::from(handle).into())
        .or_log(std::ptr::null_mut())
}

#[no_mangle]
pub extern "C" fn winit_window_request_redraw(window: *mut ValueBox<Window>) {
    window.with_ref_ok(Window::request_redraw).log();
}

#[no_mangle]
pub extern "C" fn winit_window_get_scale_factor(window: *mut ValueBox<Window>) -> f64 {
    window.with_ref_ok(Window::scale_factor).or_log(1.0)
}

#[no_mangle]
pub extern "C" fn winit_window_get_inner_size(
    window: *mut ValueBox<Window>,
    size: *mut ValueBox<SizeBox<u32>>,
) {
    window
        .with_ref(|window| {
            size.with_mut_ok(|size| {
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
        .with_ref_ok(|window| window.set_inner_size(PhysicalSize::new(width, height)))
        .log();
}

#[no_mangle]
pub extern "C" fn winit_window_get_position(
    window: *mut ValueBox<Window>,
    position: *mut ValueBox<PointBox<i32>>,
) {
    window
        .with_ref(|window| {
            position.with_mut(|position| {
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
        .with_ref_ok(|window| window.set_outer_position(PhysicalPosition::new(x, y)))
        .log();
}

#[no_mangle]
pub extern "C" fn winit_window_get_id(
    window: *mut ValueBox<Window>,
    window_id: *mut ValueBox<U128Box>,
) {
    window
        .with_ref(|window| {
            window_id.with_mut_ok(|window_id| {
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
        .with_ref(|window| {
            window_title.with_ref_ok(|window_title| window.set_title(window_title.as_str()))
        })
        .log();
}

#[no_mangle]
pub extern "C" fn winit_window_set_cursor_icon(
    window: *mut ValueBox<Window>,
    cursor_icon: WinitCursorIcon,
) {
    window
        .with_ref_ok(|window| window.set_cursor_icon(cursor_icon.into()))
        .log();
}

#[no_mangle]
pub extern "C" fn winit_window_set_maximized(window: *mut ValueBox<Window>, maximized: bool) {
    window
        .with_ref_ok(|window| window.set_maximized(maximized))
        .log();
}

#[no_mangle]
pub extern "C" fn winit_window_focus_window(window: *mut ValueBox<Window>) {
    window.with_ref_ok(|window| window.focus_window()).log();
}

#[cfg(target_os = "windows")]
#[no_mangle]
pub extern "C" fn winit_window_get_hwnd(
    window_ptr: *mut ValueBox<Window>,
) -> *mut std::ffi::c_void {
    window_ptr
        .with_ref_ok(|window| unsafe { std::mem::transmute(window.hwnd()) })
        .or_log(std::ptr::null_mut())
}

#[cfg(target_os = "macos")]
#[no_mangle]
pub extern "C" fn winit_window_get_ns_view(window_ptr: *mut ValueBox<Window>) -> cocoa::base::id {
    window_ptr
        .with_ref_ok(|window| window.ns_view() as cocoa::base::id)
        .or_log(cocoa::base::nil)
}

#[cfg(any(
    target_os = "linux",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd",
))]
#[no_mangle]
pub extern "C" fn winit_window_get_xlib_display(
    window: *mut ValueBox<Window>,
) -> *mut std::ffi::c_void {
    window
        .with_ref_ok(|window| window.xlib_display().unwrap_or(std::ptr::null_mut()))
        .or_log(std::ptr::null_mut())
}

#[cfg(any(
    target_os = "linux",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd",
))]
#[no_mangle]
pub extern "C" fn winit_window_get_xlib_window(window: *mut ValueBox<Window>) -> std::ffi::c_ulong {
    window
        .with_ref_ok(|window| window.xlib_window().unwrap_or(0))
        .or_log(0)
}

#[no_mangle]
pub extern "C" fn winit_window_current_monitor(
    window: *mut ValueBox<Window>,
) -> *mut ValueBox<MonitorHandle> {
    window
        .with_ref_ok(|window| {
            window
                .current_monitor()
                .map(|monitor| ValueBox::new(monitor).into_raw())
                .unwrap_or(std::ptr::null_mut())
        })
        .or_log(std::ptr::null_mut())
}

#[no_mangle]
pub extern "C" fn winit_window_drop(ptr: *mut ValueBox<Window>) {
    ptr.release();
}
