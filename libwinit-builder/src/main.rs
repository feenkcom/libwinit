use std::error::Error;

use libwinit_builder::latest_libwinit;
use shared_library_builder::build_standalone;

fn main() -> Result<(), Box<dyn Error>> {
    build_standalone(|_| Ok(Box::new(latest_libwinit())))
}
