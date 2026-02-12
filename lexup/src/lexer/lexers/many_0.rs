use crate::*;


pub struct Many0<Lx>(pub Lx) where Lx: Lexer;

impl<Lx> Lexer for Many0<Lx> where Lx: Lexer
{
    type Output = Vec<Lx::Output>;

    fn lex<'s>(&self, source: &'s str) -> LexResult<'s, Self::Output>
    {
        let mut residue = source;
        let mut out = vec![];

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
