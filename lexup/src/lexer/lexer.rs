use crate::*;


pub trait Lexer
{
    type Output;

    fn lex<'s>(&self, source: &'s str) -> LexResult<'s, Self::Output>;

    fn map<Mapper, Out>(self, f: Mapper) -> Mapped<Self, Mapper, Out>
        where
            Self: Sized,
            Mapper: Fn(Self::Output) -> Out
    {
        Mapped(self, f)
    }
}
