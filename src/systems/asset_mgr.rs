use {
	log::{error, info, warn},
	quicksilver::{
		graphics::{FontRenderer, VectorFont},
		Graphics,
	},
	std::collections::HashMap,
};

struct FontData {
	pub font: Option<FontRenderer>,
	pub loaded: bool
}

impl FontData {
	pub fn new() -> Self {
		FontData {
			font: None, 
			loaded: false
		}
	}
}

//#todo: Wrap assets in Arc's?
pub struct AssetMgr<T> where T: Fn() {
	fonts: HashMap<String, FontRenderer>,
	to_load: Vec<T>
}

impl AssetMgr {
	pub fn new() -> Self {
		Self {
			fonts: HashMap::new(),
		}
	}

	pub async fn finish_load(&mut self, gfx: &Graphics) {
		for (name, font_data) in &self.fonts {
			match self.load_font(&name, &gfx).await {
				Ok(font) => { font_data.font = Some(font); }
				_ => {}
			}
		}
	}

	pub async fn load_font(&self, name: &String, gfx: &Graphics) -> Result<FontRenderer, ()> {
		let ttf = match VectorFont::load(name).await {
			Ok(ttf) => {
				info!("Loaded {}", name);
				ttf
			}
			Err(err) => {
				error!("Error while loading font {}: {}", name, err);
				return Err(());
			}
		};

		let font = match ttf.to_renderer(&gfx, 72.) {
			Ok(font) => return Ok(font),
			Err(err) => {
				error!("Error rendering font {}: {}", name, err);
				return Err(());
			}
		};
	}

	pub fn add_font(&mut self, name: &str) {
		if self.fonts.contains_key(name) {
			warn!("Font {} already found in cache", name);
		}

		self.fonts.insert(name.to_string(), FontData::new());
		

		//font.draw(&mut self.gfx, "THE NET", FG_COLOR, Vector::new(500., title_pos))?;
	}

	pub fn get_font(&self, name: &str) -> Result<&FontRenderer, ()> {
		if !self.fonts.contains_key(name) {
			//#todo Add if dbg
			error!("No font {} found!", name);
			return Err(());
		}

		match &self.fonts[name] {
			Some(font) => Ok(font.font),
			None => {
				error!("Font {} not loaded", name);
				Err(())
			}
		}
	}

	pub fn clear_font(&mut self, name: &str) -> Result<(), ()> {
		if !self.fonts.contains_key(name) {
			error!("No font {} found!", name);
			return Err(());
		}

		self.fonts.remove(name);
		Ok(())
	}

	pub fn clear_all_fonts(&mut self) {
		self.fonts.clear();
	}
	pub fn clear_all(&mut self) {
		self.clear_all_fonts();
	}
}
