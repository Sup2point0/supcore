#[macro_export]
macro_rules! impl_bitand {
    ($target:ident) =>
    {
        // impl<Lx> std::ops::BitAnd<Lx> for $target
        //     where Lx: Lexer<Output = <Self as Lexer>::Output>
        // {
        //     type Output = crate::lexer::And<Self, Lx>;

        //     fn bitand(self, rhs: Lx) -> Self::Output {
        //         crate::lexer::And(self, rhs)
        //     }
        // }
    };
}

pub use impl_bitand;
