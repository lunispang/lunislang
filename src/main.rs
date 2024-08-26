enum Token {
    Plus,
    Minus,
    Multiply,
    Divide,
    Equal,
    Lparen,
    Rparen,
    Int(i32),
    Ident(String),
}

struct Lexer<'a> {
    chars: &'a str,
}

impl Iterator for Lexer<'_> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        match self.chars.chars().next()? {
            '+' => Some(Token::Plus),
            '-' => Some(Token::Minus),
            '*' => Some(Token::Multiply),
            '/' => Some(Token::Divide),
            '=' => Some(Token::Equal),
            '(' => Some(Token::Lparen),
            ')' => Some(Token::Rparen),
            '0'..='9' => Some(Token::Int(self.chars.chars().take_while(|c| c.is_ascii_digit()).collect::<String>().parse().unwrap())),
            _ => None,
        }
    }
}

fn main() {
    println!("Hello, world!");
}
