use crate::*;


pub struct Mapped<Lx, Mapper, Out>(pub Lx, pub Mapper)
    where
        Lx: Lexer,
        Mapper: Fn(Lx::Output) -> Out,
;

impl<Lx, Mapper, Out> Lexer for Mapped<Lx, Mapper, Out>
    where
        Lx: Lexer,
        Mapper: Fn(Lx::Output) -> Out,
{
    type Output = Out;

    fn lex<'s>(&self, source: &'s str) -> LexResult<'s, Self::Output>
    {
        let (res, residue) = (self.0).lex(source)?;
        let out = (self.1)(res);

        Ok((out, residue))
    }
}
