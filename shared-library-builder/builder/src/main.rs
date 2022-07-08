use std::error::Error;

use shared_library_builder::build_standalone;

use libwinit_library::latest_libwinit;

fn main() -> Result<(), Box<dyn Error>> {
    build_standalone(|_| Ok(Box::new(latest_libwinit())))
}
