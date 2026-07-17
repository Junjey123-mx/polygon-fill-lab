use crate::polygon::Polygon;
use raylib::prelude::*;

pub fn polygon_2() -> Polygon {
    Polygon::new(
        vec![
            Vector2::new(321.0, 335.0),
            Vector2::new(288.0, 286.0),
            Vector2::new(339.0, 251.0),
            Vector2::new(374.0, 302.0),
        ],
        Color::BLUE,
        Color::BLACK,
    )
}
