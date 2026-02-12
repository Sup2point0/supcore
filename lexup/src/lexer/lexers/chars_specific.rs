use crate::*;
use crate::lexer::*;


pub fn chars(str: &str) -> Lexer<Chars>
{
    Lexer(Chars(str.chars().collect()))
}


pub struct Chars(pub Vec<char>);

impl Lexes for Chars
{
    type Output = String;

    fn lex<'s>(&self, source: &'s str) -> LexResult<'s, Self::Output>
    {
        for (target, received) in self.0.iter().zip(source.chars()) {
            if *target != received {
                Err(LexError::NoParse)?
            }
        }

        let residue = &source[self.0.len()..];

        Ok((self.0.iter().collect(), residue))
    }
}
