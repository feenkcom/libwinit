use shared_library_builder::{GitLocation, LibraryLocation, RustLibrary};

pub fn libwinit(version: Option<impl Into<String>>) -> RustLibrary {
    let mut location = GitLocation::github("feenkcom", "libwinit");
    if let Some(version) = version {
        location = location.tag(version);
    }
    RustLibrary::new(
        "Winit",
        LibraryLocation::Git(location),
    )
    .package("libwinit")
}

pub fn latest_libwinit() -> RustLibrary {
    let version: Option<String> = None;
    libwinit(version)
}