use interfaces::graphics::{GpuAdapter, MediaError, MediaResult, MediaInterface};
use interfaces::{cstr_ptr, log_err, printable_cstr_ptr};
use std::time::Instant;
use crate::bindings::*;

#[allow(warnings)]
mod bindings;
mod adapter;

use adapter::*;

const QUIT_EVT: Uint32 = SDL_EventType_SDL_QUIT as Uint32;

pub struct SDL {
	gpu_interface: Gpu,
	last_drawn: Instant,
	running: bool,

	// C STUFF
	window: *mut SDL_Window,
	event_ptr: Box<SDL_Event>,
}

impl MediaInterface for SDL {
	fn init(app_name: &str, engine_name: &str) -> MediaResult<Self> {
		unsafe {
			if  SDL_Init(SDL_INIT_VIDEO)  != 0 {
				let msg = format!("SDL_Init failed: {}", printable_cstr_ptr(SDL_GetError()));
				return log_err!(msg, MediaError)
			}
		}
		let window = unsafe { SDL_CreateWindow(cstr_ptr(app_name), 200, 200, 1200, 600, SDL_WINDOW_FLAGS) };
		let gpu_interface = init_gpu(app_name, engine_name, window)?;
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

	fn is_running(&self) -> bool {
		self.running
	}

	fn process_io(&mut self) {
		while unsafe { SDL_PollEvent(self.event_ptr.as_mut()) } != 0 {
			match unsafe { self.event_ptr.as_ref().type_ } {
				QUIT_EVT => {
					self.running = false;
					return;
				},
				_ => {}
			}
		}
	}

	fn draw(&mut self) {
		let timelaps = self.last_drawn.saturating_duration_since(Instant::now()).as_micros();
		if timelaps > 50 {
			unsafe { SDL_Delay(1); }
		} else {
			unsafe { SDL_Delay((50 - timelaps) as Uint32); }
		}
		self.last_drawn = Instant::now();
	}

	fn close(self) {
		if !self.window.is_null() {
			unsafe { SDL_DestroyWindow(self.window); }
		} else {
			log::info!("Window was not initialized properly: window is a nullptr");
		}
		self.gpu_interface.close();
		unsafe { SDL_Quit(); }
	}
}