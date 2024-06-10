use crate::point::Point;
use crate::{
    multi_step::solve_milna,
    single_step::{eiler::solve_eiler, runge::solve_runge},
};
use crate::{Method, Settings, TNumber};

pub fn runge_rule(
    mut settings: Settings,
    method: Method,
    epsilon: TNumber,
) -> Result<Vec<Point>, &'static str> {
    const MAX_TRIES: usize = 100;
    let (precision, method): (i32, fn(&Settings) -> Vec<Point>) = {
        match method {
            Method::Runge => (4, solve_runge),
            Method::Eiler => (2, solve_eiler),
            Method::Milna => (4, solve_milna),
        }
    };

    let mut results = method(&settings);

    for try_number in 0..MAX_TRIES {
        println!("Trial number: {try_number}");
        if results.iter().any(|point| point.y.is_infinite()) {
            return Err("Function has discontinuity on range specified. Try other range or other start condition".into());
        }

        settings.step /= 2.;
        let halved = method(&settings);

        let precision_coeficient =
            (results.last().unwrap().y - halved.last().unwrap().y) / 2_f64.powi(precision);

        if precision_coeficient <= epsilon {
            return Ok(results);
        }

        results = halved;
        println!("Failed to achived requested precision: {precision_coeficient} > {epsilon}.");
        println!("Trying with lower step: {}", settings.step);
        println!("{}", "=".repeat(80));
    }

    return Err("Max tries excided! Too high prescision required. Could not achive it");
}
