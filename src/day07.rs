use lazy_static::*;
use petgraph::graphmap::GraphMap;
use petgraph::Directed;
use petgraph::Direction;
use regex::Regex;
use itertools::*;
use std::collections::HashSet;

type Graph = GraphMap<char, u32, Directed>;

pub fn solve() {
    let input = include_str!("../input/day07");
    let input = parse_input(&input);

    let answer = part_one(input);
    println!("part1={:?}", answer);
}

fn parse_input(input: &'static str) -> Graph {
    let mut gr = GraphMap::<_, _, Directed>::with_capacity(26, 26*26);

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


fn part_one(input :Graph) -> String {
    let mut ret = vec![];
    let mut input = input.clone();
    while input.node_count() > 0 {
//        println!("---");
//        println!("input: {:?}", input);
//        println!("node_count={:?}", input.node_count());
//        println!("---");
        let mut next_node = input.nodes().filter(|n| { input.neighbors_directed(*n, Direction::Incoming).count() == 0 }).min().expect("something went wrong");
        ret.push(next_node);
        input.remove_node(next_node);
    }
    ret.iter().collect()
}

fn part_two() {}
