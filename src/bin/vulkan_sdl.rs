use std::time::Duration;
use bhomz::logger::bind_logger;
use bhomz::spacial::two_dimension::Dimension;
use bhuiz::constants::UNDEFINED_WINDOW_POSITION;
use log::LevelFilter;

fn main() {
	bind_logger(LevelFilter::Debug, None).expect("Failed to bind logger");
	let dimension = Dimension {
		x: UNDEFINED_WINDOW_POSITION as i32,
		y: UNDEFINED_WINDOW_POSITION as i32,
		width: 1200,
		height: 600,
	};
	bhuiz::start(
		"Vulkan/SDL Sample",
		Duration::from_millis(1000 / 60),
		dimension
	).unwrap()
}