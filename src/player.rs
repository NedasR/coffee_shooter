use raylib::prelude::*;
//use crate::{player, systems};
use crate::systems::input::Input;
use std::f32::consts::PI;

pub static mut DIR: f32 = 0.;

pub struct Player
{
    pub texture: Texture2D,
    pub position: Vector2,
    pub rotation: f32,
    pub velocity: Vector2,
    pub lastmouse: Vector2,
    pub cursor_state : bool,
}

impl Player 
{
    pub fn new(texture: Texture2D, position: Option<Vector2>, velocity: Option<Vector2>, rotation: Option<f32>,lastmouse: Option<Vector2>) -> Self
    {
        Player
        {
            texture,
            position: position.unwrap_or(Vector2::new(200.0, 200.0)), 
            velocity: velocity.unwrap_or(Vector2::new(0.0, 0.0)), 
            lastmouse: lastmouse.unwrap_or(Vector2::new(0.0, 0.0)), 
            rotation: rotation.unwrap_or(0.0),
            cursor_state: true,
        }
    }

    pub fn draw(&mut self, drawer: &mut RaylibMode2D<'_, RaylibDrawHandle<'_>>)
    {
        let size = Rectangle::new(0.0, 0.0, self.texture.width() as f32, self.texture.height() as f32);
        let size2 = Rectangle::new(self.position.x,self.position.y, self.texture.width() as f32/ 2.0, self.texture.height() as f32 / 2.0);
        drawer.draw_texture_pro(&self.texture,size, size2,Vector2::new(size2.width / 2.00,size2.height / 2.0),self.rotation, Color::WHITE);
    }

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

        self.position = Vector2::new(self.position.x + diraction.x, self.position.y + diraction.y);
    }

    pub fn player_look_at(&mut self, rl: &mut RaylibHandle, mouse : &mut Vector2) // fix problem with look at when going to from postive to negative coordinates both on x and y look at is bugged
    {
        let mut dir = Vector2::new(mouse.x - self.position.x , mouse.y - self.position.y);
        dir = Vector2::new(dir.x / 10.0, dir.y / 10.0);
        self.rotation = (dir.y.atan2(dir.x)) * 180.0 / 3.141592653589793;
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