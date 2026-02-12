#[macro_export]
macro_rules! impl_bitor {
    ($target:ident) =>
    {
        impl<Lx> std::ops::BitOr<Lx> for $target
            where Lx: Lexer<Output = <Self as Lexer>::Output>
        {
            type Output = lx::Or<Self, Lx, <Self as Lexer>::Output>;

            fn bitor(self, rhs: Lx) -> Self::Output {
                lx::Or(self, rhs)
            }
        }
    };
}
pub use impl_bitor;
