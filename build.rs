use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::{read_to_string, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

const VERSION_FILE_NAME: &str = ".binding_version";

fn need_rebuild(headers_directory: &HashSet<String>, header_paths: &Vec<String>) -> bool {
	let path = PathBuf::from(VERSION_FILE_NAME);
	if path.exists() && path.is_file() {
		if let Ok(ctnt) = read_to_string(path) {
			return get_current_version(headers_directory, header_paths) != ctnt;
		}
	}
	true
}

fn get_current_version(headers_directory: &HashSet<String>, header_paths: &Vec<String>) -> String {
	format!(
		"{}\n\n{}\n\n{}",
		[
			#[cfg(feature = "vulkan")] "vulkan",
			#[cfg(feature = "sdl")] "sdl",
			env!("CARGO_PKG_VERSION")
		].join("-"),
		headers_directory.iter().map(|e| e.to_string()).collect::<Vec<String>>().join("\n"),
		header_paths.iter().map(|e| e.to_string()).collect::<Vec<String>>().join("\n"),
	)
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
	let dotenv = parse_dotenv();
	let mut link: HashMap<String, HashSet<String>> = HashMap::new();
	let mut headers_directory = HashSet::new();
	let mut header_paths: Vec<String> = Vec::new();

	#[cfg(feature = "vulkan")]
	add_vulkan_bindings(
		&dotenv,
		&mut link,
		&mut headers_directory,
		&mut header_paths
	);

	#[cfg(feature = "sdl")]
	add_sdl_bindings(
		&dotenv,
		&mut link,
		&mut headers_directory,
		&mut header_paths
	);

	if need_rebuild(&headers_directory, &header_paths) {
		let mut bindings = bindgen::Builder::default()
			.headers(&header_paths)
			.clang_args(&headers_directory.iter().map(|p| format!("-I{}", &p)).collect::<Vec<String>>());

		// Derive macro are experimental on assign expressions
		if env::var("CARGO_FEATURE_DEBUG").is_ok() {
			bindings = bindings.derive_debug(true);
		}

		bindings
			.generate()
			.expect("Unable to generate bindings")
			.write_to_file("src/bindings.rs")
			.expect("Couldn't write bindings!");

		let path = PathBuf::from(VERSION_FILE_NAME);
		OpenOptions::new()
			.create(true)
			.write(true)
			.truncate(true)
			.open(&path)
			.unwrap_or_else(|_| panic!("Unable to access file '{}'", path.display()))
			.write_all(get_current_version(&headers_directory, &header_paths).as_bytes())
			.unwrap_or_else(|_| panic!("Unable to write version to '{}'", path.display()));
	}

	for (lib_dir, libs) in link {
		println!("cargo:rustc-link-search=native={}", lib_dir);
		for libs in libs {
			println!("cargo:rustc-link-lib={libs}");
		}
	}
}

#[allow(warnings)]
fn add_vulkan_bindings(
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
	header_paths.push(format!("{}/vulkan/vk_enum_string_helper.h", header_dir));
	if !link.contains_key(lib_dir) {
		link.insert(lib_dir.clone(), HashSet::new());
	}
	let libs = link.get_mut(lib_dir).expect(format!("Could not find {lib_dir}").as_str());
	libs.insert("vulkan-1".to_string());
}

#[allow(warnings)]
fn add_sdl_bindings(
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