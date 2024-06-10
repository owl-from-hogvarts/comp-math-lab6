use crate::{point::Point, Settings, MAX_ITERATIONS};

pub fn solve_eiler(
    Settings {
        range,
        step,
        start_condition,
        function,
    }: &Settings,
) -> Vec<Point> {
    let mut current = Point {
        x: range.start,
        y: *start_condition,
    };
    let mut points: Vec<Point> = vec![current];

    for _ in 0..MAX_ITERATIONS {
        let approximation = current.y + step * function(current.x, current.y);
        let next_x = current.x + step;

        let next_y = current.y
            + step / 2. * (function(current.x, current.y) + function(next_x, approximation));

        if next_x >= range.end {
            break;
        }

        let next_point = Point {
            x: next_x,
            y: next_y,
        };

        points.push(next_point);

        current = next_point;
    }

    return points;
}
