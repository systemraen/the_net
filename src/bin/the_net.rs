use quicksilver::{geom::Vector, run, Graphics, Input, Result, Settings, Window};

use the_net::modules::{
    game_loop::GameLoop,
    game_data::{DEFAULT_WIDTH, DEFAULT_HEIGHT}
};

fn main() {
    println!("hello world ^_^");

    //*~*~*~todo*~*~*~*~*~*~*~*~*~*~*~
    //☐ pull settings in    

    run(
        Settings {
            title: "the net",
            cursor_icon: None, //#todo add to settings,            
            size: Vector::new(DEFAULT_WIDTH, DEFAULT_HEIGHT),
            //fullscreen: true,
            ..Settings::default()
        },
        app,
    );
}

// the game loop
async fn app(window: Window, gfx: Graphics, input: Input) -> Result<()> {
    let mut game_loop = GameLoop::new(window, gfx, input);

    game_loop.init().run().await
}
