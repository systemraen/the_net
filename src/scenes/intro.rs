use {
	crate::{
		scenes::prelude::{Scene, SceneData},
		structs::{game_data::{TITLE_FONT, FG_COLOR}, GameData},
	},
	log::error,
	net_ui::{structs::WidgetData, widgets::Button, Context, Layer},
	quicksilver::{geom::Vector, Graphics},
};

pub struct IntroScene {
	data: SceneData,
	title_pos: f32,
}

impl IntroScene {
	pub fn new() -> Self {
		IntroScene {
			data: SceneData {
				context: Context::new(),
			},
			title_pos: -500.,
		}
	}
}

//#region "Data Structures"
// some struts and enums here
//#endregion

impl Scene for IntroScene {
	fn init(&mut self, gd: &mut GameData, gfx: &Graphics) {
		//init data
		//self.context
		self.data.context.add_layer(Layer {
			widgets: vec![Box::new(Button {
				data: WidgetData::new(0., 0., 100., 100., true, true),
			})],
		});

		gd.asset_mgr.add_font(TITLE_FONT, gfx);
	}

	fn handle_data(&mut self, _gd: &mut GameData) {
		// intro gd handling
		// eastern eggs and stuff
		if _gd.timer.tick() {
			self.title_pos += 5.;
		}
	}

	fn draw_scene(&self, gd: &mut GameData, gfx: &mut Graphics) {
		//let context handle drawing
		self.data
			.context
			.draw(gfx, crate::structs::game_data::FG_COLOR);

		let font = match gd.asset_mgr.get_font(TITLE_FONT) {
			Ok(font) => font,
			Err(_) => {
				error!("Hey I'm trying to get font {} and cant T_T", TITLE_FONT);
				return;
			}
		};

		match font.draw(gfx, "THE NET", FG_COLOR, Vector::new(500., self.title_pos)) {
			Ok(_) => {},
			Err(_) => {}
		};
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
