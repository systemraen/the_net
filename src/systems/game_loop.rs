use quicksilver::{
    geom::Vector,
    graphics::Color,
    input::{Event, Key},
    Graphics, Input, Result, Timer, Window,
};

use crate::structs::game_data::GameData;
use crate::systems::{AssetManager, EventManager, SceneManager};

pub struct GameLoop {
    running: bool,
    window: Window,
    gfx: Graphics,
    input: Input,
    gd: GameData,
    scene_mgr: SceneManager,
    event_mgr: EventManager
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
                timer: Timer::time_per_second(60.),
                asset_mgr: AssetManager::new(),
            },
            scene_mgr: SceneManager::new(),
            event_mgr: EventManager::new(),
        }
    }

    fn init(&mut self) {
        //*~*~*~todo*~*~*~*~*~*~*~*~*~
        // ☐ start network stuff
        // ☐ init asset cacher
        // ☐ load initial assets
        // ☑ init scene manager
        // *~scene_mgr~*~*~*~*~*~*~*~*~*~*~

        self.scene_mgr.init(&mut self.gd, &self.gfx);
    }

    pub async fn run(&mut self) -> Result<()> {
        self.init();

        while self.running {
            self.gd.asset_mgr.finish_load(&self.gfx).await;
            //self.event_mgr.handle_input(&mut self.input);

            self.handle_input().await;
            self.gfx.clear(Color::BLACK);

            self.scene_mgr
                .draw_scene(&mut self.gd, &mut self.gfx, &self.window);

            match self.gfx.present(&self.window) {
                Ok(_) => {}
                Err(e) => println!("err {}", e), // ☒ ~~add logger~~ use quicksilver's logger
                                                 // ☒ maybe crash program at this point?
                                                 //     - why would it give an error?
            }
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
