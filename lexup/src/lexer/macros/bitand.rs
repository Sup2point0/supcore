#[macro_export]
macro_rules! impl_bitand {
    ($target:ident) =>
    {
        impl<Lx> std::ops::BitAnd<Lx> for $target
            where Lx: Lexer<Output = <Self as Lexer>::Output>
        {
            type Output = lx::Or<Self, Lx, <Self as Lexer>::Output>;

            fn bitand(self, rhs: Lx) -> Self::Output {
                lx::And(self, rhs)
            }
        }
    };
}
pub use impl_bitand;
