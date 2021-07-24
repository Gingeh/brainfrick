use std::collections::VecDeque;

pub struct BrainFuck {
    memory: Vec<Cell>,
    pointer: usize,
    input: VecDeque<u8>,
    counter: usize,
}

impl BrainFuck {
    #[must_use]
    pub fn new(size: usize, input: &str) -> Self {
        BrainFuck {
            memory: vec![Cell::from(0); size],
            pointer: 0,
            counter: 0,
            input: queue_from(input),
        }
    }
    pub fn run(&mut self, program: &str) -> String {
        let mut output = String::new();
        let program: Vec<char> = program.chars().collect();

        while self.counter < program.len() {
            let ch = program[self.counter];
            match ch {
                '+' => self.add(),
                '-' => self.sub(),
                '<' => self.left(),
                '>' => self.right(),
                '.' => output += &self.print(),
                ',' => self.read(),
                '[' => self.start(&program),
                ']' => self.end(&program),
                _ => {}
            }
            self.counter += 1;
        }

        output
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
        self.memory[self.pointer] = Cell::from(self.input.pop_front().unwrap_or(0))
    }
    fn start(&mut self, program: &[char]) {
        if self.memory[self.pointer] == Cell::from(0) {
            let mut level: usize = 1;
            while level > 0 {
                self.counter += 1;
                if program[self.counter] == '[' {
                    level += 1;
                } else if program[self.counter] == ']' {
                    level -= 1;
                }
            }
        }
    }
    fn end(&mut self, program: &[char]) {
        if self.memory[self.pointer] != Cell::from(0) {
            let mut level: usize = 1;
            while level > 0 {
                self.counter -= 1;
                if program[self.counter] == '[' {
                    level -= 1;
                } else if program[self.counter] == ']' {
                    level += 1;
                }
            }
        }
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
        let mut engine = BrainFuck::new(256, "");
        assert_eq!(engine.run(program), "Hello World!");
    }

    #[test]
    fn test_input() {
        let program = ",++.,-.";
        let mut engine = BrainFuck::new(256, "a");
        assert_eq!(engine.run(program), "cÿ");
    }
    #[test]
    fn test_wraparound() {
        let program = "->>.";
        let mut engine = BrainFuck::new(2, "");
        assert_eq!(engine.run(program), "ÿ");
    }
}
