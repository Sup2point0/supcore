pub use crate::*;


pub fn satisfies<Pred>(pred: Pred) -> Lexer<Satisfies<Pred>> where Pred: Fn(&char) -> bool
{
    Lexer(Satisfies(pred))
}


pub struct Satisfies<Pred>(pub Pred) where Pred: Fn(&char) -> bool;

impl<Pred> Lexes for Satisfies<Pred> where Pred: Fn(&char) -> bool
{
    type Output = char;

    fn lex<'s>(&self, source: &'s str) -> LexResult<'s, Self::Output>
    {
        if let Some(c) = source.chars().next() && (self.0)(&c) {
            Ok((c, &source[1..]))
        }
        else {
            Err(LexError::NoParse)
        }
    }
}
