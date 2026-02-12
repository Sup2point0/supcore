use crate::*;


pub struct AnyChar;

impl Lexer for AnyChar
{
    type Output = char;

    fn lex<'s>(&self, source: &'s str) -> LexResult<'s, Self::Output>
    {
        Satisfies(|c| ('a'..'z').contains(c) || ('A'..'Z').contains(c))
            .lex(source)
    }
}

impl_bitor!(AnyChar);
impl_bitand!(AnyChar);
