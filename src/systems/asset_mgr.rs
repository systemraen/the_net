use {
	log::{error, info, warn},
	quicksilver::{
		graphics::{FontRenderer, VectorFont},
		Graphics,
	},
	std::collections::HashMap,
};

//#todo: Wrap assets in Arc's?
pub struct AssetManager {
	fonts: HashMap<String, Option<FontRenderer>>,
}

impl AssetManager {
	pub fn new() -> Self {
		Self {
			fonts: HashMap::new(),
		}
	}

	pub async fn finish_load(&mut self, gfx: &Graphics) {
		// get fonts where None -> can just use the keys
		let to_load_fonts: Vec<String> = self
			.fonts
			.iter()
			.filter(|x| x.1.is_none())
			.map(|x| x.0.clone())
			.collect();

		for font_name in to_load_fonts {
			match self.load_font(&font_name, &gfx).await {
				Ok(font) => {
					self.fonts.insert(font_name.clone(), Some(font));
				}
				Err(_) => {}
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

		self.fonts.insert(name.to_string(), None);
	}

	pub fn get_font(&mut self, name: &str) -> Result<&mut FontRenderer, ()> {
		if !self.fonts.contains_key(name) {
			//#todo Add if dbg
			error!("No font {} found!", name);
			return Err(());
		}

		match self.fonts.get_mut(name) {
			Some(font) => {
				match font {
					Some(font) => Ok(font),
					None => {
						error!("Font {} not initialized before draw", name);
						Err(())
					}
				}
			}
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
