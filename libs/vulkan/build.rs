extern crate bindgen;

use std::env;

fn main() {
	println!("cargo:rerun-if-changed=build.rs");
	let vulkan_sdk = env::var("VK_SDK_PATH").unwrap();
	let bindings = bindgen::Builder::default()
        .headers([
            format!("{vulkan_sdk}/Include/vulkan/vulkan.h"),
            format!("{vulkan_sdk}/Include/vulkan/vulkan_core.h"),
        ])
        .clang_arg(format!("-I{vulkan_sdk}/Include"))
		.generate()
		.expect("Unable to generate bindings");

	bindings
		.write_to_file("src/bindings.rs")
		.expect("Couldn't write bindings!");
}