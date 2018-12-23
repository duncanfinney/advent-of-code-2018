#[allow(unused)]
use itertools::*;
use std::collections::BTreeMap;
use std::cmp::{Ord, Ordering};

pub fn solve() {
    let input = include_str!("../input/day15");
    let input = parse_input(&input);

    input.debug_print();
}

fn parse_input(input: &str) -> Board {
    let mut units = BTreeMap::new();
    let mut tiles = BTreeMap::new();

    input
        .lines()
        .enumerate()
        .for_each(|(y, l)| {
            l.chars()
                .enumerate()
                .for_each(|(x, c)| {
                    let coordinate = Coordinate { x, y };
                    match c {
                        '#' =>  { tiles.insert(coordinate, Tile::Wall); },
                        '.' => { tiles.insert(coordinate, Tile::OpenCavern); },
                        'E' => {
                            units.insert(coordinate, Unit {
                                x,
                                y,
                                hp: 200,
                                species: Species::Elf,
                            });
                            tiles.insert(coordinate, Tile::OpenCavern);
                        }
                        'G' => {
                            units.insert(coordinate, Unit {
                                x,
                                y,
                                hp: 200,
                                species: Species::Goblin,
                            });
                            tiles.insert(coordinate, Tile::OpenCavern);
                        }
                        _ => panic!("bad puzzle input"),
                    }
                });
        });

    Board { tiles, units }
}

#[derive(Debug)]
struct Board {
    tiles: BTreeMap<Coordinate, Tile>,
    units: BTreeMap<Coordinate, Unit>,
}

#[derive(Debug)]
enum Tile {
    Wall,
    OpenCavern,
}

#[derive(Debug, PartialEq)]
enum Species {
    Goblin,
    Elf,
}

#[derive(Debug)]
struct Unit {
    species: Species,
    hp: i32,
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy, Default, Eq, Hash, PartialEq)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn distance(&self, other: Coordinate) -> usize {
        let x = (self.x as isize - other.x as isize).abs();
        let y = (self.y as isize - other.y as isize).abs();
        (x + y) as usize
    }
}

impl Ord for Coordinate {
    fn cmp(&self, other: &Coordinate) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Coordinate {
    fn partial_cmp(&self, other: &Coordinate) -> Option<Ordering> {
        Some((self.y, self.x).cmp(&(other.y, other.x)))
    }
}


impl Board {
    fn debug_print(&self) {
        for y in 0..=35 {
            for x in 0..=35 {
                let c =  self.tiles.get(&Coordinate{ x, y }).unwrap_or(&Tile::Wall);
                let rep = match self.units.iter().map(|pair| pair.1).find(|c| c.is_on_tile(x, y)) {
                    Some(Unit { species, .. }) => match species {
                        Species::Elf => "E",
                        Species::Goblin => "G",
                    },
                    None => match c {
                        Tile::Wall => "#",
                        Tile::OpenCavern => ".",
                    },
                };
                print!("{}", rep);
            }

            print!("\n");
        }
    }

    fn do_tick(&mut self) -> bool {
        let mut any_move = false;
        let unit_locations : Vec<_> = self.units.keys().cloned().collect();

        false
    }
}

impl Unit {
    fn is_on_tile(&self, cx: usize, cy: usize) -> bool {
        self.x == cx && self.y == cy
    }

    fn is_enemy(&self, other: &Unit) -> bool {
        self.species != other.species
    }
}
