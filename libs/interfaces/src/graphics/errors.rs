#[derive(Debug)]
pub struct GpuError(pub String);
pub type GpuResult<T> = Result<T, GpuError>;

#[derive(Debug)]
pub struct GraphicsError(pub String);
pub type GraphicsResult<T> = Result<T, GraphicsError>;

impl Into<GraphicsError> for GpuError {
	fn into(self) -> GraphicsError {
		GraphicsError(format!("{}", self.0))
	}
}
impl From<GraphicsError> for GpuError {
	fn from(value: GraphicsError) -> Self {
		GpuError(format!("{}", value.0))
	}
}