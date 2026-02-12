use crate::*;
use crate::lexer as lx;


pub struct Char(char);

impl Lexer for Char
{
    type Output = char;

    fn lex<'s>(&self, source: &'s str) -> Result<(Self::Output, &'s str), LexError>
    {
        lx::Satisfies(|c| *c == self.0).lex(source)
    }
}
