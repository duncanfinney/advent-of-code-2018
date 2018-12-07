#![feature(slice_patterns, duration_as_u128)]

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day07;

fn main() {
    use std::time::Instant;
    let start_time = Instant::now();

    day07::solve();

    let duration = start_time.elapsed().as_millis();
    println!("runtime: {}ms", duration);
}
