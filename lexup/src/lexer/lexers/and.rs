use std::boxed::Box;

use crate::*;


pub struct And<Lx1, Lx2>(pub Lx1, pub Lx2)
    where
        Lx1: Lexes,
        Lx2: Lexes,
;

impl<Lx1, Lx2> And<Lx1, Lx2>
    where
        Lx1: Lexes,
        Lx2: Lexes,
{
    pub fn merge<Merger, Out>(self, merger: Merger) -> Lexer<Pair<Lx1, Lx2, Merger, Out>>
        where Merger: Fn(Lx1::Output, Lx2::Output) -> Out
    {
        Lexer(Pair { lexer1: self.0, lexer2: self.1, resolver: merger })
    }
    
    pub fn left(self) -> Lexer<Pair<Lx1, Lx2, impl Fn(Lx1::Output, Lx2::Output) -> Lx1::Output, Lx1::Output>>
    {
        Lexer(Pair { lexer1: self.0, lexer2: self.1, resolver: |l, _| l })
    }
    
    pub fn right(self) -> Lexer<Pair<Lx1, Lx2, impl Fn(Lx1::Output, Lx2::Output) -> Lx2::Output, Lx2::Output>>
    {
        Lexer(Pair { lexer1: self.0, lexer2: self.1, resolver: |_, r| r })
    }
}


pub struct Pair<Lx1, Lx2, Resolver, Out>
    where
        Lx1: Lexes,
        Lx2: Lexes,
        Resolver: Fn(Lx1::Output, Lx2::Output) -> Out,
{
    pub lexer1: Lx1,
    pub lexer2: Lx2,
    pub resolver: Resolver,
}

impl<Lx1, Lx2, Merger, Out> Lexes for Pair<Lx1, Lx2, Merger, Out>
    where
        Lx1: Lexes,
        Lx2: Lexes,
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
