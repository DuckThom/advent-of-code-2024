use std::env;
use std::process::exit;

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod utils;

fn main() {
    let days: Vec<fn()> = vec![
        day_1::execute,
        day_2::execute,
        day_3::execute,
        day_4::execute,
        day_5::execute,
    ];

    let args: Vec<String> = env::args().collect();

    let day: usize = args
        .get(1)
        .map(|s| s.parse::<i32>().unwrap().clamp(0, 31) as usize)
        .unwrap_or(0);

    if day == 0 {
        for i in 1..=days.len() {
            let took = utils::time_it(days[i - 1]);

            utils::print_duration(took, i);
        }
    } else {
        if day > days.len() {
            eprintln!("Day {} is not implemented", day);

            exit(1)
        }

        let took = utils::time_it(days[day - 1]);

        utils::print_duration(took, day);
    }
}
