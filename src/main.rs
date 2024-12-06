use std::env;
use std::process::exit;

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod utils;

const DAYS: [fn(); 6] = [
    day_1::execute,
    day_2::execute,
    day_3::execute,
    day_4::execute,
    day_5::execute,
    day_6::execute,
];

fn main() {
    let args: Vec<String> = env::args().collect();

    let day: usize = args
        .get(1)
        .map(|s| s.parse::<usize>().unwrap().clamp(0, DAYS.len()))
        .unwrap_or(0);

    if day == 0 {
        run_all_days()
    } else {
        run_day(day)
    }
}

fn run_all_days() {
    for day in 1..=DAYS.len() {
        run_day(day)
    }
}

fn run_day(day: usize) {
    if day > DAYS.len() {
        eprintln!("Day {} is not implemented", day);

        exit(1)
    }

    let took = utils::time_it(DAYS[day - 1]);

    utils::print_duration(took, day);
}