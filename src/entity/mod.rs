pub mod drawable;
pub mod moveable;
pub mod player;
pub mod tile;

use raylib::{math::Vector2, prelude::RaylibDrawHandle};

use crate::entity::{drawable::Drawable, moveable::Moveable, player::Player, tile::Tile};

pub enum Entity {
    Player(Player),
    Tile(Tile),
}

impl Drawable for Entity {
    fn draw(&self, raylib_draw_handle: &mut RaylibDrawHandle) {
        match self {
            Entity::Player(p) => {
                p.draw(raylib_draw_handle);
            }
            Entity::Tile(t) => {
                t.draw(raylib_draw_handle);
            }
        }
    }
}

impl Moveable for Entity {
    fn apply_velocity(&mut self, dt: f32) {
        match self {
            Entity::Player(p) => {
                p.apply_velocity(dt);
            }
            _ => {}
        }
    }

    fn set_velocity(&mut self, velocity: Vector2) {
        match self {
            Entity::Player(p) => {
                p.set_velocity(velocity);
            }
            _ => {}
        }
    }

    fn add_velocity(&mut self, velocity: Vector2) {
        match self {
            Entity::Player(p) => {
                p.add_velocity(velocity);
            }
            _ => {}
        }
    }
}
