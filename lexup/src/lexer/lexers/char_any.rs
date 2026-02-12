use crate::*;


pub fn any_char() -> Lexer<AnyChar>
{
    Lexer(AnyChar)
}


pub struct AnyChar;

impl Lexes for AnyChar
{
    type Output = char;

    fn lex<'s>(&self, source: &'s str) -> LexResult<'s, Self::Output>
    {
        Satisfies(|c| ('a'..'z').contains(c) || ('A'..'Z').contains(c))
            .lex(source)
    }
}
