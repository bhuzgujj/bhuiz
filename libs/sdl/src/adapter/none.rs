use crate::bindings::{SDL_Window, SDL_WindowFlags_SDL_WINDOW_SHOWN, Uint32};
use interfaces::graphics::{GpuAdapter, MediaResult};

pub(crate) const FLAGS: Uint32 = (SDL_WindowFlags_SDL_WINDOW_SHOWN) as Uint32;

pub(crate) struct DefaultGpu;

impl GpuAdapter for DefaultGpu {
	fn close(self) {
	}
}

pub(crate) type Gpu = DefaultGpu;

pub(crate) fn init_gpu(_app_name: &str, _engine_name: &str, _window: *mut SDL_Window) -> MediaResult<crate::abs::Gpu> {
	Ok(DefaultGpu)
}