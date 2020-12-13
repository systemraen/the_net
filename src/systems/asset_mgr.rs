use {
	log::{error, warn},
	quicksilver::{graphics::FontRenderer, Graphics},
	std::collections::HashMap,
};

pub struct AssetMgr {
	fonts: HashMap<String, FontRenderer>,
}
impl AssetMgr {
	pub fn new() -> Self {
		Self {
			fonts: HashMap::new(),
		}
	}

	pub fn add_font(&mut self, name: &str, gfx: &Graphics) {
		if self.fonts.contains_key(name) {
			warn!("Font {} already found in cache", name);
		}

		
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
