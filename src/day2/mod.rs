use std::collections::HashMap;

struct Memory {
    mem: Vec<i32>
}

impl Memory {
    fn get(&self, index: usize) -> i32 {
        self.mem[index]
    }
    fn set(&mut self, index: usize, value: i32) {
        self.mem[index] = value;
    }
}

pub struct Program {
    memory : Memory,
    instruction_pointer: usize,
    finished: bool
}

trait Operation {
    fn opcode(&self) -> i32;

    fn perform(&self, program: &mut Program);
}

fn bi_operation_helper(program: &mut Program, operate: impl Fn(&i32, &i32) -> i32) {
    let x_index = program.memory.get(program.instruction_pointer + 1) as usize;
    let x = program.memory.get(x_index);
    let y_index = program.memory.get(program.instruction_pointer + 2) as usize;
    let y = program.memory.get(y_index);
    let res_index = program.memory.get(program.instruction_pointer + 3) as usize;
    let res = operate(&x, &y);
    program.memory.set(res_index, res);
    program.instruction_pointer += 4;
}

struct AddOperation {}

impl AddOperation {
    fn operate(x: &i32, y: &i32) -> i32 {
        let res = x + y;
        res
    }
}

impl Operation for AddOperation {

    fn opcode(&self) -> i32 {
        1
    }

    fn perform(&self, program: &mut Program) {
        bi_operation_helper(program, AddOperation::operate);
    }
}


struct MultiplyOperation {}
impl MultiplyOperation {
    fn operate(x: &i32, y: &i32) -> i32 {
        x * y
    }
}

impl Operation for MultiplyOperation {
    fn opcode(&self) -> i32 {
        2
    }

    fn perform(&self, program: &mut Program) {
        bi_operation_helper(program, MultiplyOperation::operate);
    }
}

struct FinishedOperation {}
impl Operation for FinishedOperation {
    fn opcode(&self) -> i32 {
        99
    }

    fn perform(&self, program: &mut Program) {
        program.finished = true;
        program.instruction_pointer += 1;
    }
}

fn operations() -> HashMap<i32, Box<dyn Operation>> {
    let add_operation = AddOperation {};
    let multiply_operation = MultiplyOperation {};
    let finished_operation = FinishedOperation {};
    let mut operations: HashMap<i32, Box<dyn Operation>> = HashMap::new();
    operations.insert(add_operation.opcode(), Box::from(add_operation));
    operations.insert(multiply_operation.opcode(), Box::from(multiply_operation));
    operations.insert(finished_operation.opcode(), Box::from(finished_operation));
    operations
}

impl Program {

    pub fn new(memory: Vec<i32>) -> Program {
        Program {
            memory: Memory {mem: memory},
            instruction_pointer: 0,
            finished: false
        }
    }

    pub fn run(&mut self) {
        let operations = operations();
        while !self.finished {
            let opcode = self.memory.get(self.instruction_pointer);
            let operation = operations.get(&opcode);

            match operation {
                Some(operation) => operation.perform(self),
                None => panic!("unknown opcode {} at index {}", opcode, self.instruction_pointer),
            }
        }
    }

    pub fn read_memory(&self, address: usize) -> i32 {
        self.memory.get(address)
    }
}


#[cfg(test)]
mod tests {

    #[test]
    fn test1() {
        let memory = vec![1,9,10,3,2,3,11,0,99,30,40,50];
        let mut program = super::Program::new(memory);
        program.run();
        assert_eq!(program.read_memory(0), 3500);
    }

    #[test]
    fn test2() {
        let memory = vec![1,0,0,0,99];
        let mut program = super::Program::new(memory);
        program.run();
        assert_eq!(program.read_memory(0), 2);
    }

    #[test]
    fn test3() {
        let memory = vec![2,3,0,3,99];
        let mut program = super::Program::new(memory);
        program.run();
        assert_eq!(program.read_memory(3), 6);
    }

    #[test]
    fn test4() {
        let memory = vec![2,4,4,5,99,0];
        let mut program = super::Program::new(memory);
        program.run();
        assert_eq!(program.read_memory(5), 9801);
    }

    #[test]
    fn test5() {
        let memory = vec![1,1,1,4,99,5,6,0,99];
        let mut program = super::Program::new(memory);
        program.run();
        assert_eq!(program.read_memory(0), 30);
    }
}


