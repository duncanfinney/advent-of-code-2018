use itertools::*;
pub fn solve() {
    let input = include_str!("../input/day15");
    let input = parse_input(&input);

    input.debug_print();
}

fn parse_input(input: &str) -> Board {
    let mut units = vec![];

    let tiles = input
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '#' => Tile::Wall,
                    '.' => Tile::OpenCavern,
                    'E' => {
                        units.push(Unit {
                            x,
                            y,
                            hp: 200,
                            species: Species::Elf,
                        });
                        Tile::OpenCavern
                    }
                    'G' => {
                        units.push(Unit {
                            x,
                            y,
                            hp: 200,
                            species: Species::Goblin,
                        });
                        Tile::OpenCavern
                    }
                    _ => panic!("bad puzzle input"),
                })
                .collect::<Vec<Tile>>()
        })
        .collect_vec();

    Board { tiles, units }
}

#[derive(Debug)]
struct Board {
    tiles: Vec<Vec<Tile>>,
    units: Vec<Unit>,
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

impl Board {
    fn debug_print(&self) {
        for (y, r) in self.tiles.iter().enumerate() {
            for (x, c) in r.iter().enumerate() {
                let rep = match self.units.iter().find(|c| c.is_on_tile(x, y)) {
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
}

impl Unit {
    fn is_on_tile(&self, cx: usize, cy: usize) -> bool {
        self.x == cx && self.y == cy
    }
}
