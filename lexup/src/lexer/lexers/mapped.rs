use crate::*;


/// Lexer that applies a mapping function to the output of the given lexer. Construct by calling `.map` on any wrapped [`Lexer`].
pub struct Mapped<Lx, Mapper, Out>(pub Lx, pub Mapper)
    where
        Lx: Lexes,
        Mapper: Fn(Lx::Output) -> Out,
;

impl<Lx, Mapper, Out> Lexes for Mapped<Lx, Mapper, Out>
    where
        Lx: Lexes,
        Mapper: Fn(Lx::Output) -> Out,
{
    type Output = Out;

    fn lex<'s>(&self, source: &'s str) -> LexResult<'s, Self::Output>
    {
        let (res, residue) = self.0.lex(source)?;
        let out = (self.1)(res);

        Ok((out, residue))
    }
}
