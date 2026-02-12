// use std::boxed::Box;

// use crate::*;


// pub struct Chain<Lx1, Lx2>(pub Lx1, pub Lx2)
//     where
//         Lx1: Lexer,
//         Lx2: Lexer,
// ;

// impl<Lx1, Lx2> Chain<Lx1, Lx2>
//     where
//         Lx1: Lexer,
//         Lx2: Lexer,
// {
//     pub fn merge<Merger, Out>(self, merger: Merger) -> Chained<Lx1, Lx2, Merger, Out>
//         where Merger: Fn(Lx1::Output, Lx2::Output) -> Out
//     {
//         Chained { lexer1: self.0, lexer2: self.1, resolver: merger }
//     }
    
//     pub fn left(self) -> Chained<Lx1, Lx2, impl Fn(Lx1::Output, Lx2::Output) -> Lx1::Output, Lx1::Output>
//     {
//         Chained { lexer1: self.0, lexer2: self.1, resolver: |l, _| l }
//     }
    
//     pub fn right(self) -> Chained<Lx1, Lx2, impl Fn(Lx1::Output, Lx2::Output) -> Lx2::Output, Lx2::Output>
//     {
//         Chained { lexer1: self.0, lexer2: self.1, resolver: |_, r| r }
//     }
// }


// pub struct AndSeq<Part, Merger, Out> where Merger: Fn(Part, Part) -> Out
// {
//     pub lexers: Vec<Box<dyn Lexer<Output = Part>>>,
//     pub merger: Merger,
// }
