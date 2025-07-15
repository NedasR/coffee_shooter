use crate::core::drawable::Drawable;
use std::vec;

pub struct RenderManager {
    render_graph: vec::Vec<Drawable>,
}

impl RenderManager {
    pub fn new() -> Self {
        RenderManager {
            render_graph: vec::Vec::new(),
        }
    }

    pub fn render_pass(&mut self) {
        // will need to go through the render graph and draw everything recursively
    }
}
