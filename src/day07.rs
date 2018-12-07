use lazy_static::*;
use regex::Regex;
use std::collections::HashMap;

pub fn solve() {
    let input = include_str!("../input/day07");
    let input = parse_input(&input);
}

fn parse_input(input: &'static str) {
    let mut ret: HashMap<char, Step> = HashMap::new();
    input
        .lines()
        .map(|l| {
            lazy_static! {
                static ref RE: Regex =
                    Regex::new("^Step ([A-Z]) must be finished before step ([A-Z]) can begin.$")
                        .unwrap();
            }
            let caps = RE.captures(l).expect("your regex is no good");

            {
                let step = caps.get(1).unwrap().as_str().chars().next().unwrap();
                let step = ret.entry(step).or_insert_with(|| Step::new_with_letter(step));
                println!( "step={:?}", step );
            }

            {
                let step_before = caps.get(2).unwrap().as_str().chars().next().unwrap();
                let step_before = ret.entry(step_before).or_insert_with(|| Step::new_with_letter(step_before));
                println!( "step_before={:?}", step_before);
            }


            0
        })
        .collect::<Vec<_>>();
}

#[derive(Debug)]
struct Step {
    requirements: Vec<Step>,
    letter: char,
}

impl Step {
    pub fn new_with_letter(letter: char) -> Self {
        Step {
            letter,
            requirements: vec![],
        }
    }
}

fn part_one() {}

fn part_two() {}
