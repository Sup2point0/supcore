use crate::*;


pub type LexResult<'s, Output> = Result<(Output, &'s str), LexError>;


/// Any struct with a `.lex()` method, which can be used in a lexing combinator chain.
pub trait Lexes
{
    type Output;

    /// Lex the given `source` code, returning `(product, residue)` if a lex is successfully made.
    fn lex<'s>(&self, source: &'s str) -> LexResult<'s, Self::Output>;
}
