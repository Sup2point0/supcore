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


// == COMBINATORS == //

// p | q
impl<Lx, Lxr, Out> std::ops::BitOr<Lexer<Lxr>> for Lexer<Lx>
    where
        Lx:  Lexes<Output = Out>,
        Lxr: Lexes<Output = Out>,
{
    type Output = Lexer<Or<Lx, Lxr, Out>>;

    /// *Alternative* combinator: try applying the left lexer, and if it fails, try applying the right lexer.
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

    /// *Sequence* combinator: apply the left lexer, then apply the right lexer.
    /// 
    /// Note that this does not return a new lexer, but an intermediate representation – you must provide a way to resolve the outputs of the lexers. See [`And`] for more.
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

    /// *Sequence* combinator: apply the left lexer, then apply the right lexers.
    /// 
    /// Note that this does not return a new lexer, but an intermediate representation – you must provide a way to resolve the outputs of the lexers. See [`Chain`] for more.
    fn bitand(self, rhs: And<Lx1, Lx2>) -> Self::Output
    {
        Chain(vec![
            Box::new(self.0),
            Box::new(rhs.0),
            Box::new(rhs.1),
        ])
    }
}

// p & (q & r & s)
impl<Lx, Out> std::ops::BitAnd<Chain<Out>> for Lexer<Lx>
    where
        Lx:  Lexes<Output = Out> + 'static,
{
    type Output = Chain<Out>;

    /// *Sequence* combinator: apply the left lexer, then apply the right lexers.
    /// 
    /// Note that this does not return a new lexer, but an intermediate representation – you must provide a way to resolve the outputs of the lexers. See [`Chain`] for more.
    fn bitand(self, rhs: Chain<Out>) -> Self::Output
    {
        let mut lexers = Vec::<Box<dyn Lexes<Output = Out>>>::with_capacity(1 + rhs.0.len());

        lexers.push(Box::new(self.0));
        lexers.extend(rhs.0);

        Chain(lexers)
    }
}
