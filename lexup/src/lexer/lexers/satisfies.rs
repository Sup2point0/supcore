pub use crate::*;


pub struct Satisfies<Pred>(pub Pred) where Pred: Fn(&char) -> bool;

impl<Pred> Lexer for Satisfies<Pred> where Pred: Fn(&char) -> bool
{
    type Output = char;

    fn lex<'s>(&self, source: &'s str) -> Result<(Self::Output, &'s str), LexError>
    {
        if let Some(c) = source.chars().next() && (self.0)(&c) {
            Ok((c, &source[1..]))
        }
        else {
            Err(LexError::NoParse)
        }
    }
}
