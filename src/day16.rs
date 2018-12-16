pub fn solve() {

}

fn parse_input() {

}

type Register = u32;
type Value = u32;

#[derive(Debug)]
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
    Eqrr
}

#[derive(Debug)]
struct Instruction {
    op: Opcode,
    a: u32,
    b: u32,
    c: u32
}


impl Instruction {
    fn apply(&self, regs: &mut Vec<i32>) {
        use self::Instruction::*;
        let &Instruction{ op, a, b, c } = self;
        let new_c_val = match op {
            Addr => regs[a] + regs[b],
            Addi => regs[a] + b,
            Mulr => regs[a] * regs[b],
            Muli => regs[a] * b
        };

        regs[c] = c_val;
    }
}
