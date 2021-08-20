pub mod lexer;

use lexer::{Logos, Token};

fn main() {
    let input = std::fs::read_to_string("test.reflect").unwrap();

    let mut lex = Token::lexer(&input);
    while let Some(t) = lex.next() {
        dbg!(t);
    }
}
