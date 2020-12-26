#![cfg(target_os = "macos")]

use boxer::{ValueBox, ValueBoxPointer};
use winit::platform::macos::{WindowBuilderExtMacOS, WindowExtMacOS};
use winit::window::{WindowBuilder, Window};
use cocoa::{appkit::NSView, base::id as cocoa_id};
use cocoa::base::{nil, YES, NO};

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

#[no_mangle]
pub fn winit_window_get_ns_view(window_ptr: *mut ValueBox<Window>) -> cocoa_id {
    window_ptr.with_not_null_return(nil, |window| {
        window.ns_view() as cocoa_id
    })
}

#[no_mangle]
pub fn winit_ns_view_set_wants_layer(ns_view: cocoa_id, wants_layer: bool) {
    unsafe { ns_view.setWantsLayer(if wants_layer { YES } else { NO }) }
}

#[no_mangle]
pub fn winit_ns_view_set_layer(ns_view: cocoa_id, layer: cocoa_id) {
    unsafe { ns_view.setLayer(layer) }
}