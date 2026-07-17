use crate::framebuffer::Framebuffer;
use crate::line::line;
use raylib::prelude::*;

pub struct Polygon {
    pub vertices: Vec<Vector2>,
    pub fill_color: Color,
    pub border_color: Color,
    pub holes: Vec<Vec<Vector2>>,
}

impl Polygon {
    pub fn new(vertices: Vec<Vector2>, fill_color: Color, border_color: Color) -> Self {
        Self {
            vertices,
            fill_color,
            border_color,
            holes: Vec::new(),
        }
    }

    pub fn add_hole(&mut self, hole: Vec<Vector2>) {
        if hole.len() >= 3 {
            self.holes.push(hole);
        }
    }

    pub fn draw_border(&self, framebuffer: &mut Framebuffer) {
        framebuffer.set_current_color(self.border_color);
        draw_closed_path(framebuffer, &self.vertices);

        for hole in &self.holes {
            draw_closed_path(framebuffer, hole);
        }
    }
}

fn draw_closed_path(framebuffer: &mut Framebuffer, vertices: &[Vector2]) {
    if vertices.len() < 2 {
        return;
    }

    for index in 0..vertices.len() {
        let start = vertices[index];
        let end = vertices[(index + 1) % vertices.len()];
        line(framebuffer, start, end);
    }
}
