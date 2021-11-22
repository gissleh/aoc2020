use std::fmt::Display;
use std::fs::File;
use std::io::Read;
use time::PreciseTime;

pub fn load_input(name: &str) -> String {
    let mut buf = String::with_capacity(2048);

    match File::open(format!("./input/{}.txt", name)) {
        Ok(mut file) => {
            file.read_to_string(&mut buf).unwrap();
        }
        Err(e) => {
            panic!("Could not load file ./input/{}.txt: {}", name, e);
        }
    }

    buf
}

pub fn load_input_bytes(name: &str) -> Vec<u8> {
    let mut buf = Vec::with_capacity(2048);

    match File::open(format!("./input/{}.txt", name)) {
        Ok(mut file) => {
            file.read_to_end(&mut buf)
                .expect("Could not read file ./input/{}.txt");
        }
        Err(e) => {
            panic!("Could not load file ./input/{}.txt: {}", name, e);
        }
    }

    buf
}

pub fn run_once<T>(callback: impl Fn() -> T) -> (T, i64) {
    let start = PreciseTime::now();
    let result = callback();
    let end = PreciseTime::now();

    (result, start.to(end).num_nanoseconds().unwrap())
}

pub fn run_many<T>(times: usize, callback: impl Fn() -> T) -> (T, i64) {
    let start = PreciseTime::now();
    let mut result = callback();
    for _ in 1..times {
        result = callback();
    }
    let end = PreciseTime::now();

    (
        result,
        start.to(end).num_nanoseconds().unwrap() / times as i64,
    )
}

pub fn run_many_mut<T>(times: usize, mut callback: impl FnMut() -> T) -> (T, i64) {
    let start = PreciseTime::now();
    let mut result = callback();
    for _ in 1..times {
        result = callback();
    }
    let end = PreciseTime::now();

    (
        result,
        start.to(end).num_nanoseconds().unwrap() / times as i64,
    )
}

pub fn print_result(label: &str, result: impl Display) {
    println!("Result ({}): {}", label, result);
}

pub fn print_two_results(label: &str, result1: impl Display, result2: impl Display) {
    println!("Result ({}): {} {}", label, result1, result2);
}

pub fn print_result_multiline(label: &str, result: impl Display) {
    println!("Result ({}):\n{}", label, result);
}

pub fn print_time(label: &str, ns: i64) {
    if ns > 10_000_000_000 {
        println!(
            "Duration ({}): {:.1}s",
            label,
            (ns as f64) / (1_000_000_000 as f64)
        );
    } else if ns > 1_000_000_000 {
        println!(
            "Duration ({}): {:.4}s",
            label,
            (ns as f64) / (1_000_000_000 as f64)
        );
    } else if ns > 1_000_000 {
        println!(
            "Duration ({}): {:.3}ms",
            label,
            (ns as f64) / (1_000_000 as f64)
        );
    } else if ns > 1_000 {
        println!(
            "Duration ({}): {:.2}µs",
            label,
            (ns as f64) / (1_000 as f64)
        );
    } else {
        println!("Duration ({}): {}ns", label, ns);
    }
}
