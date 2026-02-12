use crate::*;
use crate::lexer::*;


pub struct Chars;

impl Lexer for Chars
{
    type Output = String;

    fn lex<'s>(&self, source: &'s str) -> LexResult<'s, Self::Output>
    {
        Many(AnyChar)
            .lex(source)
            .map(|(chars, residue)|
                (
                    chars.into_iter().collect::<String>(),
                    residue
                )
            )
    }
}

impl_bitor!(Chars);
impl_bitand!(Chars);
