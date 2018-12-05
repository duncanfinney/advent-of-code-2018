use std::collections::HashMap;
use std::collections::HashSet;

pub fn solve() {
    let input = include_str!("../input/day02").lines().collect::<Vec<_>>();

    let answer = part_one(&input);
    println!("part1={}", answer);

    let answer = part_two(&input);
    println!("part2={:?}", answer);
}

fn part_one(input: &[&str]) -> usize {
    let to_hash: Vec<Counts> = input
        .iter()
        .filter_map(|s: &&str| -> _ {
            let mut letter_counts = HashMap::new();
            for ch in s.chars() {
                let counter = letter_counts.entry(ch).or_insert(0);
                *counter += 1;
            }

            let has_two = letter_counts.iter().any(|(&ch, &count)| count == 2);
            let has_three = letter_counts.iter().any(|(&ch, &count)| count == 3);
            if has_two || has_three {
                Some(Counts { has_two, has_three })
            } else {
                None
            }
        })
        .collect();

    let num_with_two = to_hash.iter().filter(|c| c.has_two).count();
    let num_with_three = to_hash.iter().filter(|c| c.has_three).count();
    num_with_two * num_with_three
}

struct Counts {
    has_two: bool,
    has_three: bool,
}

fn part_two(input: &[&str]) -> Option<String> {
    let mut lookup = HashSet::new();
    let answer = input.iter().find_map(|s| {
        //split the char at character i
        for i in 0..s.len() {
            let mut key = s[0..i].to_owned();
            key.push_str("_");
            key.push_str(&s[(i + 1)..]);
            if lookup.contains(&key) {
                return Some(key.replace("_", ""));
            }
            lookup.insert(key);
        }
        None
    });
    answer
}
