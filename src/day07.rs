use lazy_static::*;
use petgraph::graphmap::GraphMap;
use petgraph::Directed;
use petgraph::Direction;
use regex::Regex;
use std::collections::VecDeque;

type Graph = GraphMap<char, u32, Directed>;

pub fn solve() {
    let input = include_str!("../input/day07");
    let input = parse_input(&input);

    let answer = part_one(&input);
    println!("part1= {}", answer);

    let answer = part_two(&input);
    println!("part2= {:?}", answer);
}

fn parse_input(input: &'static str) -> Graph {
    let mut gr = GraphMap::<_, _, Directed>::with_capacity(26, 26 * 26);

    input.lines().for_each(|l| {
        lazy_static! {
            static ref RE: Regex =
                Regex::new("^Step ([A-Z]) must be finished before step ([A-Z]) can begin.$")
                    .unwrap();
        }
        let caps = RE.captures(l).expect("your regex is no good");

        let step_must_happen_first = caps.get(1).unwrap().as_str().chars().next().unwrap();
        let step = caps.get(2).unwrap().as_str().chars().next().unwrap();
        gr.add_edge(step_must_happen_first, step, 1);
    });

    gr
}

fn part_one(input: &Graph) -> String {
    let mut ret = vec![];
    let mut input = input.clone();
    while input.node_count() > 0 {
        let next_node = input
            .nodes()
            .filter(|n| input.neighbors_directed(*n, Direction::Incoming).count() == 0)
            .min()
            .expect("something went wrong");
        ret.push(next_node);
        input.remove_node(next_node);
    }
    ret.iter().collect()
}

fn part_two(input: &Graph) -> u32 {
    let mut input = input.clone();

    let mut worker_pool = Worker::get_pool(5);
    let mut secs_elapsed = 0;

    while input.node_count() > 0 || worker_pool.iter().any(|w| w.working_on.is_some()) {
        let mut looking_for_work: VecDeque<_> = worker_pool
            .iter_mut()
            .filter(|w| w.working_on.is_none())
            .collect();

        let mut able_to_work_on: VecDeque<_> = input
            .nodes()
            .filter(|n| input.neighbors_directed(*n, Direction::Incoming).count() == 0)
            .collect();

        while !looking_for_work.is_empty() && !able_to_work_on.is_empty() {
            let worker = looking_for_work.pop_front().unwrap();
            let work = able_to_work_on.pop_front().unwrap();
            worker.work_on(&work);
            input.remove_node(work);
        }

        for w in worker_pool.iter_mut().filter(|w| w.work_left > 0) {
            w.work_left -= 1;
            if w.work_left == 0 {
                w.working_on = None;
            }
        }

        secs_elapsed = secs_elapsed + 1;
    }
    secs_elapsed
}

#[derive(Debug, Default, PartialEq)]
struct Worker {
    working_on: Option<char>,
    work_left: u32,
}

impl Worker {
    fn get_pool(num: u32) -> Vec<Worker> {
        (0..num).map(|_| Worker::default()).collect()
    }

    fn work_on(&mut self, c: &char) {
        self.working_on = Some(*c);
        self.work_left = 61 + (*c as u32 - 'A' as u32);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_work_on() {
        let mut worker = Worker {
            working_on: None,
            work_left: 0,
        };
        worker.work_on(&'B');
        assert_eq!(
            worker,
            Worker {
                working_on: Some('B'),
                work_left: 62
            }
        );
    }

    #[test]
    fn test_bad_guesses() {
        let input = include_str!("../input/day07");
        let input = parse_input(&input);

        let answer = part_two(&input);
        assert_ne!(answer, 336);
        assert_ne!(answer, 341);
        assert_ne!(answer, 342); // STILL too low. I quit
        assert_ne!(answer, 425); // STILL too low. I quit
    }

    #[test]
    fn run_example() {
        let input = r#"Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin."#

    }
}
