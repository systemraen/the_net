use crate::structs::game_data::{GameData, DEFAULT_HEIGHT, DEFAULT_WIDTH, FG_COLOR};
use quicksilver::{
	geom::{Rectangle, Vector},
	graphics::Color,
	Graphics, Window,
};

use crate::traits::scene::Scene;
use crate::scenes::*;

enum SceneName {
	Loading,
	Title,
	Game,
	Settings, 
	DevConsole
}

pub struct SceneManager {
	scenes: Vec<Box<dyn Scene>>,
	current_scene: usize,
}

impl SceneManager {
	pub fn new() -> Self {
		SceneManager {
			scenes: vec![
				
			],
			current_scene: 0,
		}
	}

	pub fn init(&mut self) {
		
	}

	pub fn change_to(&mut self, scene_index: u32) {
		//self.active_scene = scene_index;
	}

	pub fn transition_to(&self) {}
	//pub fn handle_transition(&self) {}

	pub fn draw_scene(&self, gd: &GameData, gfx: &mut Graphics, window: &Window) {
		gfx.clear(Color::BLACK);		

		//self.current_scene = self.scenes[self.current_scene].run(gd, gfx);

		match gfx.present(&window) {
			Ok(_) => {}
			Err(e) => println!("err {}", e), // ☒ ~~add logger~~ use quicksilver's logger
			                                 // ☒ maybe crash program at this point?
			                                 //     - why would it give an error?
		}
	}

	fn draw_ui(&self, gfx: &mut Graphics) {
		let rect = Rectangle::new(
			Vector::new(1., 1.),
			Vector::new(DEFAULT_WIDTH - 1., DEFAULT_HEIGHT - 2.),
		);
		gfx.stroke_rect(&rect, FG_COLOR);
	}

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
}
