use lazy_static::*;
use petgraph::graphmap::GraphMap;
use petgraph::Directed;
use petgraph::Direction;
use regex::Regex;
use std::collections::HashSet;

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
        for w in worker_pool.iter_mut().filter(|w| w.working_on.is_some()) {
            w.work_left -= 1;
            if w.work_left <= 0 {
                input.remove_node(w.working_on.unwrap());
                w.working_on = None;
            }
        }

        let being_worked_on: HashSet<char> = {
            worker_pool
                .iter()
                .filter_map(|w| w.working_on)
                .collect::<HashSet<_>>()
        };
        let mut able_to_work_on: Vec<_> = input
            .nodes()
            // remove the ones that are already being worked on
            .filter(|n| !being_worked_on.contains(n))
            // and the ones that have dependencies
            .filter(|n| input.neighbors_directed(*n, Direction::Incoming).count() == 0)
            .collect();

        able_to_work_on.sort();

        loop {
            if able_to_work_on.is_empty() {
                break;
            }

            let worker = worker_pool.iter_mut().find(|w| w.working_on.is_none());
            if let Some(w) = worker {
                w.work_on(&able_to_work_on.remove(0));
                continue;
            } else {
                break;
            }
        }

        secs_elapsed = secs_elapsed + 1;
    }
    secs_elapsed - 1
}

#[derive(Debug, Default, PartialEq)]
struct Worker {
    working_on: Option<char>,
    work_left: i32,
}

impl Worker {
    fn get_pool(num: u32) -> Vec<Worker> {
        (0..num).map(|_| Worker::default()).collect()
    }

    fn work_on(&mut self, c: &char) {
        self.working_on = Some(*c);
        self.work_left = 60 + 1 + *c as i32 - 'A' as i32;
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
        assert_ne!(answer, 1504);
        assert_ne!(answer, 1505);
        assert_ne!(answer, 1506);

        assert_eq!(answer, 848); // FINALLY
    }
}
