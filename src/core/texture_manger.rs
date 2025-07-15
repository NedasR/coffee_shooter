use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use raylib::prelude::*;
use raylib::texture::Texture2D;

pub struct TextureManger {
    textures: HashMap<String, Rc<Texture2D>>,
    rl: Rc<RefCell<RaylibHandle>>,
    rt: Rc<RaylibThread>,
    error_texture: Rc<Texture2D>,
}

impl TextureManger {
    pub fn new(rl: Rc<RefCell<RaylibHandle>>, rt: Rc<RaylibThread>) -> Self {
        let error_texture_hard = rl.borrow_mut()
            .load_texture(&rt, "assets/textures/debug/missing_texture.png")
            .unwrap();

        let error_texture = Rc::new(error_texture_hard);

        Self {
            textures: HashMap::new(),
            rl,
            rt,
            error_texture,
        }
    }

    pub fn add_texture(&mut self, texture_path: &str) {
        if self.textures.contains_key(texture_path) {
            return;
        }
        let texture_hard = self.rl.borrow_mut().load_texture(&self.rt,texture_path ).unwrap();
        self.textures.insert(texture_path.to_string(), Rc::new(texture_hard));
    }

    pub fn get_texture(&mut self, texture_path: &str) -> Rc<Texture2D> {
        if self.textures.contains_key(texture_path) {
            return self
                .textures
                .get(texture_path)
                .unwrap_or(&self.error_texture).clone();
        }

        let texture = match self.rl.borrow_mut().load_texture(&self.rt,texture_path )
        {
            Ok(texture) => texture,
            Err(e) => {
                println!("{} {}", e, " GAME_CODE_ERROR");
                return self.error_texture.clone();
            }
        };
        self.textures.insert(texture_path.to_string(), Rc::new(texture));
        return self.textures.get(texture_path).unwrap().clone();
    }
}
