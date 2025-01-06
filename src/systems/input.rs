use raylib::prelude::*;
pub trait Input
{
    fn input_update(&mut self, rl: &mut RaylibHandle)
    {
        println!("Not implemented");
    }
}