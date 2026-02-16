use crate::*;
use crate::tests::utils::*;


pub fn ident() -> Lexer<Identifier>
{
    Lexer(Identifier)
}


pub struct Identifier;

impl Lexes for Identifier
{
    type Output = String;

    fn lex<'s>(&self, source: &'s str) -> LexResult<'s, Self::Output>
    {
        (
            // `-long-identifier1-`
            (
                (alpha() | char1('-'))
                & many1(alpha() | char1('-') | digit())
            )
            .merge(resolvers::chars::cons)

            |

            // `v`
            alpha().map(|c| c.to_string())
        )
        .lex(source)
    }
}


#[cfg(test)] mod test
{
    use crate::*;
    use super::*;

    #[test] fn test()
    {
        assert_lexes!( ident().lex("") );
        assert_lexes!( ident().lex("test")           => str!("test") );
        assert_lexes!( ident().lex("test-ident")     => str!("test-ident") );
        assert_lexes!( ident().lex("test-ident2")    => str!("test-ident2") );
        assert_lexes!( ident().lex("-test-ident3")   => str!("-test-ident3") );
        assert_lexes!( ident().lex("--test-ident4")  => str!("--test-ident4") );
        assert_lexes!( ident().lex("--test-ident5-") => str!("--test-ident5-") );
    }
}
