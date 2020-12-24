use std::collections::HashMap;
use crate::structs::game_data::GameData;
use quicksilver::{
	Graphics, Window,
};

use crate::scenes::prelude::*;

type SceneStore = HashMap<SceneName, Box<dyn Scene>>;

#[derive(Hash, Eq, PartialEq)]
pub enum SceneName {
	Intro,
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
			current_scene: SceneName::Intro,
		}
	}

	pub async fn init(&mut self, gd: &mut GameData, gfx: &Graphics) {		

		self.scenes.insert(SceneName::Intro, Box::new(IntroScene::new()));
		// self.scenes.insert(SceneName::Loading, Box::new(LoadingScene::new()));
		// self.scenes.insert(SceneName::Title, Box::new(TitleScene::new()));
		// self.scenes.insert(SceneName::Game, Box::new(GameScene::new()));
		// self.scenes.insert(SceneName::Pause, Box::new(PauseScene::new()));
		// self.scenes.insert(SceneName::Menu, Box::new(MenuScene::new()));
		// self.scenes.insert(SceneName::DevConsole, Box::new(DevConScene::new()));
	
		let scene = self.scenes.get_mut(&self.current_scene).unwrap();
		scene.init(gd, gfx);
	}

	pub fn trans_to(&mut self, next_scene: SceneName, gd: &mut GameData, gfx: &Graphics) {
		let scene = self.scenes.get_mut(&self.current_scene).unwrap();
		scene.trans_from();
		self.current_scene = next_scene;

		let scene = self.scenes.get_mut(&self.current_scene).unwrap();
		scene.init(gd, gfx);
	}

	pub fn draw_scene(&mut self, gd: &GameData, gfx: &mut Graphics, window: &Window) {
		match self.scenes.get_mut(&self.current_scene) {
			Some(scene) => {
				scene.draw(gd, gfx);
			}
			None => {
				//#todo add error log
			}
		};	
	}
}
