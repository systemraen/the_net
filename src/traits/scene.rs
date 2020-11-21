//switch scene to trait
	// components are traits?

pub trait Scene {
	fn think(&self);
	fn draw(&self);
}

pub struct SceneData {
	title: String,
	scene_pos: u8,
	last_scene: u8
}