use rayon::prelude::*;

pub fn solve() {
    let input = 2866;

    let answer = part_one(&input);
    println!("part_one={:?}", answer);

    let answer = part_two(&input);
    println!("part_two={:?}", answer);
}

fn part_one(input: &i32) -> FuelCell {
    let board = (0..=300)
        .map(|y| {
            (0..=300)
                .map(|x| FuelCell::new(x, y, *input))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let total_power = |x: i32, y: i32| -> i32 {
        if x >= 299 || y >= 299 {
            return 0;
        }

        let mut ret = 0;
        for x in x..=x + 2 {
            for y in y..=y + 2 {
                ret += board[y as usize][x as usize].power_level;
            }
        }
        ret
    };

    board
        .iter()
        .flatten()
        .max_by_key(|fc| total_power(fc.x, fc.y))
        .unwrap()
        .clone()
}

// TODO
fn part_two(input: &i32) -> (i32, i32, i32) {
    let board = (0..=300)
        .map(|y| {
            (0..=300)
                .map(|x| FuelCell::new(x, y, *input))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let total_power = |x: i32, y: i32, size: i32| -> i32 {
        if size == 0 {
            return 0;
        }
        if x > 300 - size || y > 300 - size {
            return 0;
        }

        let mut ret = 0;
        for y in y..=y + (size - 1) {
            for x in x..=x + (size - 1) {
                ret += board[y as usize][x as usize].power_level;
            }
        }
        ret
    };

    // you can solve this with dynamic programming...
    // but I'm brute forcing the shit out of it.
    let largest_square = (0..300i32)
        .into_par_iter()
        .map(|size| {
            board
                .iter()
                .flatten()
                .max_by_key(|fc| total_power(fc.x, fc.y, size))
                .unwrap()
                .clone()
        })
        .enumerate()
        .max_by_key(|pair| total_power(pair.1.x, pair.1.y, pair.0 as i32))
        .unwrap();


    (largest_square.1.x, largest_square.1.y, largest_square.0 as i32)
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
struct FuelCell {
    x: i32,
    y: i32,
    power_level: i32,
}

impl FuelCell {
    fn new(x: i32, y: i32, serial_num: i32) -> Self {
        let mut ret = FuelCell {
            x,
            y,
            power_level: 0,
        };
        ret.set_serial_num(serial_num);
        ret
    }

    fn rack_id(&self) -> i32 {
        self.x + 10
    }

    fn set_serial_num(&mut self, serial: i32) -> i32 {
        let mut power_level = self.rack_id() * self.y;
        power_level += serial;
        power_level *= self.rack_id();

        //take the 100s digit
        let as_str: Vec<char> = power_level.to_string().chars().collect();
        let hundreds_digit = as_str.get(as_str.len() - 3).unwrap_or(&'0');
        power_level = hundreds_digit.to_string().parse().unwrap();

        power_level -= 5;
        self.power_level = power_level;
        power_level
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn power_level_examples() {
        // fuel_cell, grid_serial_num, expected_power_level
        let examples = [
            ((122, 79), 57, -5),
            ((217, 196), 39, 0),
            ((101, 153), 71, 4),
        ];

        for example in examples.iter() {
            let (point, grid_serial_num, expected_power_level) = example;
            let point = FuelCell::new(point.0, point.1, *grid_serial_num);
            let power_level = point.power_level;
            assert_eq!(power_level, *expected_power_level);
        }
    }

    #[test]
    fn part_two_example() {
        let input = 18;
        let part_two = part_two(&input);
        assert_eq!(part_two,(90,269,16));
    }
}
