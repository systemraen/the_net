use crate::structs::game_data::{GameData, DEFAULT_HEIGHT, DEFAULT_WIDTH, FG_COLOR};
use quicksilver::{
	geom::{Rectangle, Vector},
	graphics::Graphics,
};

pub trait Scene {
	fn draw(&mut self, gd: &GameData, gfx: &mut Graphics) {
		//maybe process gd here for the mouse stuff?
		// why though?
		self.handle_data(gd);
		self.draw_border(gfx);
		self.draw_scene(gd, gfx);
		self.draw_mouse(gd, gfx);
	}

	//#todo: remove the default impls
	fn init(&mut self, _gd: &mut GameData, gfx: &Graphics) {}
	fn trans_from(&mut self) {}
	fn handle_data(&mut self, _gd: &GameData) {}
	fn draw_scene(&self, gd: &GameData, _gfx: &mut Graphics) {}
	fn draw_mouse(&self, gd: &GameData, gfx: &mut Graphics) {
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

	fn draw_border(&self, gfx: &mut Graphics) {
		let rect = Rectangle::new(
			Vector::new(1., 1.),
			Vector::new(DEFAULT_WIDTH - 1., DEFAULT_HEIGHT - 2.),
		);
		gfx.stroke_rect(&rect, FG_COLOR);
	}
}
