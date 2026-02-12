use crate::*;


pub struct And<Lx1, Lx2, Merger, Out>(pub Lx1, pub Lx2, pub Merger)
    where
        Lx1: Lexer,
        Lx2: Lexer,
        Merger: Fn(Lx1::Output, Lx2::Output) -> Out,
;

impl<Lx1, Lx2, Merger, Out> Lexer for And<Lx1, Lx2, Merger, Out>
    where
        Lx1: Lexer,
        Lx2: Lexer,
        Merger: Fn(Lx1::Output, Lx2::Output) -> Out,
{
    type Output = Out;

    fn lex<'s>(&self, source: &'s str) -> LexResult<'s, Self::Output>
    {
        let (res1, rest)    = (self.0).lex(source)?;
        let (res2, residue) = (self.1).lex(rest)?;

        let out = (self.2)(res1, res2);

        Ok((out, residue))
    }
}
