#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input
        .split(',')
        .map(|x| x.parse::<i32>().expect("Invalid input"))
        .collect::<Vec<_>>()
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &Vec<i32>) -> i32 {
    let stdin  = vec![ 1 ];
    let stdout = vec![];
    let mut prog = Program::new(input.clone(), stdin, stdout);
    prog.run();
    *prog.stdout().get(0).expect("No output value")
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &Vec<i32>) -> i32 {
    let stdin = vec![ 5 ];
    let stdout = vec![];
    let mut prog = Program::new(input.clone(), stdin, stdout);
    prog.run();
    *prog.stdout().get(0).expect("No output value")
}

struct Program {
    mem: Vec<i32>,
    pc: usize,
    stdout: Vec<i32>,
    stdin: Vec<i32>,
}

impl Program {
    fn new(mem: Vec<i32>, stdin: Vec<i32>, stdout: Vec<i32>) -> Program {
        let pc = 0;
        Program { mem, pc, stdout, stdin, }
    }

    fn run(&mut self) {
        loop {
            match self.next_op() {
                Op::Halt => break,
                Op::Add(x, y, dst) => {
                    let x = self.read(x);
                    let y = self.read(y);
                    self.write(dst, x + y);
                },
                Op::Multiply(x, y, dst) => {
                    let x = self.read(x);
                    let y = self.read(y);
                    self.write(dst, x * y);
                },
                Op::Input(dst) => {
                    let input = self.stdin.remove(0);
                    self.write(dst, input);
                },
                Op::Output(x) => {
                    let x = self.read(x);
                    self.stdout.insert(0, x);
                },
                Op::JumpIfTrue(x, dst) => {
                    let x = self.read(x);
                    if x != 0 {
                        self.pc = self.read(dst) as usize;
                    }
                },
                Op::JumpIfFalse(x, dst) => {
                    let x = self.read(x);
                    if x == 0 {
                        self.pc = self.read(dst) as usize;
                    }
                },
                Op::LessThan(x, y, d) => {
                    let x = self.read(x);
                    let y = self.read(y);
                    if x < y {
                        self.write(d, 1);
                    } else {
                        self.write(d, 0);
                    }
                },
                Op::Equals(x, y, d) => {
                    let x = self.read(x);
                    let y = self.read(y);
                    if x == y {
                        self.write(d, 1);
                    } else {
                        self.write(d, 0);
                    }
                },
            }
        }
    }

    fn read(&self, param: Param) -> i32 {
        match param.mode {
            ParamMode::Position  => self.mem[param.data as usize],
            ParamMode::Immediate => param.data,
        }
    }

    fn write(&mut self, dest: Param, val: i32) {
        self.mem[dest.data as usize] = val;
    }

    fn next_op(&mut self) -> Op {
        let opstr = format!("{:0>6}", self.mem[self.pc]);
        let (flags, op) = opstr.split_at(opstr.len() - 2);
        let flags = flags.chars().rev().collect::<Vec<_>>();

        match op {
            "01" => {
                let x = Param::new(flags.get(0), self.mem[self.pc + 1]);
                let y = Param::new(flags.get(1), self.mem[self.pc + 2]);
                let d = Param::new(flags.get(2), self.mem[self.pc + 3]);
                self.pc += 4;
                Op::Add(x, y, d)
            },
            "02" => {
                let x = Param::new(flags.get(0), self.mem[self.pc + 1]);
                let y = Param::new(flags.get(1), self.mem[self.pc + 2]);
                let d = Param::new(flags.get(2), self.mem[self.pc + 3]);
                self.pc += 4;
                Op::Multiply(x, y, d)
            },
            "03" => {
                let d = Param::new(flags.get(0), self.mem[self.pc + 1]);
                self.pc += 2;
                Op::Input(d)
            },
            "04" => {
                let x = Param::new(flags.get(0), self.mem[self.pc + 1]);
                self.pc += 2;
                Op::Output(x)
            },
            "05" => {
                let x = Param::new(flags.get(0), self.mem[self.pc + 1]);
                let d = Param::new(flags.get(1), self.mem[self.pc + 2]);
                self.pc += 3;
                Op::JumpIfTrue(x, d)
            },
            "06" => {
                let x = Param::new(flags.get(0), self.mem[self.pc + 1]);
                let d = Param::new(flags.get(1), self.mem[self.pc + 2]);
                self.pc += 3;
                Op::JumpIfFalse(x, d)
            },
            "07" => {
                let x = Param::new(flags.get(0), self.mem[self.pc + 1]);
                let y = Param::new(flags.get(1), self.mem[self.pc + 2]);
                let d = Param::new(flags.get(2), self.mem[self.pc + 3]);
                self.pc += 4;
                Op::LessThan(x, y, d)
            },
            "08" => {
                let x = Param::new(flags.get(0), self.mem[self.pc + 1]);
                let y = Param::new(flags.get(1), self.mem[self.pc + 2]);
                let d = Param::new(flags.get(2), self.mem[self.pc + 3]);
                self.pc += 4;
                Op::Equals(x, y, d)
            },
            "99" => {
                self.pc += 1;
                Op::Halt
            }
            x => panic!("Unsupported opcode {}", x),
        }
    }

    fn stdout(&self) -> &[i32] {
        self.stdout.as_ref()
    }
}

#[derive(Debug)]
enum ParamMode {
    Position,
    Immediate,
}

#[derive(Debug)]
struct Param {
    pub mode: ParamMode,
    pub data: i32,
}

impl Param {
    fn new(mode: Option<&char>, data: i32) -> Param {
        let mode = mode.unwrap_or(&'0');
        let mode = match mode {
            '0' => ParamMode::Position,
            '1' => ParamMode::Immediate,
            x   => panic!("Unsupported parameter mode {}", x),
        };
        Param { mode, data }
    }
}

#[derive(Debug)]
enum Op {
    Add(Param, Param, Param),
    Multiply(Param, Param, Param),
    Input(Param),
    Output(Param),
    JumpIfTrue(Param, Param),
    JumpIfFalse(Param, Param),
    LessThan(Param, Param, Param),
    Equals(Param, Param, Param),
    Halt,
}

#[cfg(test)]
mod tests {
    use super::*;

}
