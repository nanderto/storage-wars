//! Build script — sets up platform-specific linker flags for GPUI.

fn main() {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();

    match target_os.as_str() {
        "macos" => {
            println!("cargo:rustc-link-lib=framework=Cocoa");
            println!("cargo:rustc-link-lib=framework=Metal");
            println!("cargo:rustc-link-lib=framework=MetalKit");
            println!("cargo:rustc-link-lib=framework=CoreGraphics");
            println!("cargo:rustc-link-lib=framework=CoreText");
            println!("cargo:rustc-link-lib=framework=CoreFoundation");
            println!("cargo:rustc-link-lib=framework=AppKit");
        }
        "windows" => {
            println!("cargo:rustc-link-lib=d3d11");
            println!("cargo:rustc-link-lib=dxgi");
            println!("cargo:rustc-link-lib=dwrite");
            println!("cargo:rustc-link-lib=d2d1");
        }
        "linux" => {
            println!("cargo:rustc-link-lib=X11");
            println!("cargo:rustc-link-lib=xcb");
            println!("cargo:rustc-link-lib=vulkan");
        }
        _ => {}
    }
}