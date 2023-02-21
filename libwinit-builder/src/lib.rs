use shared_library_builder::{GitLocation, LibraryLocation, RustLibrary};

pub fn libwinit(version: Option<impl Into<String>>) -> RustLibrary {
    RustLibrary::new(
        "Winit",
        LibraryLocation::Git(GitLocation::github("feenkcom", "libwinit").tag_or_latest(version)),
    )
    .package("libwinit")
    .feature("phlow")
}

pub fn latest_libwinit() -> RustLibrary {
    let version: Option<String> = None;
    libwinit(version)
}
