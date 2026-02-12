use crate::*;


// #[macro_export]
// macro_rules! or {
//     ($prot:expr, $deut:expr, $( $more:expr ),*) =>
//     {
//         Or::of([$prot, $deut, $( $more ),*])
//     };
// }


pub struct Or<Lx1, Lx2, Out>(pub Lx1, pub Lx2)
    where
        Lx1: Lexer<Output = Out>,
        Lx2: Lexer<Output = Out>,
;

// impl<Lx1, Lx2, Out> Or<Lx1, Lx2, Out>
//     where
//         Lx1: Lexer<Output = Out>,
//         Lx2: Lexer<Output = Out>
// {
//     fn of(lexers: Vec<impl Lexer<Output = Out>>) -> Self
//     {
//         if lexers.len() == 2 {
//             Self(lexers[0], lexers[1])
//         } else {
//             Self(lexers[0], Self::of(lexers.into_iter().skip(1).collect::<Vec<_>>()))
//         }
//     }
// }

impl<Lx1, Lx2, Out> Lexer for Or<Lx1, Lx2, Out>
    where
        Lx1: Lexer<Output = Out>,
        Lx2: Lexer<Output = Out>,
{
    type Output = Out;

    fn lex<'s>(&self, source: &'s str) -> Result<(Self::Output, &'s str), LexError>
    {
        match (self.0).lex(source)
        {
            Ok(prot)  => Ok(prot),
            Err(fail) => {
                match (self.1).lex(source) {
                    Ok(deut) => Ok(deut),
                    Err{..}  => Err(fail),
                }
            },
        }
    }
}
