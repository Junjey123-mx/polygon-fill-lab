mod framebuffer;
mod line;
mod polygon;
mod polygon_fill;
mod polygons;

use framebuffer::Framebuffer;
use polygon_fill::fill_polygon;
use polygons::polygon_1::polygon_1;
use polygons::polygon_2::polygon_2;
use polygons::polygon_3::polygon_3;
use polygons::polygon_4::polygon_4;
use polygons::polygon_5_hole::polygon_5_hole;
use raylib::prelude::*;

fn main() {
    let mut framebuffer = Framebuffer::new(800, 600);

    framebuffer.set_background_color(Color::WHITE);
    framebuffer.clear();

    let polygon_1 = polygon_1();
    let polygon_2 = polygon_2();
    let polygon_3 = polygon_3();

    let mut polygon_4 = polygon_4();
    polygon_4.add_hole(polygon_5_hole());

    fill_polygon(&mut framebuffer, &polygon_1);
    polygon_1.draw_border(&mut framebuffer);

    fill_polygon(&mut framebuffer, &polygon_2);
    polygon_2.draw_border(&mut framebuffer);

    fill_polygon(&mut framebuffer, &polygon_3);
    polygon_3.draw_border(&mut framebuffer);

    fill_polygon(&mut framebuffer, &polygon_4);
    polygon_4.draw_border(&mut framebuffer);

    framebuffer.render_to_file("out.png");
    framebuffer.render_to_file("out.bmp");
}
