use crate::polygon::Polygon;
use raylib::prelude::*;

pub fn polygon_4() -> Polygon {
    Polygon::new(
        vec![
            Vector2::new(413.0, 177.0),
            Vector2::new(448.0, 159.0),
            Vector2::new(502.0, 88.0),
            Vector2::new(553.0, 53.0),
            Vector2::new(535.0, 36.0),
            Vector2::new(676.0, 37.0),
            Vector2::new(660.0, 52.0),
            Vector2::new(750.0, 145.0),
            Vector2::new(761.0, 179.0),
            Vector2::new(672.0, 192.0),
            Vector2::new(659.0, 214.0),
            Vector2::new(615.0, 214.0),
            Vector2::new(632.0, 230.0),
            Vector2::new(580.0, 230.0),
            Vector2::new(597.0, 215.0),
            Vector2::new(552.0, 214.0),
            Vector2::new(517.0, 144.0),
            Vector2::new(466.0, 180.0),
        ],
        Color::new(255, 140, 0, 255),
        Color::BLACK,
    )
}
