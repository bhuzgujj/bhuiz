use std::env;

fn main() {
	let vulkan_sdk = env::var("VK_SDK_PATH").unwrap();
	println!("cargo:rustc-link-search=native={vulkan_sdk}/Lib");
	// Windows
	println!("cargo:rustc-link-lib=vulkan-1");
	println!("cargo:rustc-link-lib=SDL2main");
	println!("cargo:rustc-link-lib=SDL2");
	println!("cargo:rustc-link-lib=user32");
	println!("cargo:rustc-link-lib=gdi32");
	println!("cargo:rustc-link-lib=dxguid");
	println!("cargo:rustc-link-lib=winmm");

	// TODO: Linux

	println!("cargo:rerun-if-changed=build.rs");
}