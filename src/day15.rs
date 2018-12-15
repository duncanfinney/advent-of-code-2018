use itertools::*;
pub fn solve() {
    let input = include_str!("../input/day15");
    let input = parse_input(&input);

    input.debug_print();
}

fn parse_input(input: &str) -> Board {
    let mut characters = vec![];

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
                        characters.push(Character::Elf { x, y });
                        Tile::OpenCavern
                    }
                    'G' => {
                        characters.push(Character::Goblin { x, y });
                        Tile::OpenCavern
                    }
                    _ => panic!("bad puzzle input"),
                })
                .collect::<Vec<Tile>>()
        })
        .collect_vec();

    Board { tiles, characters }
}

#[derive(Debug)]
struct Board {
    tiles: Vec<Vec<Tile>>,
    characters: Vec<Character>,
}

#[derive(Debug)]
enum Tile {
    Wall,
    OpenCavern,
}

#[derive(Debug)]
enum Character {
    Goblin { x: usize, y: usize },
    Elf { x: usize, y: usize },
}

impl Board {
    fn debug_print(&self) {
        for (y, r) in self.tiles.iter().enumerate() {
            for (x, c) in r.iter().enumerate() {
                let rep = match self.characters.iter().find(|c| c.is_on_tile(x, y)) {
                    Some(Character::Elf { .. }) => "E",
                    Some(Character::Goblin { .. }) => "G",
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

impl Character {
    fn is_on_tile(&self, cx: usize, cy: usize) -> bool {
        use self::Character::*;
        match self {
            &Goblin { x, y } => cx == x && cy == y,
            &Elf { x, y } => cx == x && cy == y,
        }
    }
}
