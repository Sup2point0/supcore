use std::boxed::Box;

use crate::*;


pub struct And<Lx1, Lx2>(pub Lx1, pub Lx2)
    where
        Lx1: Lexer,
        Lx2: Lexer,
;

impl<Lx1, Lx2> And<Lx1, Lx2>
    where
        Lx1: Lexer,
        Lx2: Lexer,
{
    pub fn merge<Merger, Out>(self, merger: Merger) -> Pair<Lx1, Lx2, Merger, Out>
        where Merger: Fn(Lx1::Output, Lx2::Output) -> Out
    {
        Pair { lexer1: self.0, lexer2: self.1, resolver: merger }
    }
    
    pub fn left(self) -> Pair<Lx1, Lx2, impl Fn(Lx1::Output, Lx2::Output) -> Lx1::Output, Lx1::Output>
    {
        Pair { lexer1: self.0, lexer2: self.1, resolver: |l, _| l }
    }
    
    pub fn right(self) -> Pair<Lx1, Lx2, impl Fn(Lx1::Output, Lx2::Output) -> Lx2::Output, Lx2::Output>
    {
        Pair { lexer1: self.0, lexer2: self.1, resolver: |_, r| r }
    }
}

// impl<Lx, Lx1, Lx2, Resolver, Out> std::ops::BitAnd<Lx> for Pair<Lx1, Lx2, Resolver, Out>
//     where
//         Lx: Lexer<Output = <Self as Lexer>::Output>,
//         Lx1: Lexer,
//         Lx2: Lexer,
//         Resolver: Fn(Lx1::Output, Lx2::Output) -> Out,
// {
//     type Output = Chained;

//     fn bitand(self, rhs: Lx) -> Self::Output {
//         And(self, rhs)
//     }
// }


pub struct Pair<Lx1, Lx2, Resolver, Out>
    where
        Lx1: Lexer,
        Lx2: Lexer,
        Resolver: Fn(Lx1::Output, Lx2::Output) -> Out,
{
    pub lexer1: Lx1,
    pub lexer2: Lx2,
    pub resolver: Resolver,
}

impl<Lx1, Lx2, Merger, Out> Lexer for Pair<Lx1, Lx2, Merger, Out>
    where
        Lx1: Lexer,
        Lx2: Lexer,
        Merger: Fn(Lx1::Output, Lx2::Output) -> Out,
{
    type Output = Out;

    fn lex<'s>(&self, source: &'s str) -> LexResult<'s, Self::Output>
    {
        let (res1, rest)    = self.lexer1.lex(source)?;
        let (res2, residue) = self.lexer2.lex(rest)?;

        let out = (self.resolver)(res1, res2);

        Ok((out, residue))
    }
}

impl<Lx1, Lx2, Merger, Out, Lxr> std::ops::BitOr<Lxr> for Pair<Lx1, Lx2, Merger, Out>
    where
        Lxr: Lexer<Output = <Self as Lexer>::Output>,
        Lx1: Lexer,
        Lx2: Lexer,
        Merger: Fn(Lx1::Output, Lx2::Output) -> Out,
{
    bitor_impl!();
}
