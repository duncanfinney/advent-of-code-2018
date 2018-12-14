#![feature(slice_patterns, duration_as_u128, type_ascription)]

//mod day01;
//mod day02;
//mod day03;
//mod day04;
//mod day05;
//mod day06;
//mod day07;
//mod day08;
//mod day09;
//mod day10;
//mod day11;
mod day13;

fn main() {
    use std::time::Instant;
    let start_time = Instant::now();

    day13::solve();

    let duration = start_time.elapsed().as_millis();
    println!("runtime: {}ms", duration);
}
