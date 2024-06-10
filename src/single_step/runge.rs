use crate::{point::Point, Settings};

pub fn solve_runge(
    Settings {
        range,
        step,
        start_condition,
        function,
    }: &Settings,
) -> Vec<Point> {
    let n = ((range.end - range.start) / step).round() as usize;
    let mut current = Point {
        x: range.start,
        y: *start_condition,
    };
    let mut points: Vec<Point> = vec![current];

    for _ in 0..n {
        let k1 = step * function(current.x, current.y);
        let k2 = step * function(current.x + step / 2., current.y + k1 / 2.);
        let k3 = step * function(current.x + step / 2., current.y + k2 / 2.);
        let k4 = step * function(current.x + step, current.y + k3);

        let next_y = current.y + 1.0 / 6.0 * (k1 + 2. * k2 + 2. * k3 + k4);
        let next_x = current.x + step;

        let next_point = Point {
            x: next_x,
            y: next_y,
        };

        points.push(next_point);
        current = next_point;
    }

    return points;
}
