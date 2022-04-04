use crate::enums::WinitCursorIcon;
use crate::{winit_convert_window_id, PollingEventLoop, WindowRef};

use boxer::number::BoxerUint128;
use boxer::point::BoxerPointI32;
use boxer::size::BoxerSizeU32;
use boxer::string::BoxerString;
use boxer::{Result, ValueBoxPointerReference};
use boxer::{ReturnBoxerResult, ValueBox, ValueBoxPointer};
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use std::ops::DerefMut;
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::window::{Window, WindowId};

#[cfg(target_os = "macos")]
use winit::platform::macos::WindowExtMacOS;

fn with_window<T>(
    event_loop: *mut ValueBox<PollingEventLoop>,
    window_ref: *mut ValueBox<WindowRef>,
    callback: impl FnOnce(&Window) -> Result<T>,
) -> Result<T> {
    event_loop.to_ref().and_then(|event_loop| {
        window_ref.to_ref().and_then(|window_ref| {
            event_loop
                .with_window(&window_ref.id(), |window| {
                    callback(window).map_err(|error| error.into())
                })
                .map_err(|err| err.boxed().into())
        })
    })
}

fn with_window_mut<T>(
    event_loop: *mut ValueBox<PollingEventLoop>,
    window_ref: *mut ValueBox<WindowRef>,
    callback: impl FnOnce(&mut Window, &mut WindowRef) -> Result<T>,
) -> Result<T> {
    event_loop.to_ref().and_then(|mut event_loop| {
        window_ref.to_ref().and_then(|mut window_ref| {
            event_loop
                .with_window_mut(&window_ref.id(), |window, _| {
                    callback(window, window_ref.deref_mut()).map_err(|error| error.into())
                })
                .map_err(|err| err.boxed().into())
        })
    })
}

/// Return the raw window handle that can be used to create a native rendering context.
/// Must only be called from the main thread
#[no_mangle]
pub fn winit_window_ref_raw_window_handle(
    event_loop: *mut ValueBox<PollingEventLoop>,
    window_ref: *mut ValueBox<WindowRef>,
) -> *mut ValueBox<RawWindowHandle> {
    with_window(event_loop, window_ref, |window| {
        Ok(window.raw_window_handle())
    })
    .into_raw()
}

/// Request the window to redraw. Can be called from any thread.
#[no_mangle]
pub fn winit_window_ref_request_redraw(
    event_loop: *mut ValueBox<PollingEventLoop>,
    window_ref: *mut ValueBox<WindowRef>,
) {
    with_window(event_loop, window_ref, |window| Ok(window.request_redraw())).log();
}

/// Get the scaled factor of the window. Can be called from the any thread.
#[no_mangle]
pub fn winit_window_ref_get_scale_factor(window_ref: *mut ValueBox<WindowRef>) -> f64 {
    window_ref
        .to_ref()
        .and_then(|window_ref| window_ref.scale_factor().map_err(|err| err.boxed().into()))
        .or_log(1.0)
}

/// Get the inner size of the window. Can be called from the any thread.
#[no_mangle]
pub fn winit_window_ref_get_inner_size(
    window_ref: *mut ValueBox<WindowRef>,
    inner_size: *mut ValueBox<BoxerSizeU32>,
) {
    window_ref
        .to_ref()
        .and_then(|window_ref| {
            inner_size.to_ref().and_then(|mut inner_size| {
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
pub fn winit_window_ref_set_inner_size(
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
pub fn winit_window_ref_get_position(
    window_ref: *mut ValueBox<WindowRef>,
    position: *mut ValueBox<BoxerPointI32>,
) {
    window_ref
        .to_ref()
        .and_then(|window_ref| {
            position.to_ref().and_then(|mut position| {
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
pub fn winit_window_ref_set_position(
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
pub fn winit_window_ref_get_id(
    window_ref: *mut ValueBox<WindowRef>,
    id: *mut ValueBox<BoxerUint128>,
) {
    window_ref
        .to_ref()
        .and_then(|window_ref| {
            id.to_ref().and_then(|mut id| {
                let window_id: BoxerUint128 = winit_convert_window_id(window_ref.id().clone());
                id.low = window_id.low;
                id.high = window_id.high;
                Ok(())
            })
        })
        .log();
}

#[no_mangle]
pub fn winit_window_ref_get_raw_id(
    window_ref: *mut ValueBox<WindowRef>,
) -> *mut ValueBox<WindowId> {
    window_ref
        .to_ref()
        .and_then(|window_ref| Ok(window_ref.id()))
        .into_raw()
}

#[no_mangle]
pub fn winit_window_ref_set_title(
    event_loop: *mut ValueBox<PollingEventLoop>,
    window_ref: *mut ValueBox<WindowRef>,
    title: *mut ValueBox<BoxerString>,
) {
    with_window_mut(event_loop, window_ref, |window, _window_ref| {
        title.to_ref().and_then(|title| {
            window.set_title(title.to_string().as_ref());
            Ok(())
        })
    })
    .log();
}

#[no_mangle]
pub fn winit_window_ref_set_cursor_icon(
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
pub fn winit_window_ref_set_maximized(
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

#[cfg(target_os = "macos")]
#[no_mangle]
pub fn winit_window_ref_get_ns_view(
    event_loop: *mut ValueBox<PollingEventLoop>,
    window_ref: *mut ValueBox<WindowRef>,
) -> cocoa::base::id {
    with_window(event_loop, window_ref, |window| {
        Ok(window.ns_view() as cocoa::base::id)
    })
    .or_log(cocoa::base::nil)
}

#[no_mangle]
pub fn winit_window_ref_destroy(
    event_loop: *mut ValueBox<PollingEventLoop>,
    window_ref: &mut *mut ValueBox<WindowRef>,
) {
    event_loop
        .to_ref()
        .and_then(|mut event_loop| {
            window_ref.to_ref().and_then(|window_ref| {
                event_loop
                    .destroy_window(&window_ref.id())
                    .map_err(|error| error.boxed().into())
            })
        })
        .log();
    window_ref.drop();
}
