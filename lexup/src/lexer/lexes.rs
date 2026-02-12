use crate::*;


pub type LexResult<'s, Output> = Result<(Output, &'s str), LexError>;


/// Any struct with a `.lex()` method, which can be used in a lexing parser combinator chain.
pub trait Lexes
{
    type Output;

    fn lex<'s>(&self, source: &'s str) -> LexResult<'s, Self::Output>;
}
