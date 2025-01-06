
mod player;
mod systems;
use player::Player;
use systems::input::Input;

use imgui::{Context, FontSource};
use raylib_imgui_rs::Renderer;

use raylib::prelude::*;

fn main() 
{


    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("raylib shooter")
        .build();

    let mut imgui = Context::create();
    imgui.fonts().add_font(&[FontSource::DefaultFontData { config: None }]);
    let mut renderer = Renderer::create(&mut imgui, &mut rl, &thread);

    let th: &RaylibThread= &thread;

    let mut player = Player::new(rl.load_texture(th,"assets/textures/player.png").unwrap(), Some(Vector2::new(200.0, 200.0)), None,Some(30.0));
    while !rl.window_should_close() 
    {
        {

        renderer.update(&mut imgui, &mut rl);
        {
            let ui = imgui.new_frame();
            ui.text("Hello, world!");
        }

        let mut drawer = rl.begin_drawing(&thread);
         
        drawer.draw_rectangle(0, 0, 640, 480, Color::GRAY);
        drawer.clear_background(Color::WHITE);
        drawer.draw_text("raylib shooter", 12, 12, 20, Color::BLACK);
        player.draw(&mut drawer);

        renderer.render(&mut imgui, &mut drawer);
        
        }
        player.input_update(&mut rl);
    }
}

// will need to try to use raylib-wasm see how well the game works when it is done

//TO DO
//1. Find and add a Networking lib to use for the game something with UDP and TCP is a nice to have
//2. first will be single player top down shooter later will add multiplayer but we need to keep in mind that the game will have multiplayer later so wee need to make it is to add it later
