//! The programming language's lexer.

pub use logos::{Lexer, Logos};

#[derive(Debug, Logos, PartialEq)]
pub enum Token {

    /// A comment, the pivotal token for this programming language.
    #[regex("#(.*)")]
    Comment,

    /// An identifier literal.
    #[regex("[a-zA-Z$_][a-zA-Z$_0-9]*")]
    Identifier,

    /// A quoted string literal.
    #[regex("\"(?:[^\"\\\\]|\\\\.)*\"")]
    String,

    /// A character literal.
    #[regex("'.'")]
    Char,

    /// A whitespace character that is ignored by the lexer.  Matches line breaks, tabs and regular
    /// whitespaces.
    #[regex("\\s", logos::skip)]
    Whitespace,

    /// An invalid token that means that the current character in the string isn't a valid
    /// token.
    #[error]
    Invalid,

}