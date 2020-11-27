use crate::structs::game_data::GameData;
use quicksilver::graphics::Graphics;

pub trait Scene {
	fn check_input(&self, gd: &mut GameData);
	fn draw_ui(&self, gd: &mut GameData, gfx: &mut Graphics);
	fn draw_mouse(&self, gd: &mut GameData, gfx: &mut Graphics);
}
