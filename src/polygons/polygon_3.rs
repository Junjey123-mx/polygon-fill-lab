use crate::polygon::Polygon;
use raylib::prelude::*;

pub fn polygon_3() -> Polygon {
    Polygon::new(
        vec![
            Vector2::new(377.0, 249.0),
            Vector2::new(411.0, 197.0),
            Vector2::new(436.0, 249.0),
        ],
        Color::YELLOW,
        Color::BLACK,
    )
}
