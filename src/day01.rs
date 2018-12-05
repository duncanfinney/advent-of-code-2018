use std::collections::HashSet;

pub fn solve() {
    let input = include_str!("../input/day01")
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let part1 = input.iter().fold(0, |acc, x| acc + x);
    println!("part1={}", part1);

    let part2 = part_two(&input);
    println!("part2={}", part2);
}

fn part_two(input: &[i32]) -> i32 {
    let mut items = HashSet::new();
    let mut current_freq = 0;
    let mut i = 0;
    loop {
        current_freq = current_freq + input[i];
        if items.contains(&current_freq) {
            return current_freq;
        }
        items.insert(current_freq);
        i = (i + 1) % input.len()
    }
}
