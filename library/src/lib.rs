#![allow(non_snake_case)]

#[macro_use]
extern crate log;
#[cfg(feature = "phlow")]
#[macro_use]
extern crate phlow;
#[cfg(feature = "phlow")]
extern crate phlow_extensions;

// #[macro_use]
// extern crate value_box;

use std::mem::transmute_copy;

use geometry_box::U128Box;
#[cfg(feature = "phlow")]
use phlow_extensions::CoreExtensions;
// Re-export everything from the `value_box_ffi` in order to tell Rust to include
// the corresponding `no_mangle` functions.
#[cfg(feature = "phlow")]
pub use phlow_ffi::*;
use string_box::StringBox;
use value_box::{ReturnBoxerResult, ValueBox, ValueBoxPointer};
// Re-export everything from the `value_box_ffi` in order to tell Rust to include
// the corresponding `no_mangle` functions.
pub use raw_window_handle_extensions::*;
pub use value_box_ffi::*;
use winit::window::WindowId;

pub use enums::{WinitCursorIcon, WinitUserEvent};
pub use error::{Result, WinitError};
pub use ffi::*;
pub use polling_event_loop::*;
pub use window_ref::WindowRef;

mod enums;
mod error;
mod event_loop;
#[cfg(any(
    target_os = "windows",
    target_os = "macos",
    target_os = "android",
    target_os = "linux",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd"
))]
mod event_loop_run_return;
mod events;
mod ffi;
mod polling_event_loop;
mod window;
mod window_builder;
mod window_ref;

#[cfg(feature = "phlow")]
import_extensions!(CoreExtensions);

///////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////// L I B R A R Y /////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////

#[no_mangle]
pub extern "C" fn winit_test() -> bool {
    return true;
}

#[no_mangle]
pub extern "C" fn winit_init_logger() {
    env_logger::init();
}

#[no_mangle]
pub extern "C" fn winit_println(message: *mut ValueBox<StringBox>) {
    message
        .with_ref_ok(|message| println!("{}", message.to_string()))
        .log();
}

#[no_mangle]
pub extern "C" fn winit_print(message: *mut ValueBox<StringBox>) {
    message
        .with_ref_ok(|message| print!("{}", message.to_string()))
        .log();
}

pub fn winit_convert_window_id(window_id: WindowId) -> U128Box {
    let size = std::mem::size_of::<WindowId>();

    let id_128: u128 = match size {
        4 => {
            // u32
            let id: u32 = unsafe { transmute_copy::<WindowId, u32>(&window_id) };
            id as u128
        }
        8 => {
            // u64
            let id: u64 = unsafe { transmute_copy::<WindowId, u64>(&window_id) };
            id as u128
        }
        16 => {
            //u128
            let id: u128 = unsafe { transmute_copy::<WindowId, u128>(&window_id) };
            id
        }
        _ => {
            eprintln!("Unknown size of window id ({:?})", window_id);
            0 as u128
        }
    };

    id_128.into()
}
