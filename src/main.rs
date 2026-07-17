mod framebuffer;
mod line;
mod polygon;
mod polygon_fill;
mod polygons;

use framebuffer::Framebuffer;
use polygon_fill::fill_polygon;
use polygons::polygon_4::polygon_4;
use polygons::polygon_5_hole::polygon_5_hole;
use raylib::prelude::*;

fn main() {
    let mut framebuffer = Framebuffer::new(800, 600);

    framebuffer.set_background_color(Color::WHITE);
    framebuffer.clear();

    let mut polygon = polygon_4();
    polygon.add_hole(polygon_5_hole());

    fill_polygon(&mut framebuffer, &polygon);
    polygon.draw_border(&mut framebuffer);

    framebuffer.render_to_file("evidence/polygon-5-hole.png");
}
