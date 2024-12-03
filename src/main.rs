use std::env;
use std::process::exit;

mod day_1;
mod day_2;
mod day_3;
mod utils;

#[derive(Debug, PartialEq)]
enum MethodType {
    EXECUTE,
    VALIDATE,
}

fn main() {
    let execute_days: Vec<fn()> = vec![day_1::execute, day_2::execute, day_3::execute];
    let validate_days: Vec<fn()> = vec![day_1::validate, day_2::validate, day_3::validate];

    let args: Vec<String> = env::args().collect();

    let method: MethodType = args
        .get(1)
        .map(|s| match s.as_str() {
            "execute" => MethodType::EXECUTE,
            "validate" => MethodType::VALIDATE,
            _ => {
                eprintln!(
                    "Unknown method: {}. Valid arguments: 'execute' (default), 'validate'",
                    args[1].as_str()
                );

                exit(1);
            }
        })
        .unwrap_or(MethodType::EXECUTE);

    let day: usize = args
        .get(2)
        .map(|s| s.parse::<i32>().unwrap().clamp(0, 31) as usize)
        .unwrap_or(0);

    let list_to_use: &Vec<fn()> = match method {
        MethodType::EXECUTE => &execute_days,
        MethodType::VALIDATE => &validate_days,
    };

    if day == 0 {
        for i in 1..=list_to_use.len() {
            let took = utils::time_it(list_to_use[i - 1]);

            if method == MethodType::EXECUTE {
                utils::print_duration(took, i);
            }
        }
    } else {
        if day > list_to_use.len() {
            eprintln!("Day {} is not implemented", day);

            exit(1)
        }

        let took = utils::time_it(list_to_use[day - 1]);

        if method == MethodType::EXECUTE {
            utils::print_duration(took, day);
        }
    }
}
