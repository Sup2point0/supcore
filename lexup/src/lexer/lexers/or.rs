use std::boxed::Box;

use crate::*;


#[macro_export]
macro_rules! or {
    ($prot:expr, $( $alternates:expr ),*) =>
    {
        Or {
            alternatives: vec![
                Box::new($prot),
                $(
                    Box::new($alternates)
                ),*
            ]
        }
    };
}
pub use or;


pub struct Or<Out>
{
    pub alternatives: Vec<Box<dyn Lexer<Output = Out>>>,
}

impl<Out> Lexer for Or<Out>
{
    type Output = Out;

    fn lex<'s>(&self, source: &'s str) -> LexResult<'s, Self::Output>
    {
        for lexer in &self.alternatives {
            match lexer.lex(source) {
                Ok(out) => return Ok(out),
                Err{..} => (),
            }
        }

        Err(LexError::NoParse)
    }
}
