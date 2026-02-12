use crate::*;


pub struct Token;

impl Lexes for Token
{
    type Output = SupToken;

    fn lex<'s>(&self, source: &'s str) -> LexResult<'s, Self::Output>
    {
        (
            (produces! { SupToken::ACTIVATE => chars("activate") })
            // | AnyChar.map(|c| LexError::UnknownCharacter(c))
        ).lex(source)
    }
}
