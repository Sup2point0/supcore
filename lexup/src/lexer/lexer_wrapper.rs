use crate::*;


/// A newtype wrapper for a lexer variant.'
pub struct Lexer<Lx: Lexes>(pub(crate) Lx);

impl<Lx: Lexes> Lexer<Lx>
{
    pub fn lex<'s>(&self, source: &'s str) -> LexResult<'s, Lx::Output>
    {
        self.0.lex(source)
    }

    pub fn map<Mapper, Out>(self, f: Mapper) -> Lexer<Mapped<Lx, Mapper, Out>>
        where
            Self: Sized,
            Mapper: Fn(Lx::Output) -> Out
    {
        Lexer(
            Mapped(self.0, f)
        )
    }

    pub fn produce<Out>(self, out: Out) -> Lexer<Mapped<Lx, impl Fn(Lx::Output) -> Out, Out>>
        where
            Self: Sized,
            Out: Clone + 'static,
    {
        Lexer(
            Mapped(self.0, move |_| out.clone())
        )
    }
}

impl<Lx> LexerCombinator for Lexer<Lx>
    where Lx: Lexes + 'static
{
    type Output = Lx::Output;

    fn extract_lexers(self) -> Vec<Box<dyn Lexes<Output = Lx::Output>>> where Self: Sized
    {
        vec![Box::new(self.0)]
    }
}

impl<Lx, Lxr, Out> std::ops::BitOr<Lexer<Lxr>> for Lexer<Lx>
    where
        Lx:  Lexes<Output = Out>,
        Lxr: Lexes<Output = Out>,
{
    type Output = Lexer<Or<Lx, Lxr, Out>>;

    fn bitor(self, rhs: Lexer<Lxr>) -> Self::Output
    {
        Lexer(
            Or(self.0, rhs.0)
        )
    }
}

// p & q
impl<Lx, Lxr> std::ops::BitAnd<Lexer<Lxr>> for Lexer<Lx>
    where
        Lx:  Lexes,
        Lxr: Lexes,
{
    type Output = And<Lx, Lxr>;

    fn bitand(self, rhs: Lexer<Lxr>) -> Self::Output {
        And(self.0, rhs.0)
    }
}

// p & (q & r)
impl<Lx, Lx1, Lx2, Out> std::ops::BitAnd<And<Lx1, Lx2>> for Lexer<Lx>
    where
        Lx:  Lexes<Output = Out> + 'static,
        Lx1: Lexes<Output = Out> + 'static,
        Lx2: Lexes<Output = Out> + 'static,
{
    type Output = Chain<Out>;

    fn bitand(self, rhs: And<Lx1, Lx2>) -> Self::Output
    {
        Chain(vec![
            Box::new(self.0),
            Box::new(rhs.0),
            Box::new(rhs.1),
        ])
    }
}
