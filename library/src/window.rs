use crate::enums::WinitCursorIcon;
use crate::event_loop::WinitEventLoop;
use crate::winit_convert_window_id;
use boxer::number::BoxerUint128;
use boxer::point::BoxerPointI32;
use boxer::size::BoxerSizeU32;
use boxer::string::BoxerString;
use boxer::{ValueBox, ValueBoxPointer, ValueBoxPointerReference};
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::window::Window;
use winit::window::WindowBuilder;

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
    _ptr_window: *mut ValueBox<Window>,
) -> *mut ValueBox<RawWindowHandle> {
    _ptr_window.with_not_null_return(std::ptr::null_mut(), |context| {
        ValueBox::new(context.raw_window_handle()).into_raw()
    })
}

#[no_mangle]
pub fn winit_window_request_redraw(window_ptr: *mut ValueBox<Window>) {
    window_ptr.with_not_null(|window| window.request_redraw());
}

#[no_mangle]
pub fn winit_window_get_scale_factor(window_ptr: *mut ValueBox<Window>) -> f64 {
    window_ptr.with_not_null_return(1.0, |window| window.scale_factor())
}

#[no_mangle]
pub fn winit_window_get_inner_size(
    window_ptr: *mut ValueBox<Window>,
    size_ptr: *mut ValueBox<BoxerSizeU32>,
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
pub fn winit_window_set_inner_size(window_ptr: *mut ValueBox<Window>, width: u32, height: u32) {
    window_ptr.with_not_null(|window| window.set_inner_size(PhysicalSize::new(width, height)));
}

#[no_mangle]
pub fn winit_window_get_position(
    window_ptr: *mut ValueBox<Window>,
    position_ptr: *mut ValueBox<BoxerPointI32>,
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
pub fn winit_window_set_position(window_ptr: *mut ValueBox<Window>, x: i32, y: i32) {
    window_ptr.with_not_null(|window| window.set_outer_position(PhysicalPosition::new(x, y)));
}

#[no_mangle]
pub fn winit_window_get_id(window_ptr: *mut ValueBox<Window>, id_ptr: *mut ValueBox<BoxerUint128>) {
    window_ptr.with_not_null(|window| {
        id_ptr.with_not_null(|number| {
            let id: BoxerUint128 = winit_convert_window_id(window.id());
            number.low = id.low;
            number.high = id.high
        });
    });
}

#[no_mangle]
pub fn winit_window_set_title(
    window_ptr: *mut ValueBox<Window>,
    title_ptr: *mut ValueBox<BoxerString>,
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

#[no_mangle]
pub fn winit_window_drop(ptr: &mut *mut ValueBox<Window>) {
    ptr.drop();
}
