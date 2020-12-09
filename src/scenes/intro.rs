use {
	crate::{structs::GameData, scenes::prelude::{Scene, SceneData}},
	net_ui::{Context, Layer},
	quicksilver::Graphics,
};

pub struct IntroScene {
	data: SceneData,
}

impl IntroScene {
	pub fn new() -> Self {
		IntroScene {
			data: SceneData {
				context: Context::new(), // can also init the layers/widgets here
			},
		}
	}
}

impl Scene for IntroScene {
	fn init(&mut self) {
		//init data
		//self.context
		self.data.context.add_layer(Layer {
			widgets: vec![]
		});
	}

	fn handle_data(&mut self, gd: &GameData) {
		// intro gd handling
			// eastern eggs and stuff
	}

	fn draw_scene(&self, gfx: &mut Graphics) {
		//let context handle drawing
		//self.data.context.draw(gfx, crate::structs::game_data::FG_COLOR);
	}

	fn trans_from(&mut self) {
		//free data
		self.data.context.clear_layers();
	}
}

/*
There's a few ways I could try to implement this
	> store data
*/
