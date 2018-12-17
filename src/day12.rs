use std::collections::HashMap;

pub fn solve() {
    let input = include_str!("../input/day12");
    let mut input = parse_input(&input);

    // part one
    print!("0: ");
    input.debug_print();
    for n in 0..20 {
        print!("{}: ", n);
        println!("min, max = {}, {}", input.min, input.max);
        input.do_generation();
        input.debug_print();
    }

    let mut sum = 0;
    for (n, plant_state) in input.pot_states.iter().filter(|p| p.1 == &PotState::Pot) {
        sum += n;
    }
    println!("part_one= {}", sum);


    //TODO: figure out how part two works... solution shamelessly yanked from: https://www.reddit.com/r/adventofcode/comments/a5eztl/2018_day_12_solutions/ebm4exr
}

fn parse_input(input: &str) -> Pots {
    let mut lines = input.lines();
    let mut split = lines.next().unwrap().split(":");
    split.next().unwrap();
    let pot_states = split.next().unwrap().trim();
    let pot_states = pot_states
        .chars()
        .map(PotState::from)
        .enumerate()
        .map(|(size, ps)| (size as i32, ps))
        .collect();

    //burn a line
    lines.next();

    //repacements
    let replacements = lines
        .map(|l| {
            let mut split = l.split(" => ");
            let old = split.next().unwrap();
            let old = old.chars().map(PotState::from).collect::<Vec<_>>();

            let new = split.next().unwrap().chars().next().unwrap();
            let new = PotState::from(new);
            Replacement { old, new }
        })
        .collect::<Vec<_>>();

    Pots::new(pot_states, replacements)
}

#[derive(Debug)]
struct Pots {
    pot_states: HashMap<i32, PotState>,
    replacements: Vec<Replacement>,
    min: i32,
    max: i32,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum PotState {
    Pot,
    Empty,
}

impl From<char> for PotState {
    fn from(c: char) -> Self {
        match c {
            '#' => PotState::Pot,
            '.' => PotState::Empty,
            _ => panic!("bad input"),
        }
    }
}

#[derive(Debug)]
struct Replacement {
    old: Vec<PotState>,
    new: PotState,
}

impl Pots {
    fn new(pots: HashMap<i32, PotState>, replacements: Vec<Replacement>) -> Self {
        let min = pots.iter().map(|p| p.0).min().unwrap().to_owned();
        let max = pots.iter().map(|p| p.0).max().unwrap().to_owned();

        Pots {
            pot_states: pots,
            replacements,
            min,
            max,
        }
    }

    fn get(&self, n: i32) -> &PotState {
        self.pot_states.get(&n).unwrap_or(&PotState::Empty)
    }

    fn set(&mut self, n: i32, p: PotState) {
        self.pot_states.insert(n, p);
        if n > self.max {
            self.max = n;
        }
        if n < self.min {
            self.min = n;
        }
    }

    fn do_generation(&mut self) {
        let mut new_pots = HashMap::new();
        let mut min = 0;
        let mut max = 0;
        for pot_n in self.min-2..=self.max+2 {
            let pattern = vec![
                *self.get(pot_n - 2),
                *self.get(pot_n - 1),
                *self.get(pot_n),
                *self.get(pot_n + 1),
                *self.get(pot_n + 2),
            ];
            let new_state = match self.replacements.iter().find(|r| r.old.eq(&pattern)) {
                Some(r) => r.new,
                None => PotState::Empty,
            };

            new_pots.insert(pot_n, new_state);
            if pot_n < min {
                min = pot_n
            }
            if pot_n > max {
                max = pot_n;
            }
        }
        self.pot_states = new_pots;
        self.min = min;
        self.max = max;
    }

    fn debug_print(&self) {
//        let dbg_str :String = self
//            .pot_states
//            .iter()
//            .map(|p| match p.1 {
//                PotState::Pot => "#",
//                PotState::Empty => ".",
//            })
//            .collect();
        for n in -2..30 {
            let c = match self.pot_states.get(&n) {
              Some(x) if x == &PotState::Pot => "#",
               _ => "."
           };
           print!("{}", c)
        }
        print!("\n");
//        println!("{}", dbg_str);
    }
}
