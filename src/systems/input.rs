use raylib::prelude::*;
pub trait Input
{
    fn input_update(&mut self, rl: &mut RaylibHandle, mouse : &mut Vector2, delta_time: &mut f32)
    {
        println!("Not implemented");
    }
}