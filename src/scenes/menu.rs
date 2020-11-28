use {
	crate::{
		structs::game_data::{GameData, DEFAULT_HEIGHT, DEFAULT_WIDTH, FG_COLOR},
		traits::scene::Scene,
	},
	mergui::{widgets::ButtonConfig, Context, FontStyle, MFont},
	quicksilver::{
		geom::{Rectangle, Vector},
		graphics::Graphics,
	},
};

pub struct MenuScene {}

impl MenuScene {
	pub fn new() -> Self {
		MenuScene {}
	}
}

impl Scene for MenuScene {
	fn check_input(&self, gd: &mut GameData) {}
	fn draw_ui(&self, gd: &mut GameData, gfx: &mut Graphics) {
		let rect = Rectangle::new(
			Vector::new(1., 1.),
			Vector::new(DEFAULT_WIDTH - 1., DEFAULT_HEIGHT - 2.),
		);
		gfx.stroke_rect(&rect, FG_COLOR);
	}
}
