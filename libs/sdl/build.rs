extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
	println!("cargo:rerun-if-changed=build.rs");
	let is_vulkan_enabled = env::var("CARGO_FEATURE_VULKAN").is_ok();

	// TODO: allow for building from source
	let vulkan_sdk = env::var("VK_SDK_PATH").unwrap();
	let mut headers = Vec::new();
	headers.push(format!("{vulkan_sdk}/Include/SDL2/SDL.h"));
	if is_vulkan_enabled {
		headers.push(format!("{vulkan_sdk}/Include/SDL2/SDL_vulkan.h"));
	}
	if !PathBuf::from("src").join("bindings.rs").exists() {
		let bindings = bindgen::Builder::default()
			.headers(headers)
			.clang_arg(format!("-I{vulkan_sdk}/Include/SDL2"))
			.generate()
			.expect("Unable to generate bindings");

		bindings
			.write_to_file("src/bindings.rs")
			.expect("Couldn't write bindings!");
	}
}