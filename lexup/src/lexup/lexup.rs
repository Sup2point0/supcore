use crate::*;


pub struct Lexup;

impl Lexup
{
    pub fn lex_supcode<'s>(source_code: &'s str) -> LexResult<'s, Vec<SupToken>>
    {
        Many0(Token).lex(source_code)
    }
}
