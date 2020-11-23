use quicksilver::{
    geom::Vector,
    input::Event,
    Graphics, Input, Result, Window,
};

use crate::structs::game_data::GameData;
use crate::systems::scene_mgr::SceneManager;

pub struct GameLoop {
    window: Window,
    gfx: Graphics,
    input: Input,
    gd: GameData,
    scene_manager: SceneManager
}

impl GameLoop {
    pub fn new(window: Window, gfx: Graphics, input: Input) -> GameLoop {
        GameLoop {
            window: window,
            gfx: gfx,
            input: input,
            gd: GameData {
                mouse_pos: Vector::new(0., 0.),
                last_mouse_pos: Vector::new(0., 0.),
            },
            scene_manager: SceneManager::new(),
        }
    }

    pub fn init(mut self) -> Self {
        //*~*~*~todo*~*~*~*~*~*~*~*~*~
        // ☐ start network stuff
        // ☐ init asset cacher
        // ☐ load initial assets
        // ☐ init scene manager
        // *~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~

        self.scene_manager.init();

        self
    }

    pub async fn run(&mut self) -> Result<()> {
        loop {
            self.handle_input().await;

            self.scene_manager.draw_scene(&self.gd, &mut self.gfx, &self.window);
        }
    }

    async fn handle_input(&mut self) {
        while let Some(event) = self.input.next_event().await {
            match event {
                Event::KeyboardInput(e) if e.is_down() => {
                    println!("{:?}", e.key());
                }
                _ => {}
            }
        }
        
        self.gd.set_mouse_pos(self.input.mouse().location());
        
        #[cfg(debug_assertions)]
        self.gd.print();
    }
}
