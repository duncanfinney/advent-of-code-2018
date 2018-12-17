use std::collections::HashMap;

pub fn solve() {
    let input = include_str!("../input/day12");
    let input = parse_input(&input);
}

fn parse_input(input: &str) -> Pots {

    let mut lines = input.lines();
    let mut split = lines.next().unwrap().split(":");
    split.next().unwrap();
    let pot_states = split.next().unwrap().trim();
    let pot_states = pot_states
        .chars()
        .map(|c| match c {
            '#' => PotState::Pot,
            '.' => PotState::Empty,
            _ => panic!("bad input"),
        })
        .enumerate()
        .map(|(size, ps)| (size as i32, ps))
        .collect();

    println!("pot_state= {:?}", pot_states);
    Pots {
        pot_states
    }

}

#[derive(Debug)]
struct Pots {
    pot_states: HashMap<i32, PotState>
}

#[derive(Debug, PartialEq)]
enum PotState {
    Pot,
    Empty
}

#[derive(Debug)]
struct Replacement {
    old: Vec<PotState>,
    new: Vec<PotState>
}

impl Pots {
    fn get(&self, n: i32) -> &PotState {
        self.pot_states.get(&n).unwrap_or(&PotState::Empty)
    }
}

