use geometry_box::SizeBox;
use value_box::{ReturnBoxerResult, ValueBox, ValueBoxPointer};
use winit::monitor::MonitorHandle;

#[no_mangle]
pub extern "C" fn winit_monitor_get_hidpi_factor(
    monitor_handle: *mut ValueBox<MonitorHandle>,
) -> f64 {
    monitor_handle
        .with_ref_ok(|monitor_handle| monitor_handle.scale_factor())
        .or_log(1.0)
}

#[no_mangle]
pub extern "C" fn winit_monitor_get_size(
    monitor_handle: *mut ValueBox<MonitorHandle>,
    size: *mut ValueBox<SizeBox<u32>>,
) {
    monitor_handle
        .with_ref(|monitor_handle| {
            size.with_mut_ok(|size| {
                let monitor_size = monitor_handle.size();
                size.width = monitor_size.width;
                size.height = monitor_size.height;
            })
        })
        .log();
}

#[no_mangle]
pub extern "C" fn winit_monitor_drop(ptr: *mut ValueBox<MonitorHandle>) {
    ptr.release();
}
