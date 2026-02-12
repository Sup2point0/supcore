use crate::*;
use crate::lexer as lx;


pub struct Chars;

impl Lexer for Chars
{
    type Output = String;

    fn lex<'s>(&self, source: &'s str) -> LexResult<'s, Self::Output>
    {
        // lx::and![].lex(source)
        unimplemented!()
    }
}
