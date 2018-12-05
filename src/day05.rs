//use cached::macros::*;

pub fn solve() {
    let input = include_str!("../input/day05").chars().collect::<Vec<_>>();

    let answer = part_one(&input);
    println!("part1={}", answer);

    let answer = part_one(&input);
    //    println!("part1={}", answer);
}

fn part_one(input: &Vec<char>) -> usize {
    let mut stack = Vec::new();
    for c in input {
        let twin = &get_opposite_polarity(&c);
        let top = stack.last();
        if top.is_some() && top.unwrap() == twin {
            stack.pop();
        } else {
            stack.push(*c);
        }
    }
    stack.into_iter().count()
}

fn part_two() -> String {
    "".to_string()
}

fn get_opposite_polarity(c: &char) -> char {
    if c.is_lowercase() {
        c.to_uppercase().next().unwrap()
    } else {
        c.to_lowercase().next().unwrap()
    }
}
