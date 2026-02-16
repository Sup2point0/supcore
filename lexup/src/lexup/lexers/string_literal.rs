use crate::*;


pub fn string() -> Lexer<StringLiteral>
{
    Lexer(StringLiteral)
}


pub struct StringLiteral;

impl Lexes for StringLiteral
{
    type Output = SupToken;

    fn lex<'s>(&self, source: &'s str) -> LexResult<'s, Self::Output>
    {
        // let mut out = String::new();

        // let (prod, residue) = char1('"').lex(source)?;
        // out.push(prod);

        // let (prod, residue) = (
        //     many0(except('\\')) & char1()
        // ).lex(residue)?;
        // out.push(prod);

        // let (prod, residue) = char1('"').lex(residue)?;
        // out.push(prod);

        // Ok((SupToken::STR(out), residue))
        unimplemented!()
    }
}
