use std::time::Duration;

pub fn print_day_banner(day: usize) {
    println!("############");
    println!("#  DAY {:02}  #", day);
    println!("############");
}

pub fn time_it<F: Fn()>(f: F) -> Duration {
    let start_time = std::time::Instant::now();

    f();

    start_time.elapsed()
}

pub fn print_duration(duration: Duration, day: usize) {
    let ms = duration.as_millis();

    // When the code is too fast, switch to nanoseconds!
    if ms == 0 {
        let ns = duration.as_nanos();

        println!(
            "Day {} took {}ns\n",
            day,
            ns
        );
    } else {
        println!(
            "Day {} took {}ms\n",
            day,
            ms
        );
    }
}