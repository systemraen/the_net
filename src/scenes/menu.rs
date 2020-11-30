use {
	crate::{structs::game_data::GameData, traits::scene::Scene},
	quicksilver::graphics::Graphics,
	net_ui::context
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
