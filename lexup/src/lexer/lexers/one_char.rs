use crate::*;
use crate::lexer as lx;


pub struct Char(pub char);

impl Lexer for Char
{
    type Output = char;

    fn lex<'s>(&self, source: &'s str) -> LexResult<'s, Self::Output>
    {
        lx::Satisfies(|c| *c == self.0).lex(source)
    }
}

impl_bitor!(Char);
// impl_bitand!(Char);

impl<Lx> std::ops::BitAnd<Lx> for Char
    where Lx: Lexer<Output = <Self as Lexer>::Output>
{
    type Output = lx::Or<Self, Lx, <Self as Lexer>::Output>;

    fn bitand(self, rhs: Lx) -> Self::Output {
        lx::And(self, rhs)
    }
}
