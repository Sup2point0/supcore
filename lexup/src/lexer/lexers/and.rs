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
    pub fn merge<Merger, Out>(self, merger: Merger) -> Seq<Lx1, Lx2, Merger, Out>
        where Merger: Fn(Lx1::Output, Lx2::Output) -> Out
    {
        Seq { lexer1: self.0, lexer2: self.1, resolver: merger }
    }
    
    pub fn left(self) -> Seq<Lx1, Lx2, impl Fn(Lx1::Output, Lx2::Output) -> Lx1::Output, Lx1::Output>
    {
        Seq { lexer1: self.0, lexer2: self.1, resolver: |l, _| l }
    }
    
    pub fn right(self) -> Seq<Lx1, Lx2, impl Fn(Lx1::Output, Lx2::Output) -> Lx2::Output, Lx2::Output>
    {
        Seq { lexer1: self.0, lexer2: self.1, resolver: |_, r| r }
    }
}


pub struct Seq<Lx1, Lx2, Resolver, Out>
    where
        Lx1: Lexer,
        Lx2: Lexer,
        Resolver: Fn(Lx1::Output, Lx2::Output) -> Out,
{
    pub lexer1: Lx1,
    pub lexer2: Lx2,
    pub resolver: Resolver
}

impl<Lx1, Lx2, Merger, Out> Lexer for Seq<Lx1, Lx2, Merger, Out>
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
