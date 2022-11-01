use geometry_box::{PointBox, SizeBox, U128Box};
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use string_box::StringBox;
use value_box::{ReturnBoxerResult, ValueBox, ValueBoxPointer};
use winit::dpi::{PhysicalPosition, PhysicalSize};
#[cfg(target_os = "macos")]
use winit::platform::macos::WindowBuilderExtMacOS;
#[cfg(target_os = "macos")]
use winit::platform::macos::WindowExtMacOS;
#[cfg(target_os = "windows")]
use winit::platform::windows::WindowExtWindows;
use winit::window::Window;
use winit::window::WindowBuilder;

use crate::enums::WinitCursorIcon;
use crate::event_loop::WinitEventLoop;
use crate::winit_convert_window_id;

#[no_mangle]
pub fn winit_create_window(
    event_loop_ptr: *mut ValueBox<WinitEventLoop>,
    mut window_builder_ptr: *mut ValueBox<WindowBuilder>,
) -> *mut ValueBox<Window> {
    if event_loop_ptr.is_null() {
        error!("Event loop is null");
        return std::ptr::null_mut();
    }

    if window_builder_ptr.is_null() {
        error!("Window builder is null");
        return std::ptr::null_mut();
    }

    event_loop_ptr.with_not_null_return(std::ptr::null_mut(), |event_loop| {
        window_builder_ptr.with_not_null_value_consumed_return(
            std::ptr::null_mut(),
            |window_builder| {
                debug!("Window builder: {:?}", &window_builder);

                match window_builder.build(event_loop) {
                    Ok(window) => ValueBox::new(window).into_raw(),
                    Err(err) => {
                        error!("Could not create window {:?}", err);
                        std::ptr::null_mut()
                    }
                }
            },
        )
    })
}

///////////////////////////////////////////////////////////////////////////////////////
///////////////////////////// W I N D O W   A C C E S S O R S /////////////////////////
///////////////////////////////////////////////////////////////////////////////////////

#[no_mangle]
pub fn winit_windowed_context_raw_window_handle(
    window: *mut ValueBox<Window>,
) -> *mut ValueBox<RawWindowHandle> {
    window.with_ref(Window::raw_window_handle).into_raw()
}

#[no_mangle]
pub fn winit_window_request_redraw(window: *mut ValueBox<Window>) {
    window.with_ref(Window::request_redraw).log();
}

#[no_mangle]
pub fn winit_window_get_scale_factor(window: *mut ValueBox<Window>) -> f64 {
    window.with_ref(Window::scale_factor).or_log(1.0)
}

#[no_mangle]
pub fn winit_window_get_inner_size(
    window_ptr: *mut ValueBox<Window>,
    size_ptr: *mut ValueBox<SizeBox<u32>>,
) {
    window_ptr.with_not_null(|window| {
        size_ptr.with_not_null(|size| {
            let window_size: PhysicalSize<u32> = window.inner_size();
            size.width = window_size.width;
            size.height = window_size.height;
        });
    });
}

#[no_mangle]
pub fn winit_window_set_inner_size(window: *mut ValueBox<Window>, width: u32, height: u32) {
    window
        .with_ref(|window| window.set_inner_size(PhysicalSize::new(width, height)))
        .log();
}

#[no_mangle]
pub fn winit_window_get_position(
    window_ptr: *mut ValueBox<Window>,
    position_ptr: *mut ValueBox<PointBox<i32>>,
) {
    window_ptr.with_not_null(|window| {
        position_ptr.with_not_null(|position| match window.outer_position() {
            Ok(physical_position) => {
                position.x = physical_position.x;
                position.y = physical_position.y;
            }
            Err(err) => {
                error!(
                    "[winit_window_get_position] Error getting position: {:?}",
                    err
                );
                position.be_zero()
            }
        })
    });
}

#[no_mangle]
pub fn winit_window_set_position(window: *mut ValueBox<Window>, x: i32, y: i32) {
    window
        .with_ref(|window| window.set_outer_position(PhysicalPosition::new(x, y)))
        .log();
}

#[no_mangle]
pub fn winit_window_get_id(window_ptr: *mut ValueBox<Window>, id_ptr: *mut ValueBox<U128Box>) {
    window_ptr.with_not_null(|window| {
        id_ptr.with_not_null(|number| {
            let id: U128Box = winit_convert_window_id(window.id());
            number.low = id.low;
            number.high = id.high
        });
    });
}

#[no_mangle]
pub fn winit_window_set_title(
    window_ptr: *mut ValueBox<Window>,
    title_ptr: *mut ValueBox<StringBox>,
) {
    window_ptr.with_not_null(|window| {
        title_ptr.with_not_null(|string| window.set_title(string.to_string().as_ref()))
    });
}

#[no_mangle]
pub fn winit_window_set_cursor_icon(
    window_ptr: *mut ValueBox<Window>,
    cursor_icon: WinitCursorIcon,
) {
    window_ptr.with_not_null(|window| window.set_cursor_icon(cursor_icon.into()));
}

#[no_mangle]
pub fn winit_window_set_maximized(window_ptr: *mut ValueBox<Window>, maximized: bool) {
    window_ptr.with_not_null(|window| {
        window.set_maximized(maximized);
    });
}

#[cfg(target_os = "windows")]
#[no_mangle]
pub fn winit_window_get_hwnd(window_ptr: *mut ValueBox<Window>) -> *mut std::ffi::c_void {
    window_ptr.with_not_null_return(std::ptr::null_mut(), |window| unsafe {
        std::mem::transmute(window.hwnd())
    })
}

#[cfg(not(target_os = "macos"))]
#[no_mangle]
pub fn winit_window_builder_with_full_size(
    _ptr_window_builder: *mut ValueBox<WindowBuilder>,
    _with_full_size: bool,
) {
}

#[cfg(target_os = "macos")]
#[no_mangle]
pub fn winit_window_builder_with_full_size(
    mut window_builder_ptr: *mut ValueBox<WindowBuilder>,
    with_full_size: bool,
) {
    window_builder_ptr.with_not_null_value_mutate(|builder| {
        builder
            .with_titlebar_transparent(with_full_size)
            .with_fullsize_content_view(with_full_size)
            .with_title_hidden(with_full_size)
    })
}

#[cfg(target_os = "macos")]
#[no_mangle]
pub fn winit_window_get_ns_view(window_ptr: *mut ValueBox<Window>) -> cocoa::base::id {
    window_ptr.with_not_null_return(cocoa::base::nil, |window| {
        window.ns_view() as cocoa::base::id
    })
}

#[no_mangle]
pub fn winit_window_drop(ptr: *mut ValueBox<Window>) {
    ptr.release();
}
