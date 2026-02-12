#[macro_export]
macro_rules! impl_bitor {
    ($target:ident) =>
    {
        impl<Lxr> std::ops::BitOr<Lxr> for $target
            where Lxr: Lexer<Output = <Self as Lexer>::Output>
        {
            type Output = crate::lexer::Or<Self, Lxr, <Self as Lexer>::Output>;

            fn bitor(self, rhs: Lxr) -> Self::Output {
                crate::lexer::Or(self, rhs)
            }
        }
    };
}

pub use impl_bitor;

#[macro_export]
macro_rules! bitor_impl {
    () =>
    {
        type Output = crate::lexer::Or<Self, Lxr, <Self as Lexer>::Output>;

            fn bitor(self, rhs: Lxr) -> Self::Output {
                crate::lexer::Or(self, rhs)
            }
    };
}

pub use bitor_impl;
