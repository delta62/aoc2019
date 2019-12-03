enum Operation {
    Add(Location),
    Multiply(Location),
    Halt,
}

struct Location {
    pub a:   i32,
    pub b:   i32,
    pub out: usize,
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input
        .split(',')
        .map(|x| str::parse::<i32>(x).expect("Unable to parse integer from input"))
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &Vec<i32>) -> i32 {
    let mut input = input.clone();
    input[1] = 12;
    input[2] = 2;
    compute(&mut input);
    input[0]
}

fn compute(input: &mut Vec<i32>) {
    let mut pc = 0;
    loop {
        let next = op(pc, &input);
        match next {
            Operation::Halt => break,
            Operation::Add(Location { a, b, out }) => {
                input[out] = a + b;
                pc += 4;
            },
            Operation::Multiply(Location { a, b, out }) => {
                input[out] = a * b;
                pc += 4;
            },
        }
    }
}

fn op(pc: usize, input: &[i32]) -> Operation {
    if input[pc] == 99 {
        Operation::Halt
    } else {
        let a_addr = input[pc + 1] as usize;
        let b_addr = input[pc + 2] as usize;
        let out    = input[pc + 3] as usize;
        let a      = input[a_addr];
        let b      = input[b_addr];
        let loc    = Location { a, b, out };
        match input[pc] {
            1 => Operation::Add(loc),
            2 => Operation::Multiply(loc),
            x => panic!("Unknown op code {}", x),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1ex1() {
        let mut input = vec!(1,0,0,0,99);
        compute(&mut input);
        assert_eq!(input, vec![2,0,0,0,99]);
    }

    #[test]
    fn p1ex2() {
        let mut input = vec!(2,3,0,3,99);
        compute(&mut input);
        assert_eq!(input, vec![2,3,0,6,99]);
    }

    #[test]
    fn p1ex3() {
        let mut input = vec!(2,4,4,5,99,0);
        compute(&mut input);
        assert_eq!(input, vec![2,4,4,5,99,9801]);
    }

    #[test]
    fn p1ex4() {
        let mut input = vec!(1,1,1,4,99,5,6,0,99);
        compute(&mut input);
        assert_eq!(input, vec![30,1,1,4,2,5,6,0,99]);
    }
}
