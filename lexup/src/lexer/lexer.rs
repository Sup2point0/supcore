pub use crate::*;


pub trait Lexer
{
    type Output;

    fn lex<'s>(&self, source: &'s str) -> Result<(Self::Output, &'s str), LexError>;

    fn map<Mapper, Out>(self, f: Mapper) -> Mapped<Self, Mapper, Out>
        where
            Self: Sized,
            Mapper: Fn(Self::Output) -> Out
    {
        Mapped(self, f)
    }
}


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

    fn lex<'s>(&self, source: &'s str) -> Result<(Self::Output, &'s str), LexError>
    {
        let (res, residue) = (self.0).lex(source)?;
        let out = (self.1)(res);

        Ok((out, residue))
    }
}
