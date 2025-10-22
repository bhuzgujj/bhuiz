use log::LevelFilter;
use interfaces::graphics::MediaInterface;
use interfaces::logger::bind_logger;
use sdl::SDL;

fn main() {
	bind_logger(LevelFilter::Debug, None).expect("Failed to bind logger");
	let mut sdl = SDL::init("Sample", "Vulkan/SDL Sample Engine").unwrap();
	while sdl.is_running() {
		sdl.process_io();
		sdl.draw();
	}
	sdl.close()
}
