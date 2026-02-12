use crate::*;
use crate::lexer as lx;


pub struct AnyChar;

impl Lexer for AnyChar
{
    type Output = char;

    fn lex<'s>(&self, source: &'s str) -> LexResult<'s, Self::Output>
    {
        lx::Satisfies(|c| ('a'..'z').contains(c) || ('A'..'Z').contains(c))
            .lex(source)
    }
}
