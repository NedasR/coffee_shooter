use ffi::LoadTexture;
use raylib::prelude::*;
//use crate::{player, systems};
use crate::systems::input::Input;
use crate::core::c_transform::CTransform;
use std::f32::consts::PI;
use std::rc::Rc;
use crate::bootstrap::GameContext;

pub static mut DIR: f32 = 0.;

pub struct Player
{
    pub texture: Rc<Texture2D>,
    pub transform: CTransform,
    pub velocity: Vector2,
    pub lastmouse: Vector2,
    pub cursor_state : bool,
}

impl Player
{
    pub fn new(game_context: &mut GameContext, lastmouse: Option<Vector2>,texture_path: &str) -> Self
    {
        let transform = CTransform::new();
        Player
        {
            texture: Rc::clone(&game_context.texture_manager.get_texture(texture_path)),
            transform: transform,
            velocity: Vector2::new(0.0, 0.0),
            lastmouse: lastmouse.unwrap_or(Vector2::new(0.0, 0.0)), 
            cursor_state: true,
        }
    }

    // Replace this with a render manager or a Scene graph so it's easier to to draw objects without needing to manually draw them and change positions and rotations etc
    pub fn draw(&mut self, drawer: &mut RaylibMode2D<'_, RaylibDrawHandle<'_>>)
    {
        let size = Rectangle::new(0.0, 0.0, self.texture.width() as f32, self.texture.height() as f32);
        let size2 = Rectangle::new(self.transform.position.x,self.transform.position.y, self.texture.width() as f32/ 2.0, self.texture.height() as f32 / 2.0);
        drawer.draw_texture_pro(&self.texture.as_ref(),size, size2,Vector2::new(size2.width / 2.00,size2.height / 2.0),self.transform.rotation, Color::WHITE);
    }

    // Replace with a Input system that we can register inputs for with a predicate that way we can avoid this weird thing of 
    // having to need a function that runs the input for any said class
    pub fn movement(&mut self, rl: &mut RaylibHandle, delta_time: &mut f32)
    {

        let dt : f32 = *delta_time;
        let speed:f32 = 200. * dt;

        let mut diraction = Vector2::new(0.0, 0.0);
        if rl.is_key_down(KeyboardKey::KEY_D)
        {
            diraction.x += speed;
        }

        if rl.is_key_down(KeyboardKey::KEY_S)
        {
            diraction.y += speed;
        }

        if rl.is_key_down(KeyboardKey::KEY_A)
        {
            diraction.x -= speed;
        }

        if rl.is_key_down(KeyboardKey::KEY_W)
        {
            diraction.y -= speed;
        }

        self.transform.position = Vector2::new(self.transform.position.x + diraction.x, self.transform.position.y + diraction.y);
    }

    pub fn player_look_at(&mut self, rl: &mut RaylibHandle, mouse : &mut Vector2) // fix problem with look at when going to from postive to negative coordinates both on x and y look at is bugged
    {
        let mut dir = Vector2::new(mouse.x - self.transform.position.x , mouse.y - self.transform.position.y);
        dir = Vector2::new(dir.x / 10.0, dir.y / 10.0);
        self.transform.rotation = (dir.y.atan2(dir.x)) * 180.0 / 3.141592653589793;
    }
}

impl Input for Player
{
    fn input_update(&mut self, rl: &mut RaylibHandle, mouse : &mut Vector2, delta_time: &mut f32)
    {

        // here add if local player for multiplayer

        //if rl.is_key_down(KeyboardKey::KEY_TAB)
        //{
        //    if!self.cursor_state
        //    {
        //        rl.enable_cursor();
        //        self.cursor_state = true;
        //    }
        //} 
        //else 
        //{
        //    if self.cursor_state
        //    {
        //        rl.disable_cursor();
        //        self.cursor_state = false;
        //    }
        //}
        self.player_look_at(rl,mouse);
        self.movement(rl,delta_time);

    }
}