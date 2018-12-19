use lazy_static::*;
use itertools::Itertools;

pub fn solve() {
    let input = include_str!("../input/day19");
    let mut input = parse_input(input);

    let answer = part_one(&mut input);
    println!("part_one= {}", answer)
}

fn parse_input(input: &str) -> Program {

    use self::Opcode::*;
    let instructions = input
        .lines()
        .skip(1)
        .map(|l| {
            let mut split = l.split_whitespace();
            let op = match split.next() {
                Some("addr") => Addr,
                Some("addi") => Addi,
                Some("mulr") => Mulr,
                Some("muli") => Muli,
                Some("banr") => Banr,
                Some("bani") => Bani,
                Some("borr") => Borr,
                Some("bori") => Bori,
                Some("setr") => Setr,
                Some("seti") => Seti,
                Some("gtir") => Gtir,
                Some("gtri") => Gtri,
                Some("gtrr") => Gtrr,
                Some("eqir") => Eqir,
                Some("eqri") => Eqri,
                Some("eqrr") => Eqrr,
                other => panic!(format!("bad input: {}", l))
            };

            let a = split.next().unwrap().parse().unwrap();
            let b = split.next().unwrap().parse().unwrap();
            let c = split.next().unwrap().parse().unwrap();
            Instruction { op, a,b,c }

        })
        .collect_vec();

   Program{
       instruction_register: 2, //TODO: parse me?
       registers: vec![0;6],
       instructions
   }
}

fn part_one(input :&mut Program) -> i64 {
    input.run_to_halt();
    input.registers[0]
}

struct Program {
    instruction_register: usize,
    registers: Vec<i64>,
    instructions: Vec<Instruction>
}

impl Program {
    fn get_instruction_pointer_value(&self) -> usize {
        self.registers[self.instruction_register] as usize
    }

    fn get_current_instruction(&self) -> Option<&Instruction> {
        self.instructions.get(self.get_instruction_pointer_value())
    }

    fn inc_instruction_pointer(&mut self) {
        self.registers[self.instruction_register] += 1;
    }

    fn run_to_halt(&mut self) {
        while let Some(ins) = self.get_current_instruction() {
            let new_registers = ins.apply(&self.registers);
            if new_registers.is_none() {
                panic!("That's fucked up");
            }
            self.registers = new_registers.unwrap();
            self.inc_instruction_pointer();
        }
    }
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