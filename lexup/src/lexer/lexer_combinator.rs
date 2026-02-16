use crate::*;


/// Any composite lexer that can return a collection of the lexers it captures.
pub trait LexerCombinator
{
    type Output;

    fn extract_lexers(self) -> Vec<Box<dyn Lexes<Output = Self::Output>>>
        where Self: Sized;
}
