use shared_library_builder::{GitLocation, LibraryLocation, RustLibrary};

pub fn libwinit(version: impl Into<String>) -> RustLibrary {
    RustLibrary::new(
        "Winit",
        LibraryLocation::Git(GitLocation::github("feenkcom", "libwinit").tag(version)),
    )
    .package("libwinit")
}
