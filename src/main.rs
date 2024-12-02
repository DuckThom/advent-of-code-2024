use std::env;

mod utils;
mod day_1;
mod day_2;

#[derive(Debug, PartialEq)]
enum MethodType {
    EXECUTE,
    VALIDATE,
}

fn main() {
    let execute_days: Vec<fn()> = vec![day_1::execute, day_2::execute];
    let validate_days: Vec<fn()> = vec![day_1::validate, day_2::validate];

    let args: Vec<String> = env::args().collect();

    let method: MethodType = args
        .get(1)
        .map(|s| match s.as_str() {
            "execute" => MethodType::EXECUTE,
            "validate" => MethodType::VALIDATE,
            _ => panic!(
                "Unknown method: {}. Valid arguments: 'execute' (default), 'validate'",
                args[1].as_str()
            ),
        })
        .unwrap_or(MethodType::EXECUTE);

    let day: usize = args.get(2).map(|s| s.parse().unwrap()).unwrap_or(0);

    let list_to_use: Vec<fn()> = match method {
        MethodType::EXECUTE => execute_days,
        MethodType::VALIDATE => validate_days,
    };

    if day > list_to_use.len() {
        panic!("Day {} is not implemented", day);
    }

    if day == 0 {
        for i in 1..=2 {
            let took = utils::time_it(list_to_use[i - 1]);

            if method == MethodType::EXECUTE {
                utils::print_duration(took, i);
            }
        }
    } else {
        let took = utils::time_it(list_to_use[day - 1]);

        if method == MethodType::EXECUTE {
            utils::print_duration(took, day);
        }
    }
}


