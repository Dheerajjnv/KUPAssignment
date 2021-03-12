pub struct _Points {
    pub x: i32,
    pub y: i32,
}
pub fn quadrant_evaluation(points: &_Points) -> String {
    let x_value = points.x;
    let y_value = points.y;

    match (x_value, y_value) {
        (x_value, y_value) if x_value > 0 && y_value > 0 => format!(
            "Position::First(Coordinate::Abscissa {}, Coordinate::Ordinate {} ",
            x_value, y_value
        ),
        (x_value, y_value) if x_value < 0 && y_value > 0 => format!(
            "Position::Second(Coordinate::Abscissa {}, Coordinate::Ordinate {} ",
            x_value, y_value
        ),
        (x_value, y_value) if x_value > 0 && y_value < 0 => format!(
            "Position::Third(Coordinate::Abscissa {}, Coordinate::Ordinate {} ",
            x_value, y_value
        ),
        _ => {
            format!(
                "Position::Fourth(Coordinate::Abscissa {}, Coordinate::Ordinate {} ",
                x_value, y_value
            )
        }
    }
}
