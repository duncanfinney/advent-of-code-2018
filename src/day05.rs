//use cached::macros::*;

pub fn solve() {
    let input = include_str!("../input/day05").chars().collect::<Vec<_>>();

    let answer = part_one(&input);
    println!("part1={}", answer);

    let answer = part_two(&input);
    println!("part2={}", answer);
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
    stack.len()
}

fn part_two(input: &Vec<char>) -> u32 {
    // An iterator over the lowercase alpha's
    (0..26)
        .map(|x| (x + 'a' as u8) as char)
        .map(|to_try| {
            let mut stack = Vec::new();
            for c in input {
                if *c != to_try && *c != get_opposite_polarity(&to_try) {
                    stack.push(*c);
                    continue;
                }

                let twin = &get_opposite_polarity(&c);
                let top = stack.last();
                if top.is_some() && top.unwrap() == twin {
                    stack.pop();
                } else {
                    stack.push(*c);
                }
            }
            let len = stack.len() as u32;
            len
        })
        .min()
        .unwrap()
        .into()
}

fn get_opposite_polarity(c: &char) -> char {
    if c.is_lowercase() {
        c.to_uppercase().next().unwrap()
    } else {
        c.to_lowercase().next().unwrap()
    }
}
