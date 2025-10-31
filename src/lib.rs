use crate::media::Media;
use bhomz::spacial::two_dimension::Dimension;
use std::time::Duration;
use bhomz::error::BhomzThrowable;

/// Generated code from header files of selected features
#[allow(warnings)]
#[allow(clippy::all)]
pub(crate) mod bindings;

mod gpu;
mod media;
mod event;
mod enclosed_value;

pub mod constants {
	pub use crate::media::constants::UNDEFINED_WINDOW_POSITION;
}

pub fn start(
	app_name: &str,
	frame_rate: Duration,
	window_dimension: Dimension<i32>
) -> BhomzThrowable {
	let mut media = Media::new(app_name, window_dimension, frame_rate).expect("Failed to create media.");
	let _ = media.start_event_thread();

	media.start_drawing_loop()
}