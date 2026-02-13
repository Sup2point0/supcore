use std::boxed::Box;

use crate::{lexer::lexers, *};


// == INTERMEDIATE == //

/// Intermediate struct returned by using the `&` combinator on 2 lexers. Call `.merge()` or another appropriate resolver on this to produce a new lexer.
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

impl<Lx1, Lx2, Out> LexerCombinator for And<Lx1, Lx2>
    where
        Lx1: Lexes<Output = Out> + 'static,
        Lx2: Lexes<Output = Out> + 'static,
{
    type Output = Out;

    fn extract_lexers(self) -> Vec<Box<dyn Lexes<Output = Out>>> where Self: Sized
    {
        vec![
            Box::new(self.0),
            Box::new(self.1),
        ]
    }
}

impl<Lx1, Lx2, Rhs, Out> std::ops::BitAnd<Rhs> for And<Lx1, Lx2>
    where
        Lx1: Lexes<Output = Out> + 'static,
        Lx2: Lexes<Output = Out> + 'static,
        Rhs: LexerCombinator<Output = Out>,
{
    type Output = Chain<Out>;

    fn bitand(self, rhs: Rhs) -> Self::Output
    {
        let mut lexers = Vec::<Box<dyn Lexes<Output = Out>>>::with_capacity(3);
        
        lexers.push(Box::new(self.0));
        lexers.push(Box::new(self.1));
        lexers.extend(rhs.extract_lexers());

        Chain(lexers)
    }
}


// == LEXER == //

/// A lexer that applies 2 lexers `lexer1` and `lexer2` in sequence, and combines their outputs using `resolver`.
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


// == COMBINATORS == //

// (p & q) & r
// impl<Lx, Lx1, Lx2, Out> std::ops::BitAnd<Lexer<Lx>> for And<Lx1, Lx2>
//     where
//         Lx:  Lexes<Output = Out> + 'static,
//         Lx1: Lexes<Output = Out> + 'static,
//         Lx2: Lexes<Output = Out> + 'static,
// {
//     type Output = Chain<Out>;

//     fn bitand(self, rhs: Lexer<Lx>) -> Self::Output
//     {
//         Chain(vec![
//             Box::new(self.0),
//             Box::new(self.1),
//             Box::new(rhs.0),
//         ])
//     }
// }

// (p & q) & (r & s)
// impl<Lx1, Lx2, Lx3, Lx4, Out> std::ops::BitAnd<And<Lx3, Lx4>> for And<Lx1, Lx2>
//     where
//         Lx1: Lexes<Output = Out> + 'static,
//         Lx2: Lexes<Output = Out> + 'static,
//         Lx3: Lexes<Output = Out> + 'static,
//         Lx4: Lexes<Output = Out> + 'static,
// {
//     type Output = Chain<Out>;

//     fn bitand(self, rhs: And<Lx3, Lx4>) -> Self::Output
//     {
//         Chain(vec![
//             Box::new(self.0),
//             Box::new(self.1),
//             Box::new(rhs.0),
//             Box::new(rhs.1),
//         ])
//     }
// }
