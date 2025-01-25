use imgui::{Context, FontSource};
use raylib_imgui_rs::Renderer;
use raylib::prelude::*;
use crate::player::Player;
use crate::systems::input::Input;

pub fn bootstrap_initialize()
{
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("raylib shooter")
        .build();

    let mut imgui = Context::create();
    imgui.fonts().add_font(&[FontSource::DefaultFontData { config: None }]);
    let mut renderer = Renderer::create(&mut imgui, &mut rl, &thread);

    let mut th: &RaylibThread= &thread;

    load_assets();
    game_update(&mut renderer,&mut imgui,&mut rl, &thread);
}

fn load_assets()
{
    
}

pub fn game_update(renderer: &mut Renderer,imgui : &mut Context, rl : &mut RaylibHandle, thread : &RaylibThread) 
{
    let mut player = crate::player::Player::new(rl.load_texture(thread,"assets/textures/player.png").unwrap(), Some(Vector2::new(0.0, 0.0)), None,Some(30.0),Some(Vector2::new(110.0, 110.0)));
    let mut camera = Camera2D
    {
        offset: Vector2::new(320.0, 240.),
        target: rl.get_mouse_position(),
        rotation: 0.0,
        zoom: 1.0
    };
    let mut last_time: f32 = 0.0;
    let mut delta_time: f32 = 0.0;
    let mut last_postion = Vector2::new(0.0, 0.0);

    let mut mouse = Vector2::new(0.0, 0.0);
    while !rl.window_should_close() 
    {
        let current_time = rl.get_time() as f32;
        delta_time = current_time - last_time;
        {
        last_postion = player.position;
        player.input_update(rl,&mut mouse, &mut delta_time);
        camera.target = player.position;
        renderer.update(imgui, rl);
        {
            let ui = imgui.new_frame();
            unsafe
            {
                ui.text(format!("{}{}{}{}", "Pos: X:", player.position.x, " Y:",player.position.y));
                ui.text(format!("{}{}", "DeltaTime", delta_time));

                ui.text(format!("{}{}{}{}", "Rate of change: X:", player.position.x - last_postion.x, " Y:",player.position.y - last_postion.y));
            }

        }
let mut drawer = rl.begin_drawing(&thread);
drawer.clear_background(Color::WHITE);
{
let mut camera_mode = drawer.begin_mode2D(camera);

    camera_mode.draw_rectangle(0, 0, 640, 480, Color::GRAY);
    camera_mode.draw_line_ex(player.position, mouse, 2.0, Color::RED);
    camera_mode.draw_text("raylib shooter", 12, 12, 20, Color::BLACK);
    player.draw(&mut camera_mode);
    mouse = camera_mode.get_screen_to_world2D(camera_mode.get_mouse_position(), camera)
}
//drop(drawer);
renderer.render(imgui, &mut drawer);
        //let mut drawer = rl.begin_drawing(&thread);
        //let mut camera_mode = drawer.begin_mode2D(camera);
        //drawer.draw_rectangle(0, 0, 640, 480, Color::GRAY);
        //drawer.clear_background(Color::WHITE);
        //drawer.draw_text("raylib shooter", 12, 12, 20, Color::BLACK);
        //player.draw(&mut drawer);

        //renderer.render(imgui, drawer);
        };
        last_time = current_time;
    }
}



// TO DO
// make Collision System for that is optimized with a quadTree for collision checking 
     // need quadTree to to minimize object collisions against each other
     // handle objects switching from one quad node to another quad node

// simple map editor for making the map
     // map editor needs to be able to load and save the map to a file
     // need to make a struct that contain all the objects that will be added to the scene collider box what texture  size rot position 

// implement multiplayer for the game
     // need to make a protocol if we want to use a simple lib that just gives us sockets for TCP and UDP
     // server-client networking for the game much easier to implement then P2P networking for the game
     // server authoritative so players can't cheat server
     // make sure not to included non deterministic
// implement AI for the game (Nice to have NPC that can fight with the player)
