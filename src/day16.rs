use itertools::Itertools;
use lazy_static::*;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn solve() {
    let input = include_str!("../input/day16");
    let mut input = parse_input(input);

    let answer = part_one(&input);
    println!("part_one= {:?}", answer);

    let answer = part_two(&mut input);
    println!("part_two={:?}", answer);
}

fn parse_input(input: &str) -> Vec<TestCase> {
    let mut ret = vec![];
    let vec_from_str = |s: &str| -> Vec<i64> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\d+").unwrap();
        }
        RE.captures_iter(s)
            .filter_map(|c| c.get(0).unwrap().as_str().parse().ok())
            .collect::<Vec<_>>()
    };

    let lines = input.lines().collect::<Vec<_>>();
    for window in lines.chunks(4) {
        if !window[0].starts_with("Before") {
            break;
        }
        let before = vec_from_str(window[0]);
        let after = vec_from_str(window[2]);
        if let [opcode_val, a, b, c] = vec_from_str(window[1])[0..4] {
            ret.push(TestCase {
                before,
                after,
                opcode_val,
                a,
                b,
                c,
            });
        }
    }
    ret
}

fn part_one(input: &Vec<TestCase>) -> i64 {
    let mut count = 0;
    for (n, tc) in input.iter().enumerate() {
        let valid_codes = OPCODES
            .iter()
            .map(|op| Instruction {
                op: op.clone(),
                a: tc.a,
                b: tc.b,
                c: tc.c,
            })
            .filter(|ins| {
                let output = ins.apply(&tc.before);
                if let Some(output) = output {
                    tc.after.eq(&output)
                } else {
                    false
                }
            })
            .collect_vec();
        if valid_codes.is_empty() {
            println!("[{}] not valid for shit: {:?}", n, tc);
        }

        if valid_codes.len() >= 3 {
            count += 1;
        }
    }

    count
}

fn part_two(input: &mut Vec<TestCase>) {
    let mut figured_out = HashSet::new();
    let mut number_to_opcode = HashMap::new();

    for _ in 0..16 {
        for tc in input.iter() {
            if figured_out.contains(&tc.opcode_val) {
                continue;
            }

            let valid_codes = OPCODES
                .iter()
                .filter(|op| number_to_opcode.values().find(|op2| op2 == op).is_none())
                .map(|op| Instruction {
                    op: op.clone(),
                    a: tc.a,
                    b: tc.b,
                    c: tc.c,
                })
                .filter(|ins| {
                    let output = ins.apply(&tc.before);
                    if let Some(output) = output {
                        tc.after.eq(&output)
                    } else {
                        false
                    }
                })
                .collect_vec();
            if valid_codes.len() == 1 {
                figured_out.insert(tc.opcode_val);
                number_to_opcode.insert(tc.opcode_val, valid_codes[0].op);
            }
        }
    }
}

#[derive(Debug)]
struct TestCase {
    before: Vec<i64>,
    opcode_val: i64,
    a: i64,
    b: i64,
    c: i64,
    after: Vec<i64>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Opcode {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

static OPCODES: [Opcode; 16] = [
    Opcode::Addr,
    Opcode::Addi,
    Opcode::Mulr,
    Opcode::Muli,
    Opcode::Banr,
    Opcode::Bani,
    Opcode::Borr,
    Opcode::Bori,
    Opcode::Setr,
    Opcode::Seti,
    Opcode::Gtir,
    Opcode::Gtri,
    Opcode::Gtrr,
    Opcode::Eqir,
    Opcode::Eqri,
    Opcode::Eqrr,
];

#[derive(Debug)]
struct Instruction {
    op: Opcode,
    a: i64,
    b: i64,
    c: i64,
}

impl Instruction {
    fn apply(&self, regs: &Vec<i64>) -> Option<Vec<i64>> {
        use self::Opcode::*;
        let Instruction { op, a, b, c } = self;
        let (a, b, c) = (*a, *b, *c);

        lazy_static! {
            static ref NEEDS_REG_A: Vec<Opcode> =
                vec![Addr, Addi, Mulr, Muli, Banr, Bani, Borr, Bori, Setr, Gtri, Gtrr, Eqri, Eqrr];
            static ref NEEDS_REG_B: Vec<Opcode> =
                vec![Addr, Mulr, Banr, Borr, Gtir, Gtrr, Eqir, Eqrr];
        }

        if NEEDS_REG_A.contains(op) && regs.get(a as usize).is_none() {
            return None;
        }

        if NEEDS_REG_B.contains(op) && regs.get(b as usize).is_none() {
            return None;
        }

        let new_c_val = match op {
            Addr => regs[a as usize] + regs[b as usize],
            Addi => regs[a as usize] + b,

            Mulr => regs[a as usize] * regs[b as usize],
            Muli => regs[a as usize] * b,

            Banr => regs[a as usize] & regs[b as usize],
            Bani => regs[a as usize] & b,

            Borr => regs[a as usize] | regs[b as usize],
            Bori => regs[a as usize] | b,

            Setr => regs[a as usize],
            Seti => a,

            Gtir => {
                if a > regs[b as usize] {
                    1
                } else {
                    0
                }
            }
            Gtri => {
                if regs[a as usize] > b {
                    1
                } else {
                    0
                }
            }
            Gtrr => {
                if regs[a as usize] > regs[b as usize] {
                    1
                } else {
                    0
                }
            }

            Eqir => {
                if a == regs[b as usize] {
                    1
                } else {
                    0
                }
            }
            Eqri => {
                if regs[a as usize] == b {
                    1
                } else {
                    0
                }
            }
            Eqrr => {
                if regs[a as usize] == regs[b as usize] {
                    1
                } else {
                    0
                }
            }
        };

        let mut ret = regs.clone();
        ret[c as usize] = new_c_val;
        Some(ret)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //    #[test]
    fn test_bad_guesses() {
        let input = include_str!("../input/day16");
        let input = parse_input(input);

        let answer = part_one(&input);
        assert_ne!(answer, 507);
    }

    #[test]
    fn test_part_one_testcases() {
        let input = parse_input(
            r"Before: [3, 2, 1, 1]
9 2 1 2
After:  [3, 2, 2, 1]
",
        );
        let answer = part_one(&input);

        assert_eq!(answer, 3);
    }
}
