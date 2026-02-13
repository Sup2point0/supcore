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
        let mut char_indices = source.char_indices();

        if let Some((_, c)) = char_indices.next() && (self.0)(&c) {
            Ok((c, {
                if let Some((i, _)) = char_indices.next() {
                    &source[i..]
                } else {
                    ""
                }
            }))
        }
        else {
            Err(LexError::NoParse)
        }
    }
}
