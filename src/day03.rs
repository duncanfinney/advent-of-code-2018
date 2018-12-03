use lazy_static::*;
use regex::Regex;
use std::str::FromStr;

pub fn solve() {
    let input = include_str!("../input/day03")
        .lines()
        .map(|l| { FabricClaim::from_str(&l).expect("invalid input") })
        .collect::<Vec<_>>();

    let answer = part_one(&input);
    println!("part1={}", answer);

    let answer = part_two(&input);
    println!("part1={}", answer);
}

#[derive(Debug, PartialEq)]
struct FabricClaim {
    number: u32,
    x: usize,
    y: usize,
    w: usize,
    h: usize,
}

impl FromStr for FabricClaim {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
        }
        //let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
        let caps = RE.captures(&s).unwrap();
        Ok(FabricClaim {
            number: caps.get(1).unwrap().as_str().parse().unwrap(),
            x: caps.get(2).unwrap().as_str().parse().unwrap(),
            y: caps.get(3).unwrap().as_str().parse().unwrap(),
            w: caps.get(4).unwrap().as_str().parse().unwrap(),
            h: caps.get(5).unwrap().as_str().parse().unwrap(),
        })
    }
}

fn part_one(input: &Vec<FabricClaim>) -> usize {
    let mut overlap = vec![vec![0 as usize; 1000]; 1000]; // 1000 by 1000 boolean of false

    input.iter().for_each(|fc| {
        for x in fc.x..(fc.x + fc.w) {
            for y in fc.y..(fc.y + fc.h) {
                overlap[y][x] += 1;
            }
        }
    });

    //count up the number that have more than 1 rectangle over it
    overlap
        .iter()
        .map(|row| {
            row.iter().filter(|cnt| { **cnt > 1 }).count()
        })
        .fold(0, |acc, x| { acc + x })
}

fn part_two(input: &Vec<FabricClaim>) -> u32 {
    let mut overlap = vec![vec![vec![0u32; 0]; 1000]; 1000]; // 1000 by 1000 list of ids

    input.iter().for_each(|fc| {

        for x in fc.x..(fc.x + fc.w) {
            for y in fc.y..(fc.y + fc.h) {
                overlap[y][x].push(fc.number);
            }
        }
    });
    0 //TODO
}