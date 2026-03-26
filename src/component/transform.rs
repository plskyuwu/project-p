use raylib::math::Vector2;

pub struct Transform {
    pub position: Vector2,
    pub velocity: Vector2,
}

impl Transform {
    pub fn new(position: Vector2, velocity: Vector2) -> Self {
        Transform { position, velocity }
    }
}
