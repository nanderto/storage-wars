//! Build script for the views crate.
//!
//! Currently a no-op placeholder; extend here for code generation,
//! asset embedding, or platform-specific link flags.

fn main() {
    // Re-run only when this file itself changes.
    println!("cargo:rerun-if-changed=build.rs");

    // Platform-specific link flags for GPUI's Metal / DirectX backends.
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    match target_os.as_str() {
        "macos" => {
            println!("cargo:rustc-link-lib=framework=Metal");
            println!("cargo:rustc-link-lib=framework=MetalKit");
            println!("cargo:rustc-link-lib=framework=Cocoa");
            println!("cargo:rustc-link-lib=framework=QuartzCore");
        }
        "windows" => {
            println!("cargo:rustc-link-lib=d3d11");
            println!("cargo:rustc-link-lib=dxgi");
        }
        _ => {}
    }
}