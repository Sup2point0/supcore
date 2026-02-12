use crate::*;


pub struct Or<Lx1, Lx2, Out>(pub Lx1, pub Lx2)
    where
        Lx1: Lexer<Output = Out>,
        Lx2: Lexer<Output = Out>,
;

impl<Lx1, Lx2, Out> Lexer for Or<Lx1, Lx2, Out>
    where
        Lx1: Lexer<Output = Out>,
        Lx2: Lexer<Output = Out>,
{
    type Output = Out;

    fn lex<'s>(&self, source: &'s str) -> Result<(Self::Output, &'s str), LexError>
    {
        match (self.0).lex(source)
        {
            Ok(prot)  => Ok(prot),
            Err(fail) => {
                match (self.1).lex(source) {
                    Ok(deut) => Ok(deut),
                    Err{..}  => Err(fail),
                }
            },
        }
    }
}
