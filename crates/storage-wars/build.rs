//! Build script for the `storage-wars` crate.
//!
//! Currently this script only emits platform-specific linker flags required by
//! GPUI on macOS. It is intentionally kept minimal and will be extended when
//! build-time code generation (SQLite schema embedding, asset bundling) is
//! added.

fn main() {
    // Re-run only when this file changes — avoids spurious rebuilds.
    println!("cargo:rerun-if-changed=build.rs");

    emit_platform_link_flags();
}

#[allow(unused_variables)]
fn emit_platform_link_flags() {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();

    match target_os.as_str() {
        "macos" => {
            for framework in &[
                "Metal",
                "CoreFoundation",
                "CoreGraphics",
                "CoreText",
                "AppKit",
                "QuartzCore",
            ] {
                println!("cargo:rustc-link-lib=framework={framework}");
            }
        }
        "linux" => {
            // xcb and xkbcommon are required by GPUI's X11 / Wayland backend.
            for lib in &["xcb", "xkbcommon"] {
                println!("cargo:rustc-link-lib={lib}");
            }
        }
        _ => {}
    }
}