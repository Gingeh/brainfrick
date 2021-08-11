use std::collections::VecDeque;

pub struct BrainFuck {
    memory: Vec<Cell>,
    pointer: usize,
    input: VecDeque<u8>,
    counter: usize,
    max_steps: usize,
}

impl BrainFuck {
    #[must_use]
    pub fn new(size: usize, input: &str, max_steps: usize) -> Self {
        BrainFuck {
            memory: vec![Cell::from(0); size],
            pointer: 0,
            counter: 0,
            input: queue_from(input),
            max_steps,
        }
    }
    pub fn run(&mut self, program: &str) -> Result<String, String> {
        let mut output = String::new();
        let program: Vec<char> = program.chars().collect();
        let mut steps: usize = 0;

        while self.counter < program.len() {
            let ch = program[self.counter];
            match ch {
                '+' => self.add(),
                '-' => self.sub(),
                '<' => self.left(),
                '>' => self.right(),
                '.' => output += &self.print(),
                ',' => self.read(),
                '[' => self.start(&program)?,
                ']' => self.end(&program)?,
                _ => {}
            }
            self.counter += 1;
            if steps == self.max_steps {
                return Err(format!(
                    "Exceeded maximum steps ({}), the program may be stuck in a loop.",
                    self.max_steps
                ));
            }
            steps += 1;
        }

        Ok(output)
    }
    fn add(&mut self) {
        self.memory[self.pointer].add();
    }
    fn sub(&mut self) {
        self.memory[self.pointer].sub();
    }
    fn left(&mut self) {
        self.pointer = if self.pointer == 0 {
            self.memory.len() - 1
        } else {
            self.pointer - 1
        }
    }
    fn right(&mut self) {
        self.pointer = if self.pointer == self.memory.len() - 1 {
            0
        } else {
            self.pointer + 1
        }
    }
    fn print(&mut self) -> String {
        self.memory[self.pointer].to_string()
    }
    fn read(&mut self) {
        self.memory[self.pointer] = Cell::from(self.input.pop_front().unwrap_or(0));
    }
    fn start(&mut self, program: &[char]) -> Result<(), String> {
        if self.memory[self.pointer] == Cell::from(0) {
            let mut level: usize = 1;
            while level > 0 {
                self.counter += 1;
                if self.counter == program.len() {
                    return Err(String::from("A matching \"]\" could not be found."));
                }
                if program[self.counter] == '[' {
                    level += 1;
                } else if program[self.counter] == ']' {
                    level -= 1;
                }
            }
        }
        Ok(())
    }
    fn end(&mut self, program: &[char]) -> Result<(), String> {
        if self.memory[self.pointer] != Cell::from(0) {
            let mut level: usize = 1;
            while level > 0 {
                if self.counter == 0 {
                    return Err(String::from("A matching \"[\" could not be found."));
                }
                self.counter -= 1;
                if program[self.counter] == '[' {
                    level -= 1;
                } else if program[self.counter] == ']' {
                    level += 1;
                }
            }
        }
        Ok(())
    }
}

#[derive(PartialEq, Clone, Copy)]
struct Cell {
    value: u8,
}

impl Cell {
    fn add(&mut self) {
        self.value = if self.value == 255 { 0 } else { self.value + 1 }
    }
    fn sub(&mut self) {
        self.value = if self.value == 0 { 255 } else { self.value - 1 }
    }
}

impl ToString for Cell {
    fn to_string(&self) -> String {
        String::from(self.value as char)
    }
}

impl From<u8> for Cell {
    fn from(value: u8) -> Self {
        Self { value }
    }
}

fn queue_from(input: &str) -> VecDeque<u8> {
    VecDeque::from(input.as_bytes().to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello_world() {
        let program = "--------[>+>+++++>-->-->++++>------<<<<<<-------]>.>---.>----..>-.>.>+++++++.<<.+++.<.<-.>>>+.";
        let mut engine = BrainFuck::new(256, "", 10000);
        assert_eq!(engine.run(program), Ok(String::from("Hello World!")));
    }

    #[test]
    fn test_input() {
        let program = ",++.,-.";
        let mut engine = BrainFuck::new(256, "a", 10000);
        assert_eq!(engine.run(program), Ok(String::from("cÿ")));
    }
    #[test]
    fn test_wraparound() {
        let program = "->>.";
        let mut engine = BrainFuck::new(2, "", 10000);
        assert_eq!(engine.run(program), Ok(String::from("ÿ")));
    }
}
