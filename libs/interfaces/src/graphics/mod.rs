use crate::graphics::errors::GraphicsResult;

pub mod driver;
pub mod errors;

pub trait GraphicsInterface
where
	Self: Sized
{
	/// Initialize the graphical interface with the selected [driver::GpuInterface]
	fn init() -> GraphicsResult<Self>;
	fn is_running(&self) -> bool;
	fn process_io(&mut self);
	fn draw(&mut self);
	fn close(self);
}