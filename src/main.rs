extern crate amethyst;
mod pong;
mod systems;
use amethyst::prelude::*;
use amethyst::core::transform::TransformBundle;
use amethyst::renderer::{DisplayConfig, DrawFlat, Pipeline, PosTex, RenderBundle, Stage};
use amethyst::input::InputBundle;
fn main() -> amethyst::Result<()> {
	amethyst::start_logger(Default::default());

	let manifest_dir = env!("CARGO_MANIFEST_DIR");
	let display_config_path = format!("{}/resources/display_config.ron", manifest_dir);
	let input_binding_path = format!("{}/resources/bindings_config.ron", manifest_dir);
	let app_path = format!("{}/", manifest_dir);

	let config = DisplayConfig::load(&display_config_path);


	let pipe = Pipeline::build().with_stage(
	 Stage::with_backbuffer()
		  .clear_target([0.00196, 0.23726, 0.21765, 1.0], 1.0)
		  .with_pass(DrawFlat::<PosTex>::new()),
	);
	let input_bundle = InputBundle::<String, String>::new().with_bindings_from_file(input_binding_path)?;


	let game_data = GameDataBuilder::default()
		.with_bundle(TransformBundle::new())?
		.with_bundle(RenderBundle::new(pipe, Some(config)))?
		.with_bundle(input_bundle)?
		.with(systems::PaddleSystem, "paddle_system", &["input_system"]);
	let mut game = Application::new(app_path, pong::Pong, game_data)?;
	game.run();
	Ok(())
}