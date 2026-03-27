use std::path::Path;

use raylib::prelude::RaylibDrawHandle;

use crate::entity::{drawable::Drawable, tile::Tile};

pub struct Level {
    tiles: Vec<Vec<Tile>>,
}

impl Level {
    pub fn from_str(str: &str) -> Self {
        Level { tiles: Vec::new() }
    }

    pub fn from_file(file_path: &Path) -> Self {
        Level { tiles: Vec::new() }
    }
}

impl Drawable for Level {
    fn draw(&self, raylib_draw_handle: &mut RaylibDrawHandle) {
        for tile_vector in &self.tiles {
            for tile in tile_vector {
                tile.draw(raylib_draw_handle);
            }
        }
    }
}
