use crate::*;


/// Lexer that matches 0 or more successful lexes of the given lexer.
/// 
/// For instance, `many0(digit())` will match on `""`, `"1"`, `"20"`, `"2147483647"`, etc.
pub fn many0<Lx: Lexes>(lexer: Lexer<Lx>) -> Lexer<Many0<Lx>>
{
    Lexer(Many0(lexer.0))
}


pub struct Many0<Lx>(pub Lx) where Lx: Lexes;

impl<Lx> Lexes for Many0<Lx> where Lx: Lexes
{
    type Output = Vec<Lx::Output>;

    fn lex<'s>(&self, source: &'s str) -> LexResult<'s, Self::Output>
    {
        let mut out = vec![];
        let mut residue = source;

        loop {
            match (self.0).lex(residue)
            {
                Ok((prod, rest)) => {
                    out.push(prod);
                    residue = rest;
                },
                Err{..} => break,
            }
        }

        Ok((out, residue))
    }
}
