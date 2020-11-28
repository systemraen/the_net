use std::collections::HashMap;
use crate::structs::game_data::GameData;
use quicksilver::{
	graphics::Color,
	Graphics, Window,
};

use crate::traits::scene::Scene;
use crate::scenes::*;

type SceneStore = HashMap<SceneName, Box<dyn Scene>>;

#[derive(Hash, Eq, PartialEq)]
pub enum SceneName {
	Loading,
	Title,
	Game,
	Pause,
	Menu,
	DevConsole
}

pub struct SceneManager {
	scenes: SceneStore,
	current_scene: SceneName,
}

impl SceneManager {
	pub fn new() -> Self {
		SceneManager {
			scenes: HashMap::new(),
			current_scene: SceneName::Menu,
		}
	}

	pub fn init(&mut self) {
		self.scenes.insert(SceneName::Loading, Box::new(LoadingScene::new()));
		self.scenes.insert(SceneName::Title, Box::new(TitleScene::new()));
		self.scenes.insert(SceneName::Game, Box::new(GameScene::new()));
		self.scenes.insert(SceneName::Pause, Box::new(PauseScene::new()));
		self.scenes.insert(SceneName::Menu, Box::new(MenuScene::new()));
		self.scenes.insert(SceneName::DevConsole, Box::new(DevConScene::new()));
	}

	pub fn transition_to(&self) {}

	pub fn draw_scene(&self, gd: &mut GameData, gfx: &mut Graphics, window: &Window) {
		gfx.clear(Color::BLACK);
		
		self.scenes[&self.current_scene].draw_scene(gd, gfx);

		match gfx.present(&window) {
			Ok(_) => {}
			Err(e) => println!("err {}", e)
		}
	}
}
