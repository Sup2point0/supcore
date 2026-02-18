use crate::*;


/// Lexer that matches on 1 or more occurrences of the given lexer.
/// 
/// For instance, `many1(digit())` will match on `"2"`, `"69"`, `"4294967296"`, etc. but not on `""`.
pub fn many1<Lx: Lexes>(lexer: Lexer<Lx>) -> Lexer<Many1<Lx>>
{
    Lexer(Many1(lexer.0))
}


pub struct Many1<Lx>(pub Lx) where Lx: Lexes;

impl<Lx> Lexes for Many1<Lx> where Lx: Lexes
{
    type Output = Vec<Lx::Output>;

    fn lex<'s>(&self, source: &'s str) -> LexResult<'s, Self::Output>
    {
        let (prod, mut residue) = self.0.lex(source)?;
        let mut out = vec![prod];

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
