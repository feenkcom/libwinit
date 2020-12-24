#![cfg(target_os = "macos")]

use boxer::{ValueBox, ValueBoxPointer};
use winit::platform::macos::WindowBuilderExtMacOS;
use winit::window::WindowBuilder;

#[no_mangle]
pub fn winit_window_builder_with_full_size(
    mut _ptr_window_builder: *mut ValueBox<WindowBuilder>,
    _with_full_size: bool,
) {
    _ptr_window_builder.with_not_null_value_mutate(|builder| {
        builder
            .with_titlebar_transparent(_with_full_size)
            .with_fullsize_content_view(_with_full_size)
            .with_title_hidden(_with_full_size)
    })
}
