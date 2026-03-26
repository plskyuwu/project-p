use raylib::prelude::RaylibDrawHandle;

pub trait Drawable {
    fn draw(&self, raylib_draw_handle: &mut RaylibDrawHandle);
}
