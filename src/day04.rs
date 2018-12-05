use chrono::prelude::*;
use lazy_static::*;
use regex::Regex;

pub fn solve() {
    let input = parse_input(include_str!("../input/day04"));
    println!("input={:?}", input);

    let answer = part_one();
    println!("part1={:?}", answer);

    let answer = part_two();
    println!("part2={:?}", answer);
}

fn parse_input(input: &str) -> Vec<GuardAction> {
    //    "[1518-04-29 23:59] Guard #457 begins shift"
    //    let regexes = RegexSet::new(&[
    //        r"[(^\])] Guard #(\d)+ begins shift",
    //        r"[(^\])] falls asleep",
    //        r"[(^\])] wakes up"
    //    ]).unwrap();

    //
    lazy_static! {
        static ref RE_BEGIN: Regex = Regex::new(r"^[(.*)] Guard #(\d+) begins shift").unwrap();
        static ref FALLS_ASLEEP: Regex = Regex::new(r"[(^\])] falls asleep").unwrap();
        static ref WAKES_UP: Regex = Regex::new(r"[(^\])] wakes up").unwrap();
    }

    let mut cur_guard = -1;
    input
        .lines()
        .map(|l| match l {
            the_match if RE_BEGIN.is_match(l) => {
                let matches = RE_BEGIN.captures(l).unwrap();
                println!("{:?}", matches);
                GuardAction::StartShift {
                    guard_num: 10,
                    time: Utc::now(),
                }
            }
            the_match if FALLS_ASLEEP.is_match(l) => GuardAction::FallAsleep {
                guard_num: 10,
                time: Utc::now(),
            },
            the_match if WAKES_UP.is_match(l) => GuardAction::WakeUp {
                guard_num: 10,
                time: Utc::now(),
            },
            _ => panic!("bad line"),
        })
        //        .map(|l| match regexes.matches(l) {
        //            regex::SetMatches{matches: [true, ..], ..} => {
        //                println!("{:?}", l);
        //                GuardAction::WakeUp {guard_num: 10, time: Utc::now()}
        //            }
        //        })
        .collect()
}

fn part_one() {}

#[derive(PartialEq, Debug)]
enum GuardAction {
    StartShift { guard_num: u32, time: DateTime<Utc> },
    FallAsleep { guard_num: u32, time: DateTime<Utc> },
    WakeUp { guard_num: u32, time: DateTime<Utc> },
}

fn part_two() {}
