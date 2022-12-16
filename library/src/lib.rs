#[macro_use]
extern crate log;

use std::mem::transmute_copy;

use geometry_box::U128Box;
use string_box::StringBox;
use value_box::{ValueBox, ValueBoxPointer};
use winit::window::WindowId;

pub use enums::{WinitCursorIcon, WinitUserEvent};
pub use error::{Result, WinitError};
pub use ffi::*;
pub use polling_event_loop::*;
pub use window_ref::WindowRef;

pub mod enums;
pub mod event_loop;
pub mod events;
pub mod window;
pub mod window_builder;

mod error;
mod ffi;
mod polling_event_loop;
mod window_ref;

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
pub extern "C" fn winit_println(_ptr_message: *mut ValueBox<StringBox>) {
    _ptr_message.with_not_null(|message| println!("{}", message.to_string()));
}

#[no_mangle]
pub extern "C" fn winit_print(_ptr_message: *mut ValueBox<StringBox>) {
    _ptr_message.with_not_null(|message| print!("{}", message.to_string()));
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
