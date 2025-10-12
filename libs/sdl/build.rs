extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
	let vulkan_sdk = env::var("VK_SDK_PATH").unwrap();
	if !PathBuf::from("src").join("bindings.rs").exists() {
		let bindings = bindgen::Builder::default()
			.headers([
				format!("{vulkan_sdk}/Include/SDL2/SDL.h"),
				format!("{vulkan_sdk}/Include/SDL2/SDL_vulkan.h"),
			])
			.clang_arg(format!("-I{vulkan_sdk}/Include/SDL2"))
			.generate()
			.expect("Unable to generate bindings");

		bindings
			.write_to_file("src/bindings.rs")
			.expect("Couldn't write bindings!");
	}
}