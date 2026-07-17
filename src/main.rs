mod framebuffer;
mod line;
mod polygon;
mod polygon_fill;
mod polygons;

use framebuffer::Framebuffer;
use polygon_fill::fill_polygon;
use polygons::polygon_1::polygon_1;
use raylib::prelude::*;

fn main() {
    let mut framebuffer = Framebuffer::new(800, 600);

    framebuffer.set_background_color(Color::WHITE);
    framebuffer.clear();

    let polygon = polygon_1();

    fill_polygon(&mut framebuffer, &polygon);
    polygon.draw_border(&mut framebuffer);

    framebuffer.render_to_file("evidence/polygon-1.png");
}
