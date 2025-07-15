use raylib::prelude::Vector2;
pub struct CTransform {
    pub position: Vector2,
    pub rotation: f32,
}

impl CTransform {
    pub fn new() -> Self {
        CTransform {
            position: Vector2::new(0.0, 0.0),
            rotation: 0.0,
        }
    }
}

//TO DO
// add helper functions that will be callable from a Ctranform like basic math on calculating
// how the transform of the child should change depending on the parent transform changing
