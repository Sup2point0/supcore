use std::boxed::Box;

use crate::*;


pub struct Chain<Part>(pub Vec<Box<dyn Lexes<Output = Part>>>);

impl<Part> Chain<Part>
{
    pub fn reduce<Merger, Out>(self, init: Out, reducer: Merger) -> Lexer<Chained<Part, Merger, Out>>
        where
            Merger: Fn(Out, Part) -> Out,
            Out: Clone,
    {
        Lexer(Chained { lexers: self.0, init, reducer })
    }
}


pub struct Chained<Part, Merger, Out>
    where
        Merger: Fn(Out, Part) -> Out,
        Out: Clone,
{
    pub lexers:  Vec<Box<dyn Lexes<Output = Part>>>,
    pub init:    Out,
    pub reducer: Merger,
}

impl<Part, Merger, Out> Lexes for Chained<Part, Merger, Out>
    where
        Merger: Fn(Out, Part) -> Out,
        Out: Clone,
{
    type Output = Out;

    fn lex<'s>(&self, source: &'s str) -> LexResult<'s, Self::Output>
    {
        let mut products = vec![];
        let mut residue = source;

        for lexer in &self.lexers {
            let (prod, rest) = lexer.lex(residue)?;
            products.push(prod);
            residue = rest;
        }

        let out = products.into_iter()
            .fold(self.init.clone(), |acc, prod| (self.reducer)(acc, prod));

        Ok((out, residue))
    }
}
