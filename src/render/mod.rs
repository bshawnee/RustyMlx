use mlx::Vec2i;
use crate::mlx;
use crate::mlx::{MlxColor, MlxImage};
use std::num;

pub struct Render {
    
}

impl Render {
    pub fn draw_line(image: &MlxImage, mut p0: Vec2i, mut p1: Vec2i, color: &MlxColor) {
        let delta_x = i32::abs(p1.x - p0.x);
        let delta_y = i32::abs(p1.y - p0.x);
        let sign_x = { if p0.x < p1.x { 1 } else  { -1 } };
        let sign_y = { if p0.y < p1.y { 1 } else  { -1 } };
        let mut error = delta_x - delta_y;

        image.put_pixel(p1.x, p1.y, color);
        while p0.x != p1.x || p0.y != p1.y {
            image.put_pixel(p0.x, p0.x, color);
            let mut error2 = error * 2;
            if error > -delta_y {
                error -= delta_y;
                p0.x += sign_x;
            }
            if error2 < delta_x {
                error += delta_x;
                p0.y += sign_y;
            }
        }
    }
}