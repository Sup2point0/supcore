use crate::*;


/// Lexer that matches any alphabetical character (lowercase or uppercase A–Z).
pub fn alpha() -> Lexer<Alpha>
{
    Lexer(Alpha)
}


pub struct Alpha;

impl Lexes for Alpha
{
    type Output = char;

    fn lex<'s>(&self, source: &'s str) -> LexResult<'s, Self::Output>
    {
        Satisfies(|c| ('a'..='z').contains(c) || ('A'..='Z').contains(c))
            .lex(source)
    }
}
