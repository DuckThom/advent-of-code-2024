use std::env;

mod days;

#[derive(Debug)]
enum MethodType {
    EXECUTE,
    VALIDATE,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let day: i8 = args[1].parse().unwrap();
    let method: MethodType = match args[2].as_str() {
        "execute" => MethodType::EXECUTE,
        "validate" => MethodType::VALIDATE,
        _ => panic!("Unknown method: {}. Valid arguments: 'execute' (default), 'validate'", args[2].as_str()),
    };

    match method {
        MethodType::EXECUTE => {
            match day {
                1 => days::day_1::execute(),
                _ => panic!("Day not implemented: {}", day),
            }
        }
        MethodType::VALIDATE => {
            match day {
                1 => days::day_1::validate(),
                _ => panic!("Day not implemented: {}", day),
            }
        }
    }
}
