use raylib::math::Vector2;

pub trait Moveable {
    fn apply_velocity(&mut self, dt: f32);
    fn set_velocity(&mut self, velocity: Vector2);
    fn add_velocity(&mut self, velocity: Vector2);
}
