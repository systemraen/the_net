use crate::structs::game_data::GameData;
use crate::traits::scene::Scene;
use quicksilver::Graphics;

pub struct IntroScene {}

impl IntroScene {
	pub fn new() -> Self {
		IntroScene {}
	}
}

impl Scene for IntroScene {
	fn check_input(&self, gd: &mut GameData) {}
	fn draw_ui(&self, gd: &mut GameData, gfx: &mut Graphics) {}
	fn draw_mouse(&self, gd: &mut GameData, gfx: &mut Graphics) {}
}
