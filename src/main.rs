use std::ffi::CString;

use mlx::{MlxContext, MlxImage, MlxColor};
use crate::mlx::{MlxEvent, Vec2i};
use crate::mlx::MlxEvent::MlxKey;
use crate::mlx::MlxKey::KeyPress;
use crate::render::Render;

mod mlx;
mod render;


fn main() {
    let mlx = MlxContext::new();
    let title_name = CString::new("Test")
        .expect("Cant alloc c_string");
    
    let mut window = mlx.new_window(1000, 1000, title_name);
    let image = mlx.new_image(1000, 1000);
    image.put_pixel(100, 100, &MlxColor::new(255, 255, 255));
    Render::draw_line(&image, Vec2i::new(0,0), Vec2i::new(100, 100), &MlxColor::new(255, 255, 255));
    window.put_image(&image, 0, 0);
    window.on_close(Box::new( || {
        println!("Window closed");
        std::process::exit(0)
    }));
    mlx.start_loop();
}
