pub fn solve() {
    let input = include_str!("../input/day19");
}

fn parse_input() -> Program {
   Input{}
}

fn part_one(input :&mut Program) {

}

struct Program {
    instruction_register: usize, //todo parse me?
    registers: Vec<u32>,
    instructions: Vec<Instruction>
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