use crate::*;
use crate::lexer::*;


#[macro_export]
macro_rules! chars {
    ($str:expr) =>
    {
        Chars($str.chars().collect())
    };
}

pub use chars;


pub struct Chars(pub Vec<char>);

impl Lexer for Chars
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

impl_bitor!(Chars);
impl_bitand!(Chars);
