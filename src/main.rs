use std::{env, fs};
use std::process::exit;
use crate::utils::download_input;
use tokio;

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_9;
mod utils;

const DAYS: [fn(); 9] = [
    day_1::execute,
    day_2::execute,
    day_3::execute,
    day_4::execute,
    day_5::execute,
    day_6::execute,
    day_7::execute,
    || {},
    day_9::execute,
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

    let took = utils::time_it(DAYS[day - 1]);

    utils::print_duration(took, day);
}