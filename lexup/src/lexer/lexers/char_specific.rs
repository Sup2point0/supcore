use crate::*;
use crate::lexer as lx;


pub fn char1(c: char) -> Lexer<SpecificChar>
{
    Lexer(SpecificChar(c))
}


pub struct SpecificChar(pub char);

impl Lexes for SpecificChar
{
    type Output = char;

    fn lex<'s>(&self, source: &'s str) -> LexResult<'s, Self::Output>
    {
        lx::Satisfies(|c| *c == self.0).lex(source)
    }
}
