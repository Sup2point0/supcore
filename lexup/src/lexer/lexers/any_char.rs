use crate::*;
use crate::lexer as lx;


pub struct AnyChar;

impl Lexer for AnyChar
{
    type Output = char;

    fn lex<'s>(&self, source: &'s str) -> Result<(Self::Output, &'s str), LexError>
    {
        lx::Satisfies(|c| ('a'..'z').contains(c) || ('A'..'Z').contains(c))
            .lex(source)
    }
}
