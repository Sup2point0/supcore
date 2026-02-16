use crate::*;


pub fn except(c: char) -> Lexer<ExceptChar>
{
    Lexer(ExceptChar(c))
}


pub struct ExceptChar(char);

impl Lexes for ExceptChar
{
    type Output = char;

    fn lex<'s>(&self, source: &'s str) -> LexResult<'s, Self::Output>
    {
        Satisfies(|c| *c != self.0)
            .lex(source)
    }
}
