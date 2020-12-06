use {
	crate::scenes::prelude::{Scene, SceneData},
	net_ui::{Context, Layer},
	quicksilver::Graphics,
};

pub struct IntroScene {
	data: SceneData,
}

impl IntroScene {
	pub fn new() -> Self {
		IntroScene { data: SceneData { context: Context::new() } }
	}
}

impl Scene for IntroScene {
	fn init(&self) {
		//data.add_rect(0, 0, 100, 100);
	}
	fn draw_scene(&self, gfx: &mut Graphics) {
		// okay... maybe not cacheing the widgets - load them up every time
			// why though?		
	}
}

/*
There's a few ways I could try to implement this
	> store data
*/
