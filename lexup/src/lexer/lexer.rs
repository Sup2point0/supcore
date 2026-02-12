pub use crate::*;


pub trait Lexer
{
    type Output;

    fn lex<'s>(&self, source: &'s str) -> Result<(Self::Output, &'s str), LexError>;
}
