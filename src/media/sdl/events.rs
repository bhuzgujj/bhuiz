use crate::bindings::{SDL_PollEvent};
use crate::bindings::{SDL_Event, SDL_EventType_SDL_KEYDOWN, SDL_EventType_SDL_KEYUP, SDL_EventType_SDL_QUIT, SDL_EventType_SDL_WINDOWEVENT, Uint32};
use crate::event::{Event, WindowEvent};


pub(crate) fn capture_events() -> Option<Vec<Event>> {
	let mut events = Vec::new();
	let mut evt = Box::new_uninit();
	let ret = unsafe { SDL_PollEvent(evt.as_mut_ptr()) };
	if ret != 0 && !evt.as_ptr().is_null() {
		let sdl_event = unsafe { evt.assume_init_read() };
		let event = process(sdl_event);
		events.push(event);
		if event == Event::Quit {
			return None;
		}
	}
	Some(events)
}

fn process(event: SDL_Event) -> Event {
	let evt_type = unsafe { event.type_ };
	match evt_type {
		QUIT => Event::Quit,
		WINDOW => {
			let window_evt = unsafe { event.window };
			match window_evt.type_ {
				window::MAXIMIZED => Event::Window(WindowEvent::Maximized),
				window::MINIMIZED => Event::Window(WindowEvent::Minimized),
				window::RESTORE => Event::Window(WindowEvent::Restore),
				_ => Event::Unknown
			}
		},
		KEYDOWN => Event::Unknown,
		KEYUP => Event::Unknown,
		_ => Event::Unknown
	}
}

pub(crate) const QUIT: Uint32 = SDL_EventType_SDL_QUIT as Uint32;
pub(crate) const WINDOW: Uint32 = SDL_EventType_SDL_WINDOWEVENT as Uint32;
pub(crate) const KEYUP: Uint32 = SDL_EventType_SDL_KEYUP as Uint32;
pub(crate) const KEYDOWN: Uint32 = SDL_EventType_SDL_KEYDOWN as Uint32;

pub(crate) mod window {
	use crate::bindings::{SDL_WindowEventID_SDL_WINDOWEVENT_MAXIMIZED, SDL_WindowEventID_SDL_WINDOWEVENT_MINIMIZED, SDL_WindowEventID_SDL_WINDOWEVENT_RESTORED, Uint32};

	pub(crate) const MINIMIZED: Uint32 = SDL_WindowEventID_SDL_WINDOWEVENT_MINIMIZED as Uint32;
	pub(crate) const MAXIMIZED: Uint32 = SDL_WindowEventID_SDL_WINDOWEVENT_MAXIMIZED as Uint32;
	pub(crate) const RESTORE: Uint32 = SDL_WindowEventID_SDL_WINDOWEVENT_RESTORED as Uint32;
}