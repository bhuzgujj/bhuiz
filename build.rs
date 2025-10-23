use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::{read_to_string, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

const VERSION_FILE_NAME: &str = ".binding_version";

fn need_rebuild() -> bool {
	let path = PathBuf::from(VERSION_FILE_NAME);
	if path.exists() && path.is_file() {
		if let Ok(ctnt) = read_to_string(path) {
			return get_current_version() == ctnt;
		}
	}
	true
}

fn get_current_version() -> String {
	let mut version = Vec::new();

	#[cfg(feature = "vulkan")]
	version.push("vulkan");

	#[cfg(feature = "sdl")]
	version.push("sdl");

	version.push(env!("CARGO_PKG_VERSION"));

	version.join("-")
}

fn parse_dotenv() -> HashMap<String, String> {
	let result = read_to_string(".env");
	let mut vals = HashMap::new();
	if result.is_err() {
		return vals;
	}

	for line in result.expect(".env not found").lines() {
		let comment_split: Vec<&str> = line.split("#").collect();
		let without_comment = comment_split.first().expect("Split did not return a prefix");
		let equal_split: Vec<&str> = without_comment.split("=").collect();
		if equal_split.len() < 2 {
			continue;
		}
		let key = equal_split.first().expect("No key for a line in .env");
		let value = equal_split[1..].join("=").split('/').map(|file| {
			if let Some(env_var) = file.strip_prefix('$') {
				let substitution = env::var(env_var);
				if let Ok(sub) = substitution {
					return sub.replace("\\", "/")
				}
			}
			file.to_string()
		}).collect::<Vec<String>>().join("/");
		vals.insert(key.to_string(), value);
	}

	vals
}

fn main() {
	println!("cargo:rerun-if-changed=build.rs");
	let dotenv = parse_dotenv();
	let mut link: HashMap<String, HashSet<String>> = HashMap::new();
	let mut headers_directory = HashSet::new();
	let mut header_paths: Vec<String> = Vec::new();

	#[cfg(feature = "vulkan")]
	create_vulkan_bindings(
		&dotenv,
		&mut link,
		&mut headers_directory,
		&mut header_paths
	);

	#[cfg(feature = "sdl")]
	create_sdl_bindings(
		&dotenv,
		&mut link,
		&mut headers_directory,
		&mut header_paths
	);

	if need_rebuild() {
		let bindings = bindgen::Builder::default()
			.headers(header_paths)
			.clang_args(headers_directory.iter().map(|p| format!("-I{}", &p)).collect::<Vec<String>>())
			.generate()
			.expect("Unable to generate bindings");

		bindings
			.write_to_file("src/bindings.rs")
			.expect("Couldn't write bindings!");
	}

	for (lib_dir, libs) in link {
		println!("cargo:rustc-link-search=native={}", lib_dir);
		for libs in libs {
			println!("cargo:rustc-link-lib={libs}");
		}
	}

	let path = PathBuf::from(VERSION_FILE_NAME);
	OpenOptions::new()
		.create(true)
		.write(true)
		.truncate(true)
		.open(&path)
		.expect(format!("Unable to access file '{}'", path.display()).as_str())
		.write(get_current_version().as_bytes())
		.expect(format!("Unable to write version to '{}'", path.display()).as_str());
}

#[allow(warnings)]
fn create_vulkan_bindings(
	path: &HashMap<String, String>,
	link: &mut HashMap<String, HashSet<String>>,
	headers_directory: &mut HashSet<String>,
	header_paths: &mut Vec<String>
) {
	let header_dir = path.get("VULKAN_HEADERS").expect("Need 'VULKAN_HEADERS' in .env");
	let lib_dir = path.get("VULKAN_LIB").expect("Need 'VULKAN_LIB' in .env");
	headers_directory.insert(header_dir.clone());
	header_paths.push(format!("{}/vulkan/vulkan.h", header_dir));
	header_paths.push(format!("{}/vulkan/vulkan_core.h", header_dir));
	if !link.contains_key(lib_dir) {
		link.insert(lib_dir.clone(), HashSet::new());
	}
	let libs = link.get_mut(lib_dir).expect(format!("Could not find {lib_dir}").as_str());
	libs.insert("vulkan-1".to_string());
}

#[allow(warnings)]
fn create_sdl_bindings(
	path: &HashMap<String, String>,
	link: &mut HashMap<String, HashSet<String>>,
	headers_directory: &mut HashSet<String>,
	header_paths: &mut Vec<String>
) {
	let header_dir = path.get("SDL_HEADERS").expect("Need 'SDL_HEADERS' in .env");
	let lib_dir = path.get("SDL_LIB").expect("Need 'SDL_LIB' in .env");
	headers_directory.insert(header_dir.clone());
	header_paths.push(format!("{}/SDL.h", header_dir));

	#[cfg(feature = "vulkan")]
	header_paths.push(format!("{}/SDL_vulkan.h", header_dir));

	if !link.contains_key(lib_dir) {
		link.insert(lib_dir.clone(), HashSet::new());
	}
	let libs = link.get_mut(lib_dir).expect(format!("Could not find {lib_dir}").as_str());
	libs.insert("SDL2main".to_string());
	libs.insert("SDL2".to_string());
	libs.insert("user32".to_string());
	libs.insert("gdi32".to_string());
	libs.insert("dxguid".to_string());
	libs.insert("winmm".to_string());
}