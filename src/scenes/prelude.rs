pub use crate::intro::IntroScene;
pub use crate::dev_console::DevConScene;
pub use crate::game::GameScene;
pub use crate::loading::LoadingScene;
pub use crate::menu::MenuScene;
pub use crate::pause::PauseScene;
pub use crate::title::TitleScene;

use crate::structs::game_data::{GameData, DEFAULT_HEIGHT, DEFAULT_WIDTH, FG_COLOR};
use quicksilver::{
	geom::{Rectangle, Vector},
	graphics::Graphics,
};

pub trait Scene {
	fn draw(&self, gd: &mut GameData, gfx: &mut Graphics) {
		//maybe process gd here for the mouse stuff?
			// why though?
		self.process_gd(gd);
		self.draw_border(gfx);
		self.draw_scene(gfx);
		self.draw_mouse(gd, gfx);
	}

	fn process_gd(&self, gd: &mut GameData) {}
	fn draw_scene(&self, gfx: &mut Graphics) {}
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

	fn draw_border(&self, gfx: &mut Graphics) {
		let rect = Rectangle::new(
			Vector::new(1., 1.),
			Vector::new(DEFAULT_WIDTH - 1., DEFAULT_HEIGHT - 2.),
		);
		gfx.stroke_rect(&rect, FG_COLOR);
	}
}

#[derive(Default)]
pub struct SceneData {}