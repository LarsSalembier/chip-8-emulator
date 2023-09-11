fn main() {
    // Use pkg-config to find and link SDL2
    if let Err(e) = pkg_config::probe_library("sdl2") {
        panic!("Failed to find and link SDL2 using pkg-config: {}", e);
    }
}
