use crate::*;


#[derive(Copy, Clone)]
pub struct Many<Lx>(pub Lx) where Lx: Lexer;

impl<Lx> Lexer for Many<Lx> where Lx: Lexer
{
    type Output = Vec<Lx::Output>;

    fn lex<'s>(&self, source: &'s str) -> LexResult<'s, Self::Output>
    {
        let mut rest = source;
        let mut out = vec![];

        loop {
            match (self.0).lex(rest)
            {
                Ok((output, residue)) => {
                    out.push(output);
                    rest = residue;
                },
                Err{..} => break,
            }
        }

        Ok((out, rest))
    }
}
