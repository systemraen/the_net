//switch scene to trait
// components are traits?

pub struct Scene {
	_title: String,
	_last_scene: u8,
}

pub trait SceneActor {
	fn check_input(self, input: u8) -> Self;
	fn display(self) -> Self;
}

impl Scene {
	pub fn new(title: &str) -> Self {
		Scene {
			_title: title.to_string(),
			_last_scene: 0,
		}
	}

	fn _transition_to(&self) {}

	fn _handle_transition(&self) {}
}


impl SceneActor for Scene {
	fn check_input(self, _input: u8) -> Self {
		self
	}
	fn display(self) -> Self{
		self
	}
}

fn _get_input() -> u8{
	0
}

fn _create_scene() {
	let scene1 = Scene::new("hi I'm a scene 1");
	let _scene2 = Scene::new("hi I'm a scene 2");
	let mut current_scene = scene1;

	loop {		
		let input = _get_input();
		current_scene = current_scene.check_input(input).display();
	}
}
