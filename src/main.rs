mod solutions;
use solutions::*;
use std::fs;
use std::time;

type Today = Day25;

fn main() {
    let input = read_input("input.txt");
    let (time, ans) = with_timer(&|| Today::solve_part_1(input.clone()));
    println!("Part 1: {}", ans);
    println!("It took {} ms to solve part 1", time.as_millis());

    let (time, ans) = with_timer(&|| Today::solve_part_2(input.clone()));
    println!("Part 2: {}", ans);
    println!("It took {} ms to solve part 2", time.as_millis());
}

fn read_input(input_file_name: &str) -> String {
    fs::read_to_string(input_file_name).expect("Input Error")
}

fn with_timer<T>(f: &dyn Fn() -> T) -> (time::Duration, T) {
    let now = time::Instant::now();
    let result = f();
    let elapsed = now.elapsed();
    (elapsed, result)
}
