use crate::structs::game_data::{FG_COLOR, GameData};
use quicksilver::{
	geom::{Rectangle, Vector},
	graphics::Graphics,
};

pub trait Scene {
	fn draw_scene(&self, gd: &mut GameData, gfx: &mut Graphics) {
		self.check_input(gd);
		self.draw_ui(gd, gfx);
		self.draw_mouse(gd, gfx);
	}

	fn check_input(&self, gd: &mut GameData);
	fn draw_ui(&self, gd: &mut GameData, gfx: &mut Graphics);
	fn draw_mouse(&self, gd: &mut GameData, gfx: &mut Graphics) {
		// ☐ add to settings
		// draw pointer
		// ☑ xy
		// ☐ dXY (trail)
		// draw particles

		let pointer_size = 5.;
		let rect = Rectangle::new(
			gd.mouse_pos - Vector::new(pointer_size / 2., pointer_size / 2.),
			Vector::new(pointer_size, pointer_size),
		);
		gfx.fill_rect(&rect, FG_COLOR);
	}
}
