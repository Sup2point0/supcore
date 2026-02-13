use crate::*;


pub trait LexerCombinator
{
    type Output;

    fn extract_lexers(self) -> Vec<Box<dyn Lexes<Output = Self::Output>>>
        where Self: Sized;
}
