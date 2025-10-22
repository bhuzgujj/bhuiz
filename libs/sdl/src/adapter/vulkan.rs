use crate::adapter::vulkan::internal::{post_init, pre_init};
use crate::bindings::*;
use interfaces::graphics::MediaResult;

pub(crate) const SDL_WINDOW_FLAGS: Uint32 = (SDL_WindowFlags_SDL_WINDOW_VULKAN | SDL_WindowFlags_SDL_WINDOW_SHOWN) as Uint32;

pub(crate) type Gpu = gpu_vulkan::Vulkan;

pub(crate) fn init_gpu(app_name: &str, engine_name: &str, window: *mut SDL_Window) -> MediaResult<Gpu> {
	let (count, ext) = pre_init(window);

	match Gpu::init(app_name, engine_name, count, ext) {
		Ok(gpu) => post_init(window, gpu),
		Err(err) => Err(err.into())
	}
}

mod internal {
	use crate::adapter::Gpu;
	use crate::bindings::*;
	use interfaces::graphics::MediaResult;
	use std::ffi::c_uint;
	use std::os::raw::c_char;

	pub(crate) fn pre_init(window: *mut SDL_Window) -> (c_uint, Vec<*const c_char>) {
		let mut count = 0u32 as c_uint;
		unsafe { SDL_Vulkan_GetInstanceExtensions(window, &mut count, std::ptr::null_mut()); }

		let mut ext: Vec<*const c_char> = Vec::with_capacity(count as usize);
		unsafe {
			SDL_Vulkan_GetInstanceExtensions(window, &mut count, ext.as_mut_ptr());
			// Since C doesnt update the len, update here
			ext.set_len(count as usize);
		}
		(count, ext)
	}

	pub(crate) fn post_init(window: *mut SDL_Window, mut gpu: Gpu) -> MediaResult<Gpu> {
		let instance = gpu.instance.as_ptr();
		let surface = gpu.surface.as_mut_ptr();
		unsafe { SDL_Vulkan_CreateSurface(window, *instance as VkInstance, surface as *mut VkSurfaceKHR); }
		gpu.init_devices();
		Ok(gpu)
	}
}