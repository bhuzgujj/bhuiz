mod commons;

use crate::bindings::{vkDestroyInstance, vkDestroySurfaceKHR, SDL_Vulkan_CreateSurface, SDL_Window, Uint32, VkInstance, VkSurfaceKHR};
use crate::gpu::vulkan::commons::create_vulkan_instance;
use bhomz::error::BhomzResult;
use std::os::raw::c_char;
use std::ptr::null;

pub struct Gpu {
	instance: VkInstance,
	surface: VkSurfaceKHR,
}

impl Gpu {
	#[cfg(feature = "sdl")]
	pub fn init(app_name: &str, ext_count: Uint32, ext: Vec<*const c_char>, window: *mut SDL_Window) -> BhomzResult<Self> {
		let instance = create_vulkan_instance(app_name, ext_count, ext)?;
		let surface = unsafe {
			let mut surface_getter = Box::new_uninit();
			SDL_Vulkan_CreateSurface(window, instance, surface_getter.as_mut_ptr());
			surface_getter.assume_init_read()
		};


		Ok(Self {
			instance,
			surface
		})
	}

	pub fn close(self) {
		unsafe {
			vkDestroySurfaceKHR(self.instance, self.surface, null());
			vkDestroyInstance(self.instance, null());
		}
	}
}