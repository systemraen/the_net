use crate::{
	structs::game_data::{GameData, DEFAULT_HEIGHT, DEFAULT_WIDTH, FG_COLOR},
	traits::scene::Scene,
};
use quicksilver::{
	geom::{Rectangle, Vector},
	graphics::Graphics,
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
		self.print_outline(gfx);

		
	}
}
