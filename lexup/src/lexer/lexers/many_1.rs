use crate::*;


pub fn many1<Lx: Lexes>(lexes: Lx) -> Lexer<Many1<Lx>>
{
    Lexer(Many1(lexes))
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
