use {
	crate::{
		scenes::prelude::{Scene, SceneData},
		structs::GameData,
	},
	net_ui::{structs::WidgetData, widgets::Button, Context, Layer},
	quicksilver::{Graphics, graphics::VectorFont}
};

pub struct IntroScene {
	data: SceneData
}

impl IntroScene {

	pub fn new() -> Self {
		IntroScene {
			data: SceneData {
				context: Context::new()
			}
		}
	}
}

//#region "Data Structures"
// some struts and enums here
//#endregion

impl Scene for IntroScene {
	fn init(&mut self) {
		//init data
		//self.context
		self.data.context.add_layer(Layer {
			widgets: vec![Box::new(Button {
				data: WidgetData::new(0., 0., 100., 100., true, true),
			})],
		});
	}

	fn handle_data(&mut self, _gd: &GameData) {
		// intro gd handling
		// eastern eggs and stuff
	}

	fn draw_scene(&self, gfx: &mut Graphics) {
		//let context handle drawing
		self.data
			.context
			.draw(gfx, crate::structs::game_data::FG_COLOR);
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
