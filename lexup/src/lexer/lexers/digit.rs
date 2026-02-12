use crate::*;
use crate::lexer as lx;


pub struct Digit;

impl Lexer for Digit
{
    type Output = char;

    fn lex<'s>(&self, source: &'s str) -> LexResult<'s, Self::Output>
    {
        lx::Satisfies(|c| ('0'..'9').contains(c))
            .lex(source)
    }
}
