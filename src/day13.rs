use std::collections::HashMap;

pub fn solve() {
    let input = include_str!("../input/day13");
    let input = parse_input(&input);

    input.debug_print();

}


fn parse_input(input :&str) -> Board {
    let mut board = Board::default();
    input
        .lines()
        .enumerate()
        .for_each(|l| {
            let y = l.0 as u32;
            l.1.chars().enumerate().for_each(|pair| {
                let x = pair.0 as u32;
                let tile_type = match pair.1 {
                    '/' => TileType::DiagonalForward,
                    '\\' => TileType::DiagonalBackward,
                    '|' => TileType::UpDown,
                    '-' => TileType::LeftRight,
                    _ => TileType::Empty
                };
                board.set_tile(&Location{x,y}, tile_type);
            });
        });
    board
}

#[derive(Debug, Default)]
struct Board {
    tiles: HashMap<Location, TileType>,

    max_x: u32,
    max_y: u32,
}

impl Board {
    fn get_tile(&self, l: &Location) -> &TileType {
        self.tiles.get(l).unwrap_or(&TileType::Empty)
    }

    fn set_tile(&mut self, l: &Location, t: TileType) {
        if l.x > self.max_x{
            self.max_x = l.x;
        }
        if l.y > self.max_y {
            self.max_y = l.y
        }
        self.tiles.insert(l.clone(), t);
    }

    fn debug_print(&self) {
        for y in 0..=self.max_y {
            for x in 0..=self.max_x {
                let symbol = match self.get_tile(&Location{ x, y }) {
                    TileType::Empty => " ",
                    TileType::UpDown => "|",
                    TileType::LeftRight => "-",
                    TileType::DiagonalForward => "/",
                    TileType::DiagonalBackward => r"\",

                };
                print!("{}", symbol);
            }
            print!("\n");
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