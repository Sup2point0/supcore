use crate::*;


pub type LexResult<'s, Output> = Result<(Output, &'s str), LexError>;


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

    fn produce<Out>(self, out: Out) -> Mapped<Self, impl Fn(Self::Output) -> Out, Out>
        where
            Self: Sized,
            Out: Clone + 'static,
    {
        Mapped(self, move |_| out.clone())
    }
}
