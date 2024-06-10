// requirements
// at least 3 equations
// start condition
// interval
// step
// epsilon (precision)
// improved Eiler's method
// Runge-Kutt's method of fourth order
// Miln's method

use std::{fmt::Display, ops::Range};

use cli_table::format::{Border, HorizontalLine, Separator, VerticalLine};
use correction::runge_rule;
use inquire::{
    list_option::ListOption,
    validator::{ErrorMessage, Validation},
};
use point::Point;

mod correction;
mod multi_step;
mod point;
mod single_step;

type TEquation = fn(f64, f64) -> f64;

type TNumber = f64;
const MAX_ITERATIONS: usize = 1000;

enum Method {
    Runge,
    Eiler,
    Milna,
}

impl Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Method::Runge => write!(f, "Runge's"),
            Method::Eiler => write!(f, "Eiler's modified"),
            Method::Milna => write!(f, "Milna's"),
        }
    }
}

#[derive(Clone)]
struct Settings {
    range: Range<TNumber>,
    step: TNumber,
    start_condition: TNumber,
    function: TEquation,
}

const EQUATIONS: [TEquation; 3] = [
    |x, y| y + (1. + x) * y * y,
    |_, y| 0.6 - y / 5.,
    |x, y| 2. * x * y / (x * x + y * y),
];

fn main() {
    if let Err(error) = start() {
        println!("{error}");
    }
}

fn start() -> Result<(), Box<dyn std::error::Error>> {
    let ListOption {
        index: equation_index,
        ..
    } = inquire::Select::new(
        "Select deferential equiation",
        vec![
            "y' = y + (1 + x) * y^2",
            "y' = 0.6 - y / 5",
            "y' = 2xy / (x^2 - y^2)",
        ],
    )
    .raw_prompt()?;

    let method = inquire::Select::new(
        "Select method",
        vec![Method::Runge, Method::Eiler, Method::Milna],
    )
    .prompt()?;

    let range = {
        let range_start = inquire::prompt_f64("Range start")?;
        let range_end = inquire::prompt_f64("Range end")?;

        range_start..range_end
    };

    let start_condition = inquire::prompt_f64("Start condition")?;

    let step = inquire::prompt_f64("Step")?;

    let settings = Settings {
        range,
        step,
        start_condition,
        function: EQUATIONS[equation_index],
    };

    let epsilon = inquire::CustomType::<f64>::new("Epsilon (should be >= 0)")
        .with_validator(|&v: &f64| {
            if v < 0. {
                return Ok(Validation::Invalid(ErrorMessage::Custom(
                    "Epsilon should be positive!".to_string(),
                )));
            }

            return Ok(Validation::Valid);
        })
        .prompt()?;

    let points = runge_rule(settings, method, epsilon)?;

    print_points(&points)?;
    plot(&points)?;

    Ok(())
}

fn print_points(points: &Vec<Point>) -> Result<(), Box<dyn std::error::Error>> {
    use cli_table::Table;
    let table = points
        .iter()
        .enumerate()
        .map(|(index, point)| {
            vec![
                (index + 1).to_string(),
                format!("{:.4}", point.x),
                format!("{:.4}", point.y),
            ]
        })
        .table()
        .border(
            Border::builder()
                .top(HorizontalLine::new('╭', '╮', '┬', '─'))
                .left(VerticalLine::new('│'))
                .right(VerticalLine::new('│'))
                .bottom(HorizontalLine::new('╰', '╯', '┴', '─'))
                .build(),
        )
        .separator(
            Separator::builder()
                .row(Some(HorizontalLine::new('├', '┤', '┼', '─')))
                .build(),
        )
        .title(["Point number", "X", "Y"])
        .display()?;

    println!("{table}");
    return Ok(());
}

fn plot(points: &Vec<Point>) -> Result<(), Box<dyn std::error::Error>> {
    use plotters::prelude::*;
    const MARGINS: i32 = 10;
    const IMAGE_PATH: &'static str = "./plot.png";

    println!("Generating image. This may take several seconds");

    let x_range = {
        let min = points
            .iter()
            .min_by(|a, b| a.x.total_cmp(&b.x))
            .expect("At least one point present");
        let max = points
            .iter()
            .max_by(|a, b| a.x.total_cmp(&b.x))
            .expect("At least one point present");

        min.x..max.x
    };

    let y_range = {
        let min = points
            .iter()
            .min_by(|a, b| a.y.total_cmp(&b.y))
            .expect("At least one point present");
        let max = points
            .iter()
            .max_by(|a, b| a.y.total_cmp(&b.y))
            .expect("At least one point present");

        min.y..max.y
    };

    let root = BitMapBackend::new(IMAGE_PATH, (1920, 1080)).into_drawing_area();
    root.fill(&WHITE)?;
    let root = root.margin(MARGINS, MARGINS, MARGINS, MARGINS);

    let mut chart = ChartBuilder::on(&root)
        .margin(MARGINS * 2)
        .x_label_area_size(20)
        .y_label_area_size(40)
        .build_cartesian_2d(x_range, y_range)?;

    chart
        .configure_mesh()
        .label_style(("noto sans", 16))
        .x_labels(5)
        .y_labels(5)
        .x_desc("X")
        .y_desc("Y")
        .draw()?;
    chart.draw_series(LineSeries::new(
        points.iter().map(|point| (point.x, point.y)),
        BLACK.stroke_width(3),
    ))?;

    root.present()?;

    println!("Image saved at path: {}", IMAGE_PATH);
    Ok(())
}
