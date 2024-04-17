use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};
use winit::dpi::{PhysicalPosition, PhysicalSize};
#[cfg(target_os = "ios")]
use winit::platform::ios::WindowExtIOS;
#[cfg(target_os = "macos")]
use winit::platform::macos::WindowExtMacOS;
#[cfg(wayland_platform)]
use winit::platform::wayland::WindowExtWayland;
#[cfg(target_os = "windows")]
use winit::platform::windows::WindowExtWindows;
#[cfg(x11_platform)]
use winit::platform::x11::WindowExtX11;
use winit::window::{Window, WindowId};

use crate::enums::WinitCursorIcon;
use crate::{winit_convert_window_id, PollingEventLoop, WindowRef};
use geometry_box::{PointBox, SizeBox, U128Box};
use raw_window_handle_extensions::{VeryRawDisplayHandle, VeryRawWindowHandle};
use string_box::StringBox;
use value_box::{Result, ReturnBoxerResult, ValueBox, ValueBoxIntoRaw, ValueBoxPointer};

fn with_window<T: 'static>(
    event_loop: *mut ValueBox<PollingEventLoop>,
    window_ref: *mut ValueBox<WindowRef>,
    callback: impl FnOnce(&Window, &PollingEventLoop) -> Result<T>,
) -> Result<T> {
    event_loop.with_ref(|event_loop| {
        window_ref.with_ref(|window_ref| {
            event_loop
                .with_window(&window_ref.id(), |window| {
                    callback(window, event_loop).map_err(|error| error.into())
                })
                .map_err(|err| err.boxed().into())
        })
    })
}

fn with_window_mut<T: 'static>(
    event_loop: *mut ValueBox<PollingEventLoop>,
    window_ref: *mut ValueBox<WindowRef>,
    callback: impl FnOnce(&mut Window, &mut WindowRef) -> Result<T>,
) -> Result<T> {
    event_loop.with_mut(|event_loop| {
        window_ref.with_mut(|window_ref| {
            event_loop
                .with_window_mut(&window_ref.id(), |window, _| {
                    callback(window, window_ref).map_err(|error| error.into())
                })
                .map_err(|err| err.boxed().into())
        })
    })
}

/// Return the raw window handle that can be used to create a native rendering context.
/// Must only be called from the main thread
#[no_mangle]
pub extern "C" fn winit_window_ref_raw_window_handle(
    event_loop: *mut ValueBox<PollingEventLoop>,
    window_ref: *mut ValueBox<WindowRef>,
) -> *mut VeryRawWindowHandle {
    with_window(event_loop, window_ref, |window, _event_loop| {
        Ok(VeryRawWindowHandle::from(window.raw_window_handle()))
    })
    .map(|handle| handle.into())
    .or_log(std::ptr::null_mut())
}

/// Return the raw window handle that can be used to create a native rendering context.
/// Must only be called from the main thread
#[no_mangle]
pub extern "C" fn winit_window_ref_raw_display_handle(
    event_loop: *mut ValueBox<PollingEventLoop>,
    window_ref: *mut ValueBox<WindowRef>,
) -> *mut VeryRawDisplayHandle {
    with_window(event_loop, window_ref, |window, _event_loop| {
        Ok(VeryRawDisplayHandle::from(window.raw_display_handle()))
    })
    .map(|handle| handle.into())
    .or_log(std::ptr::null_mut())
}

/// Request the window to redraw. Can be called from any thread.
#[no_mangle]
pub extern "C" fn winit_window_ref_request_redraw(
    event_loop: *mut ValueBox<PollingEventLoop>,
    window_ref: *mut ValueBox<WindowRef>,
) {
    with_window(event_loop, window_ref, |window, _event_loop| {
        Ok(window.request_redraw())
    })
    .log();
}

/// Get the scaled factor of the window. Can be called from the any thread.
#[no_mangle]
pub extern "C" fn winit_window_ref_get_scale_factor(window_ref: *mut ValueBox<WindowRef>) -> f64 {
    window_ref
        .with_ref(|window_ref| window_ref.scale_factor().map_err(|err| err.boxed().into()))
        .or_log(1.0)
}

/// Get the inner size of the window. Can be called from the any thread.
#[no_mangle]
pub extern "C" fn winit_window_ref_get_inner_size(
    window_ref: *mut ValueBox<WindowRef>,
    inner_size: *mut ValueBox<SizeBox<u32>>,
) {
    window_ref
        .with_ref(|window_ref| {
            inner_size.with_mut(|inner_size| {
                window_ref
                    .inner_size()
                    .map_err(|err| err.boxed().into())
                    .and_then(|window_size| {
                        inner_size.width = window_size.width;
                        inner_size.height = window_size.height;
                        Ok(())
                    })
            })
        })
        .log();
}

/// Set the inner size of the window.
/// Must be called from the main thread
#[no_mangle]
pub extern "C" fn winit_window_ref_set_inner_size(
    event_loop: *mut ValueBox<PollingEventLoop>,
    window_ref: *mut ValueBox<WindowRef>,
    width: u32,
    height: u32,
) {
    with_window_mut(event_loop, window_ref, |window, window_ref| {
        let new_size = PhysicalSize::new(width, height);
        window_ref
            .set_inner_size(new_size)
            .map_err(|err| err.boxed().into())
            .map(|_| window.set_inner_size(new_size.clone()))
    })
    .log();
}

/// Get the outer position of the window. Can be called from the any thread.
#[no_mangle]
pub extern "C" fn winit_window_ref_get_position(
    window_ref: *mut ValueBox<WindowRef>,
    position: *mut ValueBox<PointBox<i32>>,
) {
    window_ref
        .with_ref(|window_ref| {
            position.with_mut(|position| {
                window_ref
                    .outer_position()
                    .map_err(|err| err.boxed().into())
                    .and_then(|window_position| {
                        position.x = window_position.x;
                        position.y = window_position.y;
                        Ok(())
                    })
            })
        })
        .log();
}

/// Set the outer position of the window.
/// Must be called from the main thread
#[no_mangle]
pub extern "C" fn winit_window_ref_set_position(
    event_loop: *mut ValueBox<PollingEventLoop>,
    window_ref: *mut ValueBox<WindowRef>,
    x: i32,
    y: i32,
) {
    with_window_mut(event_loop, window_ref, |window, window_ref| {
        let new_position = PhysicalPosition::new(x, y);
        window_ref
            .set_outer_position(new_position.clone())
            .map_err(|err| err.boxed().into())
            .map(|_| window.set_outer_position(new_position))
    })
    .log();
}

#[no_mangle]
pub extern "C" fn winit_window_ref_get_id(
    window_ref: *mut ValueBox<WindowRef>,
    id: *mut ValueBox<U128Box>,
) {
    window_ref
        .with_ref(|window_ref| {
            id.with_mut(|id| {
                let window_id: U128Box = winit_convert_window_id(window_ref.id().clone());
                id.low = window_id.low;
                id.high = window_id.high;
                Ok(())
            })
        })
        .log();
}

#[no_mangle]
pub extern "C" fn winit_window_ref_get_raw_id(
    window_ref: *mut ValueBox<WindowRef>,
) -> *mut ValueBox<WindowId> {
    window_ref
        .with_ref_ok(|window_ref| value_box!(window_ref.id()))
        .into_raw()
}

#[no_mangle]
pub extern "C" fn winit_window_ref_set_title(
    event_loop: *mut ValueBox<PollingEventLoop>,
    window_ref: *mut ValueBox<WindowRef>,
    title: *mut ValueBox<StringBox>,
) {
    with_window_mut(event_loop, window_ref, |window, _window_ref| {
        title.with_ref(|title| {
            window.set_title(title.to_string().as_ref());
            Ok(())
        })
    })
    .log();
}

#[no_mangle]
pub extern "C" fn winit_window_ref_set_cursor_icon(
    event_loop: *mut ValueBox<PollingEventLoop>,
    window_ref: *mut ValueBox<WindowRef>,
    cursor_icon: WinitCursorIcon,
) {
    with_window_mut(event_loop, window_ref, |window, _window_ref| {
        window.set_cursor_icon(cursor_icon.into());
        Ok(())
    })
    .log();
}

#[no_mangle]
pub extern "C" fn winit_window_ref_set_maximized(
    event_loop: *mut ValueBox<PollingEventLoop>,
    window_ref: *mut ValueBox<WindowRef>,
    maximized: bool,
) {
    with_window_mut(event_loop, window_ref, |window, _window_ref| {
        window.set_maximized(maximized);
        Ok(())
    })
    .log();
}

#[no_mangle]
pub extern "C" fn winit_window_ref_focus_window(
    event_loop: *mut ValueBox<PollingEventLoop>,
    window_ref: *mut ValueBox<WindowRef>,
) {
    with_window(event_loop, window_ref, |window, _event_loop| {
        Ok(window.focus_window())
    })
    .log();
}

#[cfg(target_os = "macos")]
#[no_mangle]
pub extern "C" fn winit_window_ref_get_ns_view(
    event_loop: *mut ValueBox<PollingEventLoop>,
    window_ref: *mut ValueBox<WindowRef>,
) -> cocoa::base::id {
    with_window(event_loop, window_ref, |window, _event_loop| {
        Ok(window.ns_view() as cocoa::base::id)
    })
    .or_log(cocoa::base::nil)
}

#[cfg(target_os = "ios")]
#[no_mangle]
pub extern "C" fn winit_window_ref_get_ns_view(
    event_loop: *mut ValueBox<PollingEventLoop>,
    window_ref: *mut ValueBox<WindowRef>,
) -> *mut std::ffi::c_void {
    with_window(event_loop, window_ref, |window, _event_loop| {
        Ok(window.ui_view())
    })
    .or_log(std::ptr::null_mut())
}

#[cfg(target_os = "windows")]
#[no_mangle]
pub extern "C" fn winit_window_ref_get_hwnd(
    event_loop: *mut ValueBox<PollingEventLoop>,
    window_ref: *mut ValueBox<WindowRef>,
) -> *mut std::ffi::c_void {
    with_window(event_loop, window_ref, |window, _event_loop| {
        Ok(unsafe { std::mem::transmute(window.hwnd()) })
    })
    .or_log(std::ptr::null_mut())
}

#[cfg(x11_platform)]
#[no_mangle]
pub extern "C" fn winit_window_ref_get_xlib_display(
    event_loop: *mut ValueBox<PollingEventLoop>,
    window_ref: *mut ValueBox<WindowRef>,
) -> *mut std::ffi::c_void {
    with_window(event_loop, window_ref, |window, event_loop| {
        window.xlib_display().ok_or_else(|| {
            format!(
                "Window (id: {:?}, type: {:?}) does not support X11",
                window.id(),
                event_loop.get_type()
            )
            .into()
        })
    })
    .or_log(std::ptr::null_mut())
}

#[cfg(x11_platform)]
#[no_mangle]
pub extern "C" fn winit_window_ref_get_xlib_window(
    event_loop: *mut ValueBox<PollingEventLoop>,
    window_ref: *mut ValueBox<WindowRef>,
) -> std::ffi::c_ulong {
    with_window(event_loop, window_ref, |window, event_loop| {
        window.xlib_window().ok_or_else(|| {
            format!(
                "Window (id: {:?}, type: {:?}) does not support X11",
                window.id(),
                event_loop.get_type()
            )
            .into()
        })
    })
    .or_log(0)
}

#[cfg(wayland_platform)]
#[no_mangle]
pub extern "C" fn winit_window_ref_get_wayland_surface(
    event_loop: *mut ValueBox<PollingEventLoop>,
    window_ref: *mut ValueBox<WindowRef>,
) -> *mut std::ffi::c_void {
    with_window(event_loop, window_ref, |window, event_loop| {
        window.wayland_surface().ok_or_else(|| {
            format!(
                "Window (id: {:?}, type: {:?}) does not support Wayland",
                window.id(),
                event_loop.get_type()
            )
            .into()
        })
    })
    .or_log(std::ptr::null_mut())
}

#[cfg(wayland_platform)]
#[no_mangle]
pub extern "C" fn winit_window_ref_get_wayland_display(
    event_loop: *mut ValueBox<PollingEventLoop>,
    window_ref: *mut ValueBox<WindowRef>,
) -> *mut std::ffi::c_void {
    with_window(event_loop, window_ref, |window, event_loop| {
        window.wayland_display().ok_or_else(|| {
            format!(
                "Window (id: {:?}, type: {:?}) does not support Wayland",
                window.id(),
                event_loop.get_type()
            )
            .into()
        })
    })
    .or_log(std::ptr::null_mut())
}

#[no_mangle]
pub extern "C" fn winit_window_ref_destroy(
    event_loop: *mut ValueBox<PollingEventLoop>,
    window_ref: *mut ValueBox<WindowRef>,
) {
    event_loop
        .with_mut(|event_loop| {
            window_ref.with_ref(|window_ref| {
                event_loop
                    .destroy_window(&window_ref.id())
                    .map_err(|error| error.boxed().into())
            })
        })
        .log();
    window_ref.release();
}
