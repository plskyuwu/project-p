use std::rc::Rc;

use raylib::{
    color::Color,
    math::Vector2,
    prelude::{RaylibDraw, RaylibDrawHandle},
    texture::Texture2D,
};

use crate::entity::drawable::Drawable;

pub struct Tile {
    position: Vector2,
    texture: Rc<Texture2D>,
}

impl Tile {
    pub fn new(position: Vector2, texture: Rc<Texture2D>) -> Self {
        Tile { position, texture }
    }
}

impl Drawable for Tile {
    fn draw(&self, raylib_draw_handle: &mut RaylibDrawHandle) {
        raylib_draw_handle.draw_texture_v(self.texture.as_ref(), self.position, Color::WHITE);
    }
}
