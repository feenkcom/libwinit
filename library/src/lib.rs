#[macro_use]
extern crate log;

#[cfg(target_os = "macos")]
extern crate cocoa;

pub mod enums;
pub mod event_loop;
pub mod events;
pub mod window;
pub mod window_builder;

#[cfg(target_os = "macos")]
#[path = "platform/macos.rs"]
mod ext;

mod error;
#[cfg(all(not(target_os = "macos")))]
#[path = "platform/others.rs"]
mod ext;
mod ffi;
mod polling_event_loop;
mod window_ref;

pub use ffi::*;

pub use error::{Result, WinitError};

pub use polling_event_loop::*;
pub use window_ref::WindowRef;

use boxer::number::BoxerUint128;
use boxer::string::BoxerString;

use winit::window::WindowId;

use boxer::{ValueBox, ValueBoxPointer};
use std::mem::transmute_copy;

///////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////// L I B R A R Y /////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////

#[no_mangle]
pub fn winit_test() -> bool {
    return true;
}

#[no_mangle]
pub fn winit_init_logger() {
    env_logger::init();
}

#[no_mangle]
pub fn winit_println(_ptr_message: *mut ValueBox<BoxerString>) {
    _ptr_message.with_not_null(|message| println!("{}", message.to_string()));
}

#[no_mangle]
pub fn winit_print(_ptr_message: *mut ValueBox<BoxerString>) {
    _ptr_message.with_not_null(|message| print!("{}", message.to_string()));
}

pub fn winit_convert_window_id(window_id: WindowId) -> BoxerUint128 {
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
