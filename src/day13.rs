use std::collections::HashMap;

pub fn solve() {
    let input = include_str!("../input/day13");
}


fn parse_input(input :&str) {
    input
        .lines()
        .enumerate()
        .for_each(|l| {
            let y = l.0;
            l.1.chars().enumerate().for_each(|x| match x.1 {

            });
        });
}

#[derive(Debug)]
struct Board {
    tiles: HashMap<Location, TileType>,

    max_x: u32,
    max_y: u32,
}

impl Board {
    fn get_tile(&self, l: &Location) -> &TileType {
        self.tiles.get(l).unwrap_or(&TileType::Empty)
    }

    fn set_tile(&mut self, l: &Location, t: &TileType) {
        self.tiles.insert(l.clone(), *t.clone());
    }

    fn debug_print(&self) {
        use TileType::*;
        for x in 0..self.max_x {
            for y in 0..self.max_y {
                let symbol = match self.get_tile(&Location{ x, y }) {
                    Empty => " ",
                    UpDown => "|",
                    LeftRight => "-",
                    DiagonalForward => "/",
                    DiagonalBackward => r"\",

                };
                print!(symbol);
            }
            print("\n");
        }
    }
}

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
struct Location{
    x: u32,
    y: u32
}

#[derive(Debug)]
enum TileType {
    Empty,
    UpDown,
    LeftRight,
    DiagonalForward,
    DiagonalBackward
}


#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}