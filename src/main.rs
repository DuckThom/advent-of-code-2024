use std::env;

mod days;

#[derive(Debug)]
enum MethodType {
    EXECUTE,
    VALIDATE,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let method: MethodType = match args[1].as_str() {
        "execute" => MethodType::EXECUTE,
        "validate" => MethodType::VALIDATE,
        _ => panic!("Unknown method: {}. Valid arguments: 'execute', 'validate'", args[1].as_str()),
    };
    let day: i8 = args[2].parse().unwrap();

    match method {
        MethodType::EXECUTE => {
            match day {
                1 => days::day_1::execute(),
                2 => days::day_2::execute(),
                _ => panic!("Day not implemented: {}", day),
            }
        }
        MethodType::VALIDATE => {
            match day {
                1 => days::day_1::validate(),
                2 => days::day_2::validate(),
                _ => panic!("Day not implemented: {}", day),
            }
        }
    }
}
