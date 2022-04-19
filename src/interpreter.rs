use crate::{read_line, tokens::Token};

// Types
type CELL = u8;

// Constants
const MEMORY_LENGTH: usize = 1 << 16;

pub struct Interpreter {
    tokens: Vec<Token>,
    state: State,
}

pub struct State {
    mem: [u8; MEMORY_LENGTH],
    pointer: usize,
}

impl State {
    pub fn new() -> Self {
        Self {
            mem: [0; MEMORY_LENGTH],
            pointer: 0,
        }
    }

    pub fn increment_cell(&mut self) {
        self.mem[self.pointer] += 1;
    }

    pub fn decrement_cell(&mut self) {
        self.mem[self.pointer] -= 1;
    }

    pub fn increment_pointer(&mut self) {
        self.pointer += 1;
    }

    pub fn decrement_pointer(&mut self) {
        self.pointer -= 1;
    }

    pub fn output_cell(&mut self) {
        print!("{}", self.mem[self.pointer] as char);
    }

    pub fn input_cell(&mut self) {
        let s = read_line();
        self.mem[self.pointer] = s.trim().parse::<CELL>().unwrap();
    }
}

impl Interpreter {
    pub fn new(tokens: Vec<Token>) -> Interpreter {
        Interpreter {
            tokens,
            state: State::new(),
        }
    }

    fn get_token(&self) -> Token {
        self.tokens[self.state.pointer]
    }

    pub fn run(&mut self) -> Option<&State> {
        if self.state.pointer >= self.tokens.len() {
            return None;
        }
        let token = self.get_token();
        match token {
            Token::Plus => self.state.increment_cell(),
            Token::Minus => self.state.decrement_cell(),
            Token::Greater => self.state.increment_pointer(),
            Token::Less => self.state.decrement_pointer(),
            Token::LSquare => self.state.begin_loop(),
            Token::RSquare => self.state.end_loop(),
            Token::Comma => self.state.input_cell(),
            Token::Period => self.state.output_cell(),
        }
        Some(&self.state)
    }
}
