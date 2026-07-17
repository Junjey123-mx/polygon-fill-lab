use crate::framebuffer::Framebuffer;
use crate::polygon::Polygon;
use raylib::prelude::Vector2;

pub fn fill_polygon(framebuffer: &mut Framebuffer, polygon: &Polygon) {
    if polygon.vertices.len() < 3 {
        return;
    }

    let min_x = polygon
        .vertices
        .iter()
        .map(|vertex| vertex.x)
        .fold(f32::INFINITY, f32::min)
        .floor() as i32;

    let max_x = polygon
        .vertices
        .iter()
        .map(|vertex| vertex.x)
        .fold(f32::NEG_INFINITY, f32::max)
        .ceil() as i32;

    let min_y = polygon
        .vertices
        .iter()
        .map(|vertex| vertex.y)
        .fold(f32::INFINITY, f32::min)
        .floor() as i32;

    let max_y = polygon
        .vertices
        .iter()
        .map(|vertex| vertex.y)
        .fold(f32::NEG_INFINITY, f32::max)
        .ceil() as i32;

    framebuffer.set_current_color(polygon.fill_color);

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let pixel_x = x as f32 + 0.5;
            let pixel_y = y as f32 + 0.5;

            let inside_polygon = point_is_inside(pixel_x, pixel_y, &polygon.vertices);

            let inside_hole = polygon
                .holes
                .iter()
                .any(|hole| point_is_inside(pixel_x, pixel_y, hole));

            if inside_polygon && !inside_hole {
                framebuffer.point(x, y);
            }
        }
    }
}

fn point_is_inside(x: f32, y: f32, vertices: &[Vector2]) -> bool {
    if vertices.len() < 3 {
        return false;
    }

    let mut inside = false;
    let mut previous = vertices.len() - 1;

    for current in 0..vertices.len() {
        let current_vertex = vertices[current];
        let previous_vertex = vertices[previous];

        let crosses_vertical_range = (current_vertex.y > y) != (previous_vertex.y > y);

        if crosses_vertical_range {
            let intersection_x = (previous_vertex.x - current_vertex.x) * (y - current_vertex.y)
                / (previous_vertex.y - current_vertex.y)
                + current_vertex.x;

            if x < intersection_x {
                inside = !inside;
            }
        }

        previous = current;
    }

    inside
}
