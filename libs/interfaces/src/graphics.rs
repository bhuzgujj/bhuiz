#[derive(Debug)]
pub struct GpuError(pub String);
pub type GpuResult<T> = Result<T, GpuError>;

impl From<String> for GpuError {
	fn from(s: String) -> Self {
		GpuError(s)
	}
}

impl Into<MediaError> for GpuError {
	fn into(self) -> MediaError {
		MediaError(format!("{}", self.0))
	}
}

impl From<MediaError> for GpuError {
	fn from(value: MediaError) -> Self {
		GpuError(format!("{}", value.0))
	}
}

#[derive(Debug)]
pub struct MediaError(pub String);
pub type MediaResult<T> = Result<T, MediaError>;

impl From<String> for MediaError {
	fn from(s: String) -> Self {
		MediaError(s)
	}
}

pub trait MediaInterface
where
	Self: Sized
{
	/// Initialize the graphical interface with the selected [GpuAdapter]
	fn init(app_name: &str, engine_name: &str) -> MediaResult<Self>;

	fn is_running(&self) -> bool;

	fn process_io(&mut self);

	fn draw(&mut self);

	fn close(self);
}

/// Interface for Graphic Processing Unit
pub trait GpuAdapter {
	fn close(self);
}