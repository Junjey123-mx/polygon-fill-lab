use crate::framebuffer::Framebuffer;
use raylib::prelude::*;

pub fn line(framebuffer: &mut Framebuffer, start: Vector2, end: Vector2) {
    let mut x0 = start.x.round() as i32;
    let mut y0 = start.y.round() as i32;

    let x1 = end.x.round() as i32;
    let y1 = end.y.round() as i32;

    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };

    let dy = -(y1 - y0).abs();
    let sy = if y0 < y1 { 1 } else { -1 };

    let mut error = dx + dy;

    loop {
        framebuffer.point(x0, y0);

        if x0 == x1 && y0 == y1 {
            break;
        }

        let error2 = 2 * error;

        if error2 >= dy {
            error += dy;
            x0 += sx;
        }

        if error2 <= dx {
            error += dx;
            y0 += sy;
        }
    }
}
