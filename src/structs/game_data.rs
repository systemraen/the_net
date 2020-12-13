use quicksilver::{geom::Vector, graphics::Color, Timer};
use crate::systems::asset_mgr::AssetMgr;

//https://en.wikipedia.org/wiki/16:9_aspect_ratio#Common_resolutions
pub const DEFAULT_WIDTH: f32 = 1366.; //WXGA
pub const DEFAULT_HEIGHT: f32 = 768.;
pub const _OMEGA_WIDTH: f32 = 15360.; //16k UHD
pub const _OMEGA_HEIGHT: f32 = 8640.;
pub const TITLE_FONT: &str = "NASDAQER.ttf";
pub const FG_COLOR: Color = Color::PURPLE;

pub struct GameData {
    pub mouse_pos: Vector,
    pub last_mouse_pos: Vector,
    pub timer: Timer,
    pub asset_mgr: AssetMgr
}

impl GameData {
    pub fn init(&mut self) {
        //self.asset_mgr.init();
    }

    //also records previous position
    pub fn set_mouse_pos(&mut self, pos: Vector) {
        self.last_mouse_pos = self.mouse_pos;
        self.mouse_pos = pos;
    }

    //#todo: move this to debug trait
    pub fn print(&self) {
        if self.last_mouse_pos != self.mouse_pos {
            //println!("m: {} | lm: {}", self.mouse_pos, self.last_mouse_pos);
        }
    }
}
