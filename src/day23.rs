use itertools::Itertools;
use regex::Regex;

pub fn solve() {
    let input = include_str!("../input/day23");
    let input = parse_input(input);

    let answer = part_one(&input);
    println!("part_one= {}", answer);
}

fn parse_input(input: &str) -> Vec<Nanobot> {
    let parse_regex = Regex::new(r"pos=<([^,]+),([^,]+),([^>]+)>, r=([-\d]+)").unwrap();
    input
        .lines()
        .map(|l| {
            let mut captures_iter = parse_regex.captures_iter(l);
            let cap = captures_iter.next().unwrap();

            match (cap.get(1), cap.get(2), cap.get(3), cap.get(4)) {
                (Some(x), Some(y), Some(z), Some(r)) => {
                    let x = x.as_str().parse().unwrap();
                    let y = y.as_str().parse().unwrap();
                    let z = z.as_str().parse().unwrap();
                    let r = r.as_str().parse().unwrap();

                    Nanobot {
                        position: (x, y, z),
                        radius: r,
                    }
                }
                _ => panic!(print!("bad input line: {}", input)),
            }
        })
        .collect_vec()
}

fn part_one(input: &Vec<Nanobot>) -> usize {
    let biggest_nanobot = input.iter().max_by_key(|n| n.radius).unwrap();

    input.iter().filter(|n| biggest_nanobot.is_in_range(n)).count()
}

#[derive(Debug)]
struct Nanobot {
    position: (i64, i64, i64),
    radius: i64,
}

impl Nanobot {
    fn distance_to(&self, other: &Nanobot) -> i64 {
        (other.position.0 - self.position.0).abs()
            + (other.position.1 - self.position.1).abs()
            + (other.position.2 - self.position.2).abs()
    }

    fn is_in_range(&self, other: &Nanobot) -> bool {
        self.distance_to(other) < self.radius
    }
}
