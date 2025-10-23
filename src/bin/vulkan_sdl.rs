use bhomz::logger::bind_logger;
use bhuiz::Bhuiz;
use log::LevelFilter;

fn main() {
	bind_logger(LevelFilter::Debug, None).expect("Failed to bind logger");
	let mut bhuiz = Bhuiz::new("Vulkan/SDL Sample").expect("Failed to create Bhuiz");
	while bhuiz.is_running {
		bhuiz.process_io();
		bhuiz.draw();
	}
	bhuiz.close()
}