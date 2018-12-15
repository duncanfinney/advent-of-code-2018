use itertools::Itertools;
use itertools::*;
use std::collections::HashMap;
use std::io::{stdout, Write};
use std::thread::sleep_ms;
use uuid::Uuid;

const DELAY: u32 = 0;

pub fn solve() {
    let input = include_str!("../input/day13");
    let mut input = parse_input(&input);

//    part_one(&mut input.clone());
    let answer = part_two(&mut input.clone());
    println!("part_two= {:?}", answer);
}

fn part_one(input: &mut Board) {
//    clear_console();
//    input.debug_print(0);
    sleep_ms(DELAY);

    for t in 1.. {
        let mut new_carts = input.carts.clone();
        new_carts.sort_by(|a, b| {
            if a.location.y == b.location.y {
                a.location.x.cmp(&b.location.x)
            } else {
                a.location.y.cmp(&b.location.y)
            }
        });
        for i in 0..new_carts.len() {
            if let Some(c) = new_carts.get_mut(i) {
                c.do_move(&input);
            }
            mark_crashes(&mut new_carts);
            if let Some(c) = new_carts.iter().find(|c| c.is_crashed) {
                println!("answer={:?}", c.location);
                return;
            }
        }

        input.carts = new_carts;
//        clear_console();
//        input.debug_print(t);
//        sleep_ms(DELAY);
    }
}

fn part_two(input: &mut Board) -> Location {

    for t in 1.. {

        // see if we are done
        if input.carts.len() == 1 {
            return input.carts.get(0).unwrap().location;
        }

        let mut new_carts = input.carts.clone();
        new_carts.sort_by(|a, b| {
            if a.location.y == b.location.y {
                a.location.x.cmp(&b.location.x)
            } else {
                a.location.y.cmp(&b.location.y)
            }
        });

        for i in 0..new_carts.len() {
            if let Some(c) = new_carts.get_mut(i) {
                c.do_move(&input);
            }
            mark_crashes(&mut new_carts);
        }

        input.carts = new_carts
            .iter()
            .filter_map(|c| match c {
                x if x.is_crashed => None,
                _ => Some(c.to_owned().clone())
            })
            .collect();

    }

    Location::default()
}

fn clear_console() {
    let stdout = stdout();
    let mut io = stdout.lock();
    io.write(&"\x1b[2J\x1b[1;1H".as_bytes()[..]).unwrap();
}

fn parse_input(input: &str) -> Board {
    let mut board = Board::default();
    input.lines().enumerate().for_each(|l| {
        let y = l.0 as u32;
        l.1.chars().enumerate().for_each(|pair| {
            let x = pair.0 as u32;
            let tile_type = match pair.1 {
                '/' => TileType::DiagonalForward,
                '\\' => TileType::DiagonalBackward,
                '+' => TileType::Intersection,
                '|' | '^' | 'v' => TileType::UpDown,
                '-' | '<' | '>' => TileType::LeftRight,
                _ => TileType::Empty,
            };

            let cart = match pair.1 {
                '^' => Some(Cart {
                    id: Uuid::new_v4(),
                    location: Location { x, y },
                    orientation: Direction::Up,
                    last_turn: None,
                    is_crashed: false,
                }),
                'v' => Some(Cart {
                    id: Uuid::new_v4(),
                    location: Location { x, y },
                    orientation: Direction::Down,
                    last_turn: None,
                    is_crashed: false,
                }),
                '<' => Some(Cart {
                    id: Uuid::new_v4(),
                    location: Location { x, y },
                    orientation: Direction::Left,
                    last_turn: None,
                    is_crashed: false,
                }),
                '>' => Some(Cart {
                    id: Uuid::new_v4(),
                    location: Location { x, y },
                    orientation: Direction::Right,
                    last_turn: None,
                    is_crashed: false,
                }),
                _ => None,
            };

            if let Some(c) = cart {
                board.carts.push(c);
            }

            board.set_tile(&Location { x, y }, tile_type);
        });
    });
    board
}

#[derive(Debug, Default, Clone)]
struct Board {
    tiles: HashMap<Location, TileType>,
    carts: Vec<Cart>,

    max_x: u32,
    max_y: u32,
}

fn mark_crashes(carts: &mut Vec<Cart>) {
    for (key, group) in &carts.into_iter().group_by(|l| l.location) {
        let mut v = group.collect_vec();
        if v.len() > 1 {
            for c in v {
                c.is_crashed = true
            }
        }
    }
}

impl Board {
    fn get_tile(&self, l: &Location) -> &TileType {
        self.tiles.get(l).unwrap_or(&TileType::Empty)
    }

    fn set_tile(&mut self, l: &Location, t: TileType) {
        if l.x > self.max_x {
            self.max_x = l.x;
        }
        if l.y > self.max_y {
            self.max_y = l.y
        }
        self.tiles.insert(l.clone(), t);
    }



    fn debug_print(&self, t: u32) {
        //        println!("-------------------------------------------");
        //        println!("t={}", t);
        for y in 0..=self.max_y {
            for x in 0..=self.max_x {
                let loc = Location { x, y };
                if let Some(c) = self.carts.iter().find(|c| &c.location == &loc) {
                    c.debug_print()
                } else {
                    let symbol = match self.get_tile(&Location { x, y }) {
                        TileType::Empty => " ",
                        TileType::UpDown => "|",
                        TileType::LeftRight => "-",
                        TileType::DiagonalForward => "/",
                        TileType::DiagonalBackward => r"\",
                        TileType::Intersection => "+",
                    };
                    print!("{}", symbol);
                }
            }
            print!("\n");
        }
    }
}

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone, Default)]
struct Location {
    x: u32,
    y: u32,
}

#[derive(Debug, Copy, Clone)]
enum TileType {
    Empty,
    UpDown,
    LeftRight,
    DiagonalForward,
    DiagonalBackward,
    Intersection,
}

#[derive(Debug, Clone)]
struct Cart {
    id: Uuid,
    location: Location,
    orientation: Direction,
    last_turn: Option<TurnOption>,
    is_crashed: bool,
}

impl Cart {
    fn debug_print(&self) {
        if self.is_crashed {
            print!("X");
            return;
        }
        use self::Direction::*;
        let c = match self.orientation {
            Up => '^',
            Down => 'v',
            Left => '<',
            Right => '>',
        };
        print!("{}", c);
    }

    fn do_move(&mut self, board: &Board) {
        use self::Direction::*;
        use self::TileType::*;

        // update location
        match &self.orientation {
            Up => self.location.y -= 1,
            Down => self.location.y += 1,
            Left => self.location.x -= 1,
            Right => self.location.x += 1,
        };

        // fix orientation
        self.orientation = match board.get_tile(&self.location) {
            DiagonalBackward => match self.orientation {
                Up => Left,
                Down => Right,
                Right => Down,
                Left => Up,
                _ => panic!("something is messed up"), //&self.orientation
            },

            DiagonalForward => match self.orientation {
                Up => Right,
                Down => Left,
                Right => Up,
                Left => Down,
                _ => panic!("something is messed up"),
            },

            Intersection => {
                let turn_dir = match self.last_turn {
                    Some(TurnOption::Left) => TurnOption::Straight,
                    Some(TurnOption::Straight) => TurnOption::Right,
                    Some(TurnOption::Right) => TurnOption::Left,
                    None => TurnOption::Left,
                };
                let orientation = self.orientation.get_turn_direction(&turn_dir.clone());
                self.last_turn = Some(turn_dir);
                orientation
            }

            _ => self.orientation.clone(), //no orientation change
        }
    }
}

#[derive(Debug, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn get_turn_direction(&self, d: &TurnOption) -> Self {
        use self::Direction::*;
        match d {
            TurnOption::Left => match self {
                Up => Left,
                Left => Down,
                Down => Right,
                Right => Up,
            },

            TurnOption::Right => match self {
                Up => Right,
                Left => Up,
                Down => Left,
                Right => Down,
            },

            TurnOption::Straight => self.clone(),
        }
    }
}

#[derive(Debug, Clone)]
enum TurnOption {
    Left,
    Right,
    Straight,
}
