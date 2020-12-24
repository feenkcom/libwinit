use boxer::ValueBox;
use winit::window::WindowBuilder;

#[no_mangle]
pub fn winit_window_builder_with_full_size(
    _ptr_window_builder: *mut ValueBox<WindowBuilder>,
    _with_full_size: bool,
) {
}
