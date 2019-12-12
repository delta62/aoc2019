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
    // let mut phases = [ 0, 1, 2, 3, 4 ];
    // let inputs = Heap::new(&mut phases);

    Heap::new(&mut [ 0, 1, 2, 3, 4, ])
        .map(|inputs| {
            inputs
                .iter()
                .fold(0, |acc, phase| {
                    let mem = code.clone();
                    let mut prog = Program::new(mem, vec![ *phase, acc ], vec![]);
                    prog.run();
                    *prog.stdout().get(0).expect("No output produced in amplifier")
                })
        })
        .max()
        .expect("No output produced from pipeline")
}
