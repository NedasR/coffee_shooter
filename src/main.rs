
mod bootstrap;
mod player;
mod systems;
mod util;
mod core;

use raylib::prelude::*;


fn main() 
{
    
    let mut game = bootstrap::GameContext::new();
    game.run();

}

// will need to try to use raylib-wasm see how well the game works when it is done

//TO DO
//1. Find and add a Networking lib to use for the game something with UDP and TCP is a nice to have
//2. first will be single player top down shooter later will add multiplayer but we need to keep in mind that the game will have multiplayer later so wee need to make it is to add it later
