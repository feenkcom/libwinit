use winit::dpi::LogicalSize;
#[cfg(target_os = "macos")]
use winit::platform::macos::WindowBuilderExtMacOS;
use winit::window::{WindowBuilder, WindowLevel};

use string_box::StringBox;
use value_box::{ReturnBoxerResult, ValueBox, ValueBoxPointer};

#[no_mangle]
pub extern "C" fn winit_window_builder_new() -> *mut ValueBox<WindowBuilder> {
    ValueBox::new(WindowBuilder::new()).into_raw()
}

#[no_mangle]
pub extern "C" fn winit_window_builder_drop(window_builder: *mut ValueBox<WindowBuilder>) {
    window_builder.release();
}

#[no_mangle]
pub extern "C" fn winit_window_builder_with_title(
    window_builder: *mut ValueBox<WindowBuilder>,
    window_title: *mut ValueBox<StringBox>,
) {
    window_title
        .with_ref_ok(|window_title| {
            window_builder
                .replace_value(|window_builder| window_builder.with_title(window_title.to_string()))
        })
        .log();
}

#[no_mangle]
pub extern "C" fn winit_window_builder_with_decorations(
    window_builder: *mut ValueBox<WindowBuilder>,
    with_decorations: bool,
) {
    window_builder
        .replace_value(|window_builder| window_builder.with_decorations(with_decorations))
        .log();
}

#[no_mangle]
pub extern "C" fn winit_window_builder_with_transparency(
    window_builder: *mut ValueBox<WindowBuilder>,
    with_transparency: bool,
) {
    window_builder
        .replace_value(|window_builder| window_builder.with_transparent(with_transparency))
        .log();
}

#[no_mangle]
pub extern "C" fn winit_window_builder_with_resizable(
    window_builder: *mut ValueBox<WindowBuilder>,
    with_resizable: bool,
) {
    window_builder
        .replace_value(|window_builder| window_builder.with_resizable(with_resizable))
        .log();
}

#[no_mangle]
pub extern "C" fn winit_window_builder_with_dimensions(
    window_builder: *mut ValueBox<WindowBuilder>,
    width: f64,
    height: f64,
) {
    window_builder
        .replace_value(|window_builder| {
            window_builder.with_inner_size(LogicalSize::new(width, height))
        })
        .log();
}

#[no_mangle]
pub extern "C" fn winit_window_builder_with_maximized(
    window_builder: *mut ValueBox<WindowBuilder>,
    with_maximized: bool,
) {
    window_builder
        .replace_value(|window_builder| window_builder.with_maximized(with_maximized))
        .log();
}

#[no_mangle]
pub extern "C" fn winit_window_builder_with_visibility(
    window_builder: *mut ValueBox<WindowBuilder>,
    with_visibility: bool,
) {
    window_builder
        .replace_value(|window_builder| window_builder.with_visible(with_visibility))
        .log();
}

#[no_mangle]
pub extern "C" fn winit_window_builder_with_always_on_top(
    window_builder: *mut ValueBox<WindowBuilder>,
    with_always_on_top: bool,
) {
    window_builder
        .replace_value(|window_builder| {
            let level = match with_always_on_top {
                true => WindowLevel::AlwaysOnTop,
                false => WindowLevel::Normal,
            };
            window_builder.with_window_level(level)
        })
        .log();
}

#[cfg(not(target_os = "macos"))]
#[no_mangle]
pub extern "C" fn winit_window_builder_with_full_size(
    _ptr_window_builder: *mut ValueBox<WindowBuilder>,
    _with_full_size: bool,
) {
}

#[cfg(target_os = "macos")]
#[no_mangle]
pub extern "C" fn winit_window_builder_with_full_size(
    window_builder: *mut ValueBox<WindowBuilder>,
    with_full_size: bool,
) {
    window_builder
        .replace_value(|window_builder| {
            window_builder
                .with_titlebar_transparent(with_full_size)
                .with_fullsize_content_view(with_full_size)
                .with_title_hidden(with_full_size)
        })
        .log();
}
