use quicksilver::input::Input;

pub struct EventManager {}

impl EventManager {
	pub fn new() -> Self {
		Self {}
	}
	pub fn register(&mut self) {}
	pub fn handle_input(&mut self, input: &mut Input) {}
}
