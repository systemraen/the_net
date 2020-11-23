use crate::traits::scene::*;

pub struct SceneData {
	_title: String,
	_last_scene: u8,
}
struct GameScene {
	sd: SceneData
}
impl SceneActor for GameScene {
	
}