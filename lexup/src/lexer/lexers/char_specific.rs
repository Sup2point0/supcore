use crate::*;
use crate::lexer as lx;


pub struct Char(pub char);

impl Lexer for Char
{
    type Output = char;

    fn lex<'s>(&self, source: &'s str) -> LexResult<'s, Self::Output>
    {
        lx::Satisfies(|c| *c == self.0).lex(source)
    }
}

impl_bitor!(Char);
impl_bitand!(Char);
