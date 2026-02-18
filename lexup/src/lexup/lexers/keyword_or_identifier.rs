use crate::*;
use crate::SupToken as Tk;
use super::super::*;


/// Lexer that will match an identifier, producing either a keyword or identifier token.
pub fn keyword_or_identifier() -> Lexer<KeywordOrIdentifier>
{
    Lexer(KeywordOrIdentifier)
}


pub struct KeywordOrIdentifier;

impl Lexes for KeywordOrIdentifier
{
    type Output = SupToken;

    fn lex<'s>(&self, source: &'s str) -> LexResult<'s, Self::Output>
    {
        (
            ident()
                .map(|ident|
                    /* Check if the 'identifier' is a keyword - if so, produce the corresponding keyword token, otherwise wrap the name in `IDENT` */
                    SUPCODE_KEYWORDS.get(&ident)
                        .cloned()
                        .unwrap_or(
                            Tk::IDENT(ident.to_string())
                        )
                )
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
        assert_lexes!( keyword_or_identifier().lex("") );
        assert_lexes!( keyword_or_identifier().lex("if")         => Tk::IF );
        assert_lexes!( keyword_or_identifier().lex("is")         => Tk::IS );
        assert_lexes!( keyword_or_identifier().lex("ifs")        => Tk::IDENT(str!("ifs")) );
        assert_lexes!( keyword_or_identifier().lex("while")      => Tk::WHILE );
        assert_lexes!( keyword_or_identifier().lex("while-true") => Tk::IDENT(str!("while-true")) );
    }
}
