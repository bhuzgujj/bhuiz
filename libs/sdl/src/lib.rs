use interfaces::graphics::driver::GpuInterface;
use interfaces::graphics::{errors::GraphicsError, errors::GraphicsResult, GraphicsInterface};
use interfaces::{cstr_ptr, printable_cstr_ptr};
use std::time::Instant;
use crate::bindings::*;

#[allow(warnings)]
mod bindings;

const QUIT_EVT: Uint32 = SDL_EventType_SDL_QUIT as Uint32;

#[cfg(feature = "vulkan")]
mod abs {
	use crate::bindings::{SDL_Vulkan_GetInstanceExtensions, SDL_Window};
	use interfaces::graphics::errors::GraphicsResult;

	pub(crate) type Gpu = vulkan::Vulkan;

	pub(crate) unsafe fn init_gpu(window: *mut SDL_Window) -> GraphicsResult<Gpu> {
		unsafe {
			let mut count = 0u32 as std::ffi::c_uint;
			SDL_Vulkan_GetInstanceExtensions(window, &mut count, std::ptr::null_mut());
			let mut ext: Vec<*const std::os::raw::c_char> = vec![std::ptr::null(); count as usize];
			SDL_Vulkan_GetInstanceExtensions(window, &mut count, ext.as_mut_ptr());
			match Gpu::init(count, ext) {
				Ok(gpu) => Ok(gpu),
				Err(err) => Err(err.into())
			}
		}
	}
}

pub struct SDL {
	gpu_interface: abs::Gpu,
	last_drawn: Instant,
	running: bool,

	// C STUFF
	window: *mut SDL_Window,
	event_ptr: Box<SDL_Event>,
}

impl GraphicsInterface for SDL {
	fn init() -> GraphicsResult<Self> {
		unsafe {
			if SDL_Init(SDL_INIT_VIDEO) != 0 {
				return Err(GraphicsError(format!("SDL_Init failed: {}", printable_cstr_ptr(SDL_GetError()))))
			}
			let window = SDL_CreateWindow(
				cstr_ptr("DDD"),
				200,
				200,
				1200,
				600,
				(SDL_WindowFlags_SDL_WINDOW_VULKAN | SDL_WindowFlags_SDL_WINDOW_SHOWN) as Uint32
			);
			let gpu_interface = abs::init_gpu(window)?;
			Ok(
				SDL {
					gpu_interface,
					last_drawn: Instant::now(),
					running: true,

					window,
					event_ptr: Box::new(SDL_Event{
						type_ : 0u32
					}),
				}
			)
		}
	}

	fn is_running(&self) -> bool {
		self.running
	}
//
	fn process_io(&mut self) {
		unsafe {
			while SDL_PollEvent(self.event_ptr.as_mut()) != 0 {
				match self.event_ptr.as_ref().type_ {
					QUIT_EVT => {
						self.running = false;
						return;
					},
					_ => {}
				}
			}
		}
	}

	fn draw(&mut self) {
		let timelaps = self.last_drawn.saturating_duration_since(Instant::now()).as_micros();
		unsafe {
			if timelaps > 50 {
				SDL_Delay(1);
			} else {
				SDL_Delay((50 - timelaps) as Uint32)
			}
		}
		self.last_drawn = Instant::now();
	}

	fn close(self) {
		unsafe {
			if !self.window.is_null() {
				SDL_DestroyWindow(self.window);
			} else {
				println!("Why window null?")
			}
			self.gpu_interface.close();
			SDL_Quit()
		}
	}
}