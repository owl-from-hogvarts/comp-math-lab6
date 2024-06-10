use crate::{point::Point, single_step::runge::solve_runge, Settings};

pub fn solve_milna(
    Settings {
        range,
        step,
        start_condition,
        function,
    }: &Settings,
) -> Vec<Point> {
    let mut old: [Point; 5] = solve_runge(&Settings {
        // float are inaccurate, so delta is required to ensure that four points
        // are always present
        range: range.start..(range.start + step * 4.),
        step: *step,
        start_condition: *start_condition,
        function: *function,
    })
    .try_into()
    .expect("Array should contain exactly 5 points");

    let mut points = Vec::from_iter(old);

    let mut current_x = range.start;

    let steps_total = ((range.end - range.start) / step) as isize - 2;
    for _ in 0..steps_total {
        let i_3 = function(old[3].x, old[3].y);
        let i_2 = function(old[2].x, old[2].y);
        let i_1 = function(old[1].x, old[1].y);

        let y_expected = old[4].y + 4. / 3. * step * (2. * i_3 - i_2 + 2. * i_1);
        let y_corrected = old[2].y + step / 3. * (i_2 + 4. * i_1 + function(current_x, y_expected));

        old[..].rotate_right(1);

        let current_point = Point {
            x: current_x,
            y: y_corrected,
        };

        points.push(current_point);

        old[0] = current_point;

        current_x += step;
    }

    points
}
