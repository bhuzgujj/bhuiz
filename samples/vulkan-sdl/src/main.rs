use interfaces::graphics::GraphicsInterface;
use sdl::SDL;

fn main() {
	let mut sdl = SDL::init().unwrap();
	while sdl.is_running() {
		sdl.process_io();
		sdl.draw();
	}
	sdl.close()
}
