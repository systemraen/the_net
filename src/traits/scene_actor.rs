use quicksilver::graphics::Graphics;
use crate::structs::game_data::GameData;

pub trait SceneActor {
	fn check_input(self, input: u8) -> Self;
	fn display(self) -> Self;
	fn draw_ui(gfx: &mut Graphics);
	fn draw_mouse(gd: &mut GameData, gfx: &mut Graphics);
}