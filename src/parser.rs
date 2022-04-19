use crate::tokens::Token;

pub struct Parser {
    program: String,
    index: usize,
}

impl Parser {
    pub fn new(program: String) -> Self {
        Self { program, index: 0 }
    }

    fn get_slice_at_ind(&self, ind: usize) -> &str {
        &self.program[ind..=ind]
    }

    pub fn next(&mut self) -> Option<Token> {
        if self.index >= self.program.len() {
            return None;
        }
        let c = self.get_slice_at_ind(self.index);
        self.index += 1;
        match c {
            "+" => Some(Token::Plus),
            "-" => Some(Token::Minus),
            ">" => Some(Token::Greater),
            "<" => Some(Token::Less),
            "[" => Some(Token::LSquare),
            "]" => Some(Token::RSquare),
            "," => Some(Token::Comma),
            "." => Some(Token::Period),
            _ => None,
        }
    }
    pub fn into_token_stream(mut self) -> impl Iterator<Item = Token> {
        std::iter::from_fn(move || self.next())
    }
}
