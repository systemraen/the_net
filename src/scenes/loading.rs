use crate::structs::game_data::GameData;
use crate::traits::scene::Scene;
use quicksilver::Graphics;

pub struct LoadingScene {}

impl LoadingScene {
	pub fn new() -> Self {
		LoadingScene {}
	}
}

impl Scene for LoadingScene {
	fn check_input(&self, gd: &mut GameData) {}
	fn draw_ui(&self, gd: &mut GameData, gfx: &mut Graphics) {
		
	}
	fn draw_mouse(&self, gd: &mut GameData, gfx: &mut Graphics) {}
}
