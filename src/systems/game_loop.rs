use quicksilver::{
    geom::Vector,
    input::{Event, Key},
    Graphics, Input, Result, Window,
};

use crate::structs::game_data::GameData;
use crate::systems::scene_mgr::SceneManager;

pub struct GameLoop {
    running: bool,
    window: Window,
    gfx: Graphics,
    input: Input,
    gd: GameData,
    scene_manager: SceneManager,
}

impl GameLoop {
    pub fn new(window: Window, gfx: Graphics, input: Input) -> GameLoop {
        GameLoop {
            running: true,
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
        // ☑ init scene manager
        // *~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~*~

        self.scene_manager.init();

        self
    }

    pub async fn run(&mut self) -> Result<()> {
        while self.running {
            self.handle_input().await;
            self.scene_manager
                .draw_scene(&mut self.gd, &mut self.gfx, &self.window);
        }

        Ok(())
    }

    async fn handle_input(&mut self) {
        while let Some(event) = self.input.next_event().await {
            match event {
                Event::KeyboardInput(k) if k.is_down() => {
                    if k.key() == Key::Escape {
                        self.running = false;
                    }
                    #[cfg(debug_assertions)]
                    println!("{:?}", k.key());

                    //#todo: pull in config for key mappings
                    // for c in config
                    // if e.key is c.Key
                    // c.action(e, &mut self.gd)
                }
                Event::ReceivedCharacter(_c) => {}
                _ => {}
            }
        }
        self.gd.set_mouse_pos(self.input.mouse().location());
        #[cfg(debug_assertions)]
        self.gd.print();
    }
}
