mod events;

use std::sync::{mpsc, Arc, Mutex};
use std::sync::mpsc::Sender;
use std::thread::JoinHandle;
use std::time::{Duration, Instant};
use bhomz::cstr::ToCCharPtr;
use bhomz::error::{BhomzError, BhomzResult, BhomzThrowable};
use bhomz::log_err;
use bhomz::spacial::two_dimension::Dimension;
use crate::gpu::Gpu;
use crate::bindings::{SDL_Window, SDL_CreateWindow, SDL_WindowFlags_SDL_WINDOW_SHOWN, Uint32, SDL_DestroyWindow, SDL_Init, SDL_INIT_VIDEO, SDL_Quit, SDL_Delay, SDL_Vulkan_LoadLibrary, SDL_Vulkan_GetInstanceExtensions, SDL_CreateRenderer, SDL_RenderClear, SDL_RenderDrawLine, SDL_SetRenderDrawColor, SDL_DestroyRenderer, SDL_RenderPresent};

#[cfg(feature = "vulkan")]
use crate::bindings::{SDL_WindowFlags_SDL_WINDOW_VULKAN};
use crate::enclosed_value::EnclosedValue;
use crate::event::{Event, SettingsEvent, WindowEvent};

#[cfg(feature = "vulkan")]
#[allow(unused)]
const FLAGS: Uint32 = (SDL_WindowFlags_SDL_WINDOW_SHOWN | SDL_WindowFlags_SDL_WINDOW_VULKAN) as Uint32;

#[cfg(not(feature = "vulkan"))]
#[allow(unused)]
const FLAGS: Uint32 = SDL_WindowFlags_SDL_WINDOW_SHOWN as Uint32;

pub struct Media {
	window: *mut SDL_Window,
	gpu: Gpu,

	event_thread: Option<JoinHandle<()>>,
	registered: Vec<Sender<Vec<Event>>>,

	// Flags
	is_hidden: Arc<Mutex<EnclosedValue<bool>>>,
	frame_duration: Arc<Mutex<EnclosedValue<Duration>>>,
	current_frame_duration: Arc<Mutex<EnclosedValue<Duration>>>,
	event_thread_sender: Option<Sender<Vec<Event>>>
}

impl Media {
	pub(crate) fn new(
		app_name: &str,
		window_dimension: Dimension<i32>,
		frame_rate: Duration
	) -> BhomzResult<Self> {
		if unsafe { SDL_Init(SDL_INIT_VIDEO) } != 0 {
			return log_err!(BhomzError, "Could not init SDL_Video");
		}

		#[cfg(feature = "vulkan")]
		if unsafe { SDL_Vulkan_LoadLibrary(std::ptr::null()) } != 0 {
			return log_err!(BhomzError, "Could not init SDL_Vulkan_LoadLibrary");
		}

		let window = unsafe {
			SDL_CreateWindow(
				app_name.to_c_char_ptr(),
				window_dimension.x,
				window_dimension.y,
				window_dimension.width,
				window_dimension.height,
				FLAGS,
			)
		};

		let mut ext_count = 0u32;
		let mut ext;

		#[cfg(feature = "vulkan")]
		unsafe {
			SDL_Vulkan_GetInstanceExtensions(window, &mut ext_count, std::ptr::null_mut());
			ext = Vec::with_capacity(ext_count as usize);
			SDL_Vulkan_GetInstanceExtensions(window, &mut ext_count, ext.as_mut_ptr());
			ext.set_len(ext_count as usize);
		}

		let gpu = Gpu::init(app_name, ext_count, ext, window)?;

		Ok(Self {
			window,
			gpu,
			event_thread: None,
			registered: Vec::new(),
			event_thread_sender: None,
			is_hidden: Arc::new(Mutex::new(EnclosedValue { value: false })),
			frame_duration: Arc::new(Mutex::new(EnclosedValue { value: frame_rate })),
			current_frame_duration: Arc::new(Mutex::new(EnclosedValue { value: frame_rate }))
		})
	}

	pub fn register(&mut self, messager: Sender<Vec<Event>>) {
		self.registered.push(messager)
	}

	pub fn start_event_thread(&mut self) -> Sender<Vec<Event>> {
		let is_hidden_copy = self.is_hidden.clone();
		let frame_duration_copy = self.frame_duration.clone();
		let current_frame_duration_copy = self.current_frame_duration.clone();
		let (sender, receiver) = mpsc::channel();
		self.event_thread = Some(std::thread::spawn(move || 'spawn_loop: loop {
			if let Ok(event) = receiver.recv() {
				for evt in event {
					match evt {
						Event::Quit => {
							break 'spawn_loop;
						}
						Event::Settings(settings) => {
							match settings {
								SettingsEvent::FrameTime(target) => {
									let mut duration = frame_duration_copy.lock().unwrap();
									duration.value = target;

									if !is_hidden_copy.lock().unwrap().value {
										let mut duration = current_frame_duration_copy.lock().unwrap();
										duration.value = target;
									}
								}
							}
						}
						Event::Window(window) => {
							let reference = frame_duration_copy.lock().unwrap();
							let mut duration = current_frame_duration_copy.lock().unwrap();
							let mut hidden = is_hidden_copy.lock().unwrap();
							match window {
								WindowEvent::Minimized => {
									hidden.value = true;
									duration.value = Duration::from_millis(500);
								}
								WindowEvent::Maximized => {}
								WindowEvent::Restore => {
									hidden.value = false;
									duration.value = reference.value.clone();
								}
							}
						}
						_ => {}
					}
				}
			}
		}));
		self.event_thread_sender = Some(sender.clone());
		sender
	}

	pub fn start_drawing_loop(self) -> BhomzThrowable {
		let mut last_draw = Instant::now();
		match self.event_thread {
			None => {}
			Some(event_thread) => {
				let renderer = unsafe { SDL_CreateRenderer(self.window, -1, 0) };
				while !event_thread.is_finished() {
					if renderer.is_null() {
						return log_err!(BhomzError, "Could not initiate Renderer");
					}
					unsafe {
						SDL_SetRenderDrawColor(renderer, 0, 0, 0, 255);
						SDL_RenderClear(renderer);

						SDL_SetRenderDrawColor(renderer, 255, 255, 255, 255);
						SDL_RenderDrawLine(renderer, 0, 0, 100, 200);

						SDL_RenderPresent(renderer);
					}
					let evts = events::capture_events();
					match evts {
						Some(evts) => {
							for reg in &self.registered {
								if let Err(err) = reg.send(evts.clone()) {
									log::error!("Could not send event: {}", err);
								}
							}
						}
						None => break
					}
					let time_since_last_draw = Instant::now().duration_since(last_draw);
					let time_to_wait = time_before_next_frame(
						time_since_last_draw,
						self.current_frame_duration.lock().unwrap().value,
						self.is_hidden.lock().unwrap().value
					);
					unsafe {
						SDL_Delay(time_to_wait);
					}
					last_draw = Instant::now();
				}
				if !event_thread.is_finished() {
					match self.event_thread_sender {
						None => {
							panic!("Could not send event to event_thread");
						}
						Some(evt) => {
							if let Err(err) = evt.send(vec![Event::Quit]) {
								log::error!("Could not send event: {}", err);
							}
							if let Err(_) = event_thread.join() {
								log::error!("Could not join event_thread");
							}
						}
					}
				}
				unsafe { SDL_DestroyRenderer(renderer); }
			}
		}
		unsafe {
			self.gpu.close();
			SDL_DestroyWindow(self.window);
			SDL_Quit()
		}
		Ok(())
	}
}

fn time_before_next_frame(time_since_last_draw: Duration, frame_duration: Duration, is_hidden: bool) -> Uint32 {
	if is_hidden {
		1000
	} else if frame_duration > time_since_last_draw {
		(frame_duration - time_since_last_draw).subsec_millis()
	} else {
		0
	}
}

pub mod constants {
	use crate::bindings::SDL_WINDOWPOS_UNDEFINED_MASK;

	pub const UNDEFINED_WINDOW_POSITION: u32 = SDL_WINDOWPOS_UNDEFINED_MASK;
}