use raylib::prelude::*;
//use crate::{player, systems};
use crate::systems::input::Input;
use std::f32::consts::PI;

pub struct Player
{
    pub texture: Texture2D,
    pub position: Vector2,
    pub rotation: f32,
    pub velocity: Vector2,
    pub cursor_state : bool,
}

impl Player 
{
    pub fn new(texture: Texture2D, position: Option<Vector2>, velocity: Option<Vector2>, rotation: Option<f32>) -> Self
    {
        Player
        {
            texture,
            position: position.unwrap_or(Vector2::new(200.0, 200.0)), 
            velocity: velocity.unwrap_or(Vector2::new(0.0, 0.0)), 
            rotation: rotation.unwrap_or(0.0),
            cursor_state: true,
        }
    }

    pub fn draw(&mut self, drawer: &mut RaylibDrawHandle)
    {
        let size = Rectangle::new(0.0, 0.0, self.texture.width() as f32, self.texture.height() as f32);
        let size2 = Rectangle::new(self.position.x,self.position.y, self.texture.width() as f32/ 2.0, self.texture.height() as f32 / 2.0);
        drawer.draw_texture_pro(&self.texture,size, size2,Vector2::new(size2.width / 2.00,size2.height / 2.0),self.rotation, Color::WHITE);
    }

    pub fn movement(&mut self, rl: &mut RaylibHandle)
    {
        
        //let radians = self.rotation * (PI / 180.0);
        //let mut dir: Vector2 = Vector2::new(radians.cos(), radians.sin());
        //let magnitude = (dir.x * dir.x + dir.y * dir.y).sqrt();
        //if magnitude == 0.0 {
        //    dir = Vector2::new(0.0, 0.0) // Return zero vector if the input is zero
        //} else {
        //    dir = Vector2::new(dir.x / magnitude, dir.y/ magnitude);
        //}
        //let left_dir = Vector2::new(-dir.x, dir.y);
        //println!("{} {} {}", dir.x, dir.y,"Dir");
        //if rl.is_key_down(KeyboardKey::KEY_W)
        //{
        //    self.position.x += dir.x * 0.1 as f32;
        //    self.position.y += dir.y * 0.1 as f32;
        //}
//
        //if rl.is_key_down(KeyboardKey::KEY_S)
        //{
        //    self.position.x -= dir.x * 0.1 as f32;
        //    self.position.y -= dir.y * 0.1 as f32;
        //}
//
        //if rl.is_key_down(KeyboardKey::KEY_A)
        //{
        //    self.position.x += left_dir.x * 0.1 as f32;
        //    self.position.y += left_dir.y * 0.1 as f32;
        //}

        // simple is best
        if rl.is_key_down(KeyboardKey::KEY_D)
        {
            self.position.x += 1.0 * 0.1 as f32;
        }

        println!("{} {} {}", self.position.x, self.position.y,"\n");

        if rl.is_key_down(KeyboardKey::KEY_S)
        {
            self.position.y += 1.0 * 0.1 as f32;
        }

        if rl.is_key_down(KeyboardKey::KEY_A)
        {
            self.position.x -= 1.0 * 0.1 as f32;
        }

        if rl.is_key_down(KeyboardKey::KEY_W)
        {
            self.position.y -= 1.0 * 0.1 as f32;
        }
    }

    pub fn player_look_at(&mut self, rl: &mut RaylibHandle)
    {
        let mut dir = Vector2::new(rl.get_mouse_position().x - self.position.x , rl.get_mouse_position().y - self.position.y);
        dir = Vector2::new(dir.x / 10.0, dir.y / 10.0);
        self.rotation = (dir.y.atan2(dir.x)) * 180.0 / 3.141592653589793;
    }
}

impl Input for Player
{
    fn input_update(&mut self, rl: &mut RaylibHandle)
    {

        // here add if local player for multiplayer

        if rl.is_key_down(KeyboardKey::KEY_TAB)
        {
            if!self.cursor_state
            {
                rl.enable_cursor();
                self.cursor_state = true;
            }
        } 
        else 
        {
            if self.cursor_state
            {
                rl.disable_cursor();
                self.cursor_state = false;
            }
        }

        self.player_look_at(rl);
        self.movement(rl,);

    }
}