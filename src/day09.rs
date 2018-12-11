use itertools::*;
use regex::Regex;
use std::collections::HashMap;

pub fn solve() {
    let input = include_str!("../input/day09");
    let input = parse_input(input);

    let mut game = Game::with_rules(input.n_players, input.n_points);
    let answer = part_one(&mut game);
    println!("part_one={:?}", answer);

    let mut game = Game::with_rules(input.n_players, input.n_points * 100);
    let answer = part_one(&mut game);
    println!("part_two={:?}", answer);
}

struct Input {
    n_players: u32,
    n_points: u32,
}

fn parse_input(input: &str) -> Input {
    let re = Regex::new(r#"\d+"#).unwrap();
    let mut caps = re.captures_iter(input);
    let n_players = caps.next().unwrap()[0].parse().unwrap();
    let n_points = caps.next().unwrap()[0].parse().unwrap();
    Input {
        n_players,
        n_points,
    }
}

fn part_one(game: &mut Game) -> u32 {
    game.play_to_end();
    game.player_scores
        .values()
        .filter_map(|&v| Some(v))
        .max()
        .unwrap()
}

#[derive(Debug)]
struct Game {
    pub cur_marble_idx: usize,
    pub cur_board: Vec<u32>,
    pub player_scores: HashMap<u32, u32>, //player_num --> score

    pub next_player: u32,
    pub next_player_marble: u32,

    //rules
    pub n_players: u32,
    pub n_points: u32,
}

impl Game {
    pub fn with_rules(n_players: u32, n_points: u32) -> Game {
        Game {
            cur_marble_idx: 0,
            cur_board: vec![0],
            player_scores: HashMap::new(),

            next_player: 1,
            next_player_marble: 1,

            n_players,
            n_points,
        }
    }

    pub fn play_turn(&mut self) {
        match self.next_player_marble {
            x if x % 23 == 0 => {
                //TODO: gross?
                let mut player_score = self
                    .player_scores
                    .get(&self.next_player)
                    .unwrap_or(&0)
                    .to_owned();

                player_score += self.next_player_marble;
                let mut remove_idx: i32 = self.cur_marble_idx as i32 - 7;
                if remove_idx <= 0 {
                    remove_idx += self.cur_board.len() as i32;
                }

                //remove and add to score
                player_score += self.cur_board.remove(remove_idx as usize);
                self.player_scores.insert(self.next_player, player_score);
                self.cur_marble_idx = remove_idx as usize;
           }
            _ => {
                let mut next_index: i32 = self.cur_marble_idx as i32 + 2;
                if next_index > self.cur_board.len() as i32 {
                    next_index -= self.cur_board.len() as i32;
                }

                //normal turn
                self.cur_board
                    .insert(next_index as usize, self.next_player_marble);
                self.cur_marble_idx = next_index as usize;
            }
        }

        self.next_player = (self.next_player + 1) % self.n_players;
        self.next_player_marble += 1;
    }

    pub fn play_to_end(&mut self) {
        while self.next_player_marble <= self.n_points {
            self.play_turn()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_example() {
        let mut game = Game::with_rules(9, 25);
        for _ in 0..26 {
            game.play_turn();
            print!("{:5}", format!("[{}]", game.next_player));
            for (i, item) in game.cur_board.iter().enumerate() {
                let text = if game.cur_marble_idx == i {
                    format!("({})", item)
                } else {
                    format!("{}", item)
                };

                print!("{:5}", text);
            }
            print!("\n");
        }
    }

    #[test]
    fn test_part_one_examples() {
        let test_cases = vec![
            (
                Input {
                    n_players: 10,
                    n_points: 1618,
                },
                8317,
            ),
            (
                Input {
                    n_players: 13,
                    n_points: 7999,
                },
                146373,
            ),
            (
                Input {
                    n_players: 17,
                    n_points: 1104,
                },
                2764,
            ),
            (
                Input {
                    n_players: 21,
                    n_points: 6111,
                },
                54718,
            ),
            (
                Input {
                    n_players: 30,
                    n_points: 5807,
                },
                37305,
            ),
        ];

        test_cases.iter().for_each(|test_case| {
            let mut game = Game::with_rules(test_case.0.n_players, test_case.0.n_points);
            let mut answer = part_one(&mut game);
            assert_eq!(answer, test_case.1);
        });
    }
}
