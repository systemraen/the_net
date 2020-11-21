//switch scene to trait
	// components are traits?

pub struct Scene {
	title: String,
	scene_pos: u8,
	last_scene: u8
}

impl Scene {
	pub fn new(title: &str, pos: u8) -> Self {
		Scene {
			title: title.to_string(), 
			scene_pos: pos, 
			last_scene: 0
		}
	}

	pub fn think(&self) {

	}

	pub fn start(&mut self, last_scene: u8) {
		self.last_scene = last_scene;
	}
}