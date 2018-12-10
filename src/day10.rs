use lazy_static::*;
use regex::Regex;

pub fn solve() {
    let input = include_str!("../input/day10");
    let mut input = parse_input(input);

    let mut t = 1i64;
    let mut last_area = std::i64::MAX;
    while last_area >= input.extent_at_time(t).area() {
        last_area = input.extent_at_time(t).area();
        t += 1;
    }

    println!("part_one:");
    input.print(t - 1);

    println!("part_two= {:?}", t-1);
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Light {
    x: i64,
    y: i64,
    velocity_x: i64,
    velocity_y: i64,
}

impl Light {
    fn position_at_time(&self, t: i64) -> (i64, i64) {
        (
            self.x + (self.velocity_x * t),
            self.y + (self.velocity_y * t),
        )
    }
}

#[derive(Debug)]
struct Board(Vec<Light>);

#[derive(Debug, Copy, Clone, PartialEq)]
struct Extent {
    x_min: i64,
    x_max: i64,
    y_min: i64,
    y_max: i64,
}

impl Extent {
    fn area(&self) -> i64 {
        (self.x_max - self.x_min) * (self.y_max - self.y_min)
    }
}

impl Board {
    fn extent_at_time(&self, t: i64) -> Extent {
        Extent {
            x_min: self
                .0
                .iter()
                .map(|l| l.position_at_time(t).0)
                .min()
                .unwrap(),
            x_max: self
                .0
                .iter()
                .map(|l| l.position_at_time(t).0)
                .max()
                .unwrap(),
            y_min: self
                .0
                .iter()
                .map(|l| l.position_at_time(t).1)
                .min()
                .unwrap(),
            y_max: self
                .0
                .iter()
                .map(|l| l.position_at_time(t).1)
                .max()
                .unwrap(),
        }
    }

    fn print(&self, t: i64) {
        let Extent {
            x_min,
            x_max,
            y_min,
            y_max,
        } = self.extent_at_time(t);
        for y in y_min..=y_max {
            for x in x_min..=x_max {
                let c = if self.0.iter().map(|l| { l.position_at_time(t) }).any(|l| l.0 == x && l.1 == y) {
                    "#"
                } else {
                    "."
                };
                print!("{}", c);
            }
            print!("\n");
        }
        print!("\n\n");
    }
}

fn parse_input(input: &str) -> Board {
    let lights = input
        .lines()
        .map(|l| {
            lazy_static! {
                static ref NUMBER_RE: Regex = Regex::new(r"-?\d+").unwrap();
            };

            let mut iter = NUMBER_RE.captures_iter(l);
            let mut next_num_please = move || {
                iter.next()
                    .unwrap()
                    .get(0)
                    .unwrap()
                    .as_str()
                    .parse::<i64>()
                    .unwrap()
            };
            Light {
                x: next_num_please(),
                y: next_num_please(),
                velocity_x: next_num_please(),
                velocity_y: next_num_please(),
            }
        })
        .collect();

    Board(lights)
}

