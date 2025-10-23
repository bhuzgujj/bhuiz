use bhomz::cstr::ToCCharPtr;
use bhomz::error::BhomzResult;
use crate::bindings::{SDL_Window, SDL_CreateWindow, SDL_WindowFlags_SDL_WINDOW_SHOWN, SDL_WindowFlags_SDL_WINDOW_VULKAN, Uint32, SDL_DestroyWindow};
use crate::gpu::Gpu;

#[cfg(feature = "vulkan")]
#[allow(unused)]
const FLAGS: Uint32 = (SDL_WindowFlags_SDL_WINDOW_SHOWN | SDL_WindowFlags_SDL_WINDOW_VULKAN) as Uint32;

#[cfg(not(feature = "vulkan"))]
#[allow(unused)]
const FLAGS: Uint32 = SDL_WindowFlags_SDL_WINDOW_SHOWN as Uint32;

pub struct Media {
	window: *mut SDL_Window,
	gpu: Gpu,
}

impl Media {
	pub fn new(app_name: &str) -> BhomzResult<Self> {
		let window = unsafe {
			SDL_CreateWindow(
				app_name.to_c_char_ptr(),
				1200,
				600,
				1200,
				600,
				FLAGS
			)
		};
		let gpu = Gpu::new()?;
		Ok(Self{
			window,
			gpu,
		})
	}

	pub fn close(self) {
		unsafe {
			SDL_DestroyWindow(self.window);
			self.gpu.close();
		}
	}
}
