mod program;

use permutohedron::Heap;
use program::Program;

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input.split(',')
        .map(|x| x.parse::<i32>().expect("Non-integer input given"))
        .collect::<Vec<_>>()
}

#[aoc(day7, part1)]
pub fn solve_part1(code: &Vec<i32>) -> i32 {
    let mut phases = [ 0, 1, 2, 3, 4 ];
    let inputs = Heap::new(&mut phases);

    inputs
        .map(|inputs| {
            let mut amplifiers = vec![
                Amplifier::new(), // Amp A
                Amplifier::new(), // Amp B
                Amplifier::new(), // Amp C
                Amplifier::new(), // Amp D
                Amplifier::new(), // Amp E
            ];

            amplifiers
                .iter_mut()
                .zip(&inputs)
                .fold(0, |acc, (amp, phase)| amp.run(code, *phase, acc))
        })
        .max()
        .expect("No output produced from pipeline")
}

struct Amplifier { }

impl Amplifier {
    fn new() -> Amplifier {
        Amplifier { }
    }

    fn run(&mut self, mem: &Vec<i32>, phase: i32, input: i32) -> i32 {
        let mem = mem.clone();
        let mut prog = Program::new(mem, vec![ phase, input ], vec![]);
        prog.run();
        *prog.stdout().get(0).expect("No output produced in amplifier")
    }
}
