use crate::utils::download_input;
use std::process::exit;
use std::{env, fs};
use tokio;

mod day_1;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;
mod day_18;
mod day_19;
mod day_20;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;
mod utils;

const DAYS: [fn(&str); 20] = [
    day_1::execute,
    day_2::execute,
    day_3::execute,
    day_4::execute,
    day_5::execute,
    day_6::execute,
    day_7::execute,
    day_8::execute,
    day_9::execute,
    day_10::execute,
    day_11::execute,
    day_12::execute,
    day_13::execute,
    day_14::execute,
    day_15::execute,
    day_16::execute,
    day_17::execute,
    day_18::execute,
    day_19::execute,
    day_20::execute,
];

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    let day: usize = args
        .get(1)
        .map(|s| s.parse::<usize>().unwrap().clamp(0, DAYS.len()))
        .unwrap_or(0);

    if day == 0 {
        run_all_days().await
    } else {
        run_day(day).await
    }
}

async fn run_all_days() {
    for day in 1..=DAYS.len() {
        run_day(day).await
    }
}

async fn run_day(day: usize) {
    if day > DAYS.len() {
        eprintln!("Day {} is not implemented", day);

        exit(1)
    }

    if !fs::exists(format!("inputs/day_{}/input", day)).unwrap_or(false) {
        download_input(day).await;
    }

    let input: String = utils::read_input_file(day);

    utils::print_day_banner(day);

    let took = utils::time_it(|| DAYS[day - 1](&input));

    utils::print_duration(took, day);
}
