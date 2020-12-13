use {
	log::{error, warn},
	quicksilver::{
		graphics::{FontRenderer, VectorFont},
		Graphics,
	},
	std::{collections::HashMap, sync::Arc},
};

//#todo: Wrap assets in Arc's?
pub struct AssetMgr {
	fonts: HashMap<String, FontRenderer>,
}
impl AssetMgr {
	pub fn new() -> Self {
		Self {
			fonts: HashMap::new(),
		}
	}

	pub async fn add_font(&mut self, name: &str, gfx: &Graphics) {
		if self.fonts.contains_key(name) {
			warn!("Font {} already found in cache", name);
		}
		let ttf = match VectorFont::load(name).await {
			Ok(ttf) => ttf,
			Err(err) => {
				error!("Error while loading font {}: {}", name, err);
				return;
			}
		};

		let font = match ttf.to_renderer(&gfx, 72.) {
			Ok(font) => font,
			Err(err) => {
				error!("Error rendering font {}: {}", name, err);
				return;
			}
		};

		self.fonts.insert(name.to_string(), font);

		//
	}

	pub fn get_font(&self, name: &str) -> Result<&FontRenderer, ()> {
		if !self.fonts.contains_key(name) {
			//#todo Add if dbg
			error!("No font {} found!", name);
			return Err(());
		}

		Ok(&self.fonts[name])
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
