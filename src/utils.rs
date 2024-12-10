use std::env;
use std::fs::File;
use std::io::Read;
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

        println!("Day {} took {}ns\n", day, ns);
    } else if ms > 10_000 {
        let s = duration.as_secs();

        println!("Day {} took {}s\n", day, s);
    } else {
        println!("Day {} took {}ms\n", day, ms);
    }
}

pub async fn download_input(day: usize) {
    use std::io::Write;

    let url = format!("https://adventofcode.com/2024/day/{}/input", day);

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "advent-of-code-helper")
        .header(
            "Cookie",
            format!(
                "session={}",
                env::var("AOC_SESSION")
                    .expect("env var AOC_SESSION not set, cannot auto-download input")
            ),
        )
        .send()
        .await
        .expect("Failed to send request");

    if response.status().is_success() {
        let data = response.text().await.expect("Failed to read response");

        let mut file =
            File::create(format!("inputs/day_{}/input", day)).expect("Failed to create input file");

        write!(file, "{}", data.trim_end()).expect("Could not write to file");
    } else {
        panic!("Failed to download input: {}", response.status());
    }
}

pub fn read_input_file(day: usize) -> String {
    let file_path = format!("inputs/day_{}/input", day);
    let mut file = File::open(&file_path).expect("Input file not found");
    let mut data = String::new();

    file.read_to_string(&mut data)
        .expect(format!("Failed to read input file: {}", &file_path).as_str());

    data
}

pub fn input_to_char_matrix(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

pub fn input_to_usize_matrix(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .filter_map(|c| c.to_digit(10).map(|d| d as usize))
                .collect()
        })
        .collect()
}
