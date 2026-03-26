use std::rc::Rc;

use raylib::{
    color::Color,
    math::Vector2,
    prelude::{RaylibDraw, RaylibDrawHandle},
    texture::Texture2D,
};

use crate::{
    component::transform::Transform,
    entity::{drawable::Drawable, moveable::Moveable},
};

pub struct Player {
    transform: Transform,
    texture: Rc< Texture2D>,
}

impl Player {
    pub fn new(transform: Transform, texture: Rc<Texture2D>) -> Self {
        Player { transform, texture }
    }
}

impl Drawable for Player {
    fn draw(&self, raylib_draw_handle: &mut RaylibDrawHandle) {
        raylib_draw_handle.draw_texture_v(self.texture.as_ref(), self.transform.position, Color::WHITE);
    }
}

impl Moveable for Player {
    fn apply_velocity(&mut self, dt: f32) {
        self.transform.position += self.transform.velocity * dt;
    }

    fn set_velocity(&mut self, velocity: Vector2) {
        self.transform.velocity = velocity;
    }

    fn add_velocity(&mut self, velocity: Vector2) {
        self.transform.velocity += velocity;
    }
}
