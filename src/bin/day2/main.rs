use advent::day2::Program;
use std::process::exit;

#[path = "../../day2/mod.rs"]
mod day2;

fn main() {
    let mut memory = vec![1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,9,1,19,1,19,6,23,2,6,23,27,2,27,9,31,1,5,31,35,1,35,10,39,2,39,9,43,1,5,43,47,2,47,10,51,1,51,6,55,1,5,55,59,2,6,59,63,2,63,6,67,1,5,67,71,1,71,9,75,2,75,10,79,1,79,5,83,1,10,83,87,1,5,87,91,2,13,91,95,1,95,10,99,2,99,13,103,1,103,5,107,1,107,13,111,2,111,9,115,1,6,115,119,2,119,6,123,1,123,6,127,1,127,9,131,1,6,131,135,1,135,2,139,1,139,10,0,99,2,0,14,0];

    let mut program = Program::new(to_1202_program_alarm(&memory));
    program.run();

    // prints 6730673
    println!("Solution to part 1 is {}", program.read_memory(0));

    'outer: for noun in 0..99 {
        for verb in 0..99 {
            let input_memory = set_inputs(&memory, noun, verb);
            program = Program::new(input_memory);
            program.run();
            if program.read_memory(0) == 19690720 {
                let res = 100 * noun + verb;
                // prints 3749
                println!("Solution to part 2 is {}", res);
                break 'outer;
            }
        }
    }
}

fn to_1202_program_alarm(memory: &Vec<i32>) -> Vec<i32> {
    set_inputs(memory, 12, 2)
}

fn set_inputs(memory: &Vec<i32>, noun: i32, verb: i32) -> Vec<i32> {
    let mut updated_memory = memory.clone();
    updated_memory[1] = noun;
    updated_memory[2] = verb;
    updated_memory
}
