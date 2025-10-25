/// Module trait
pub trait Module {
	type Error: Send + Sync + 'static;
	type Config;
	type Output: Default;
	/// Module name
	fn name(&self) -> &str;
	fn start(&mut self) -> Result<Self::Output, Self::Error>;
	// TODO: is this necessary?
	async fn on_config_change(&mut self, _: Self::Config) -> Result<Self::Output, Self::Error> {
		Ok(Default::default())
	}
	async fn stop(&mut self) -> Result<Self::Output, Self::Error>;
}
