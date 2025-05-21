pub trait Module {
	type Error;
	fn name(&self) -> &str;
	fn start(&mut self) -> Result<(), Self::Error>;
	fn stop(&mut self) -> Result<(), Self::Error>;
}
