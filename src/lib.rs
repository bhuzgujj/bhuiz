use bhomz::error::BhomzResult;
use crate::media::Media;

/// Generated code from header files of selected features
#[allow(warnings)]
#[allow(clippy::all)]
pub(crate) mod bindings;

mod gpu;
mod media;

pub struct Bhuiz {
	pub is_running: bool,
	media: Media,
}

impl Bhuiz {
	pub fn new(app_name: &str) -> BhomzResult<Self> {
		Ok(Self {
			is_running: true,
			media: Media::new(app_name)?,
		})
	}

	pub fn process_io(&mut self) {
		todo!("process io")
	}

	pub fn draw(&mut self) {
		todo!("draw")
	}

	pub fn close(self) {
		self.media.close()
	}
}