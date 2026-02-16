use crate::*;


pub fn string() -> Lexer<StringLiteral>
{
    Lexer(StringLiteral)
}


pub struct StringLiteral;

impl Lexes for StringLiteral
{
    type Output = SupToken;

    fn lex<'s>(&self, source: &'s str) -> LexResult<'s, Self::Output>
    {
        let mut out = String::new();
        let mut residue = source;

        let (_, rest) = char1('"').lex(residue)?;
        residue = rest;

        loop {
            match excepts("\"\\").lex(residue)
            {
                Ok((prod, rest)) => {
                    out.push(prod);
                    residue = rest;
                },
                Err{..} => match (chars("\\\"") | char1('\\').map(|c| c.to_string())).lex(residue)
                {
                    Ok((prod, rest)) => {
                        out.push_str(&prod);
                        residue = rest;
                    },
                    Err(LexError::NoParse) => {
                        /* NOTE: Guaranteed to be terminating ", so we've finished parsing the string */
                        let (_, rest) = satisfies(|_| true).lex(residue)?;
                        residue = rest;
                        break;
                    },
                    Err(e) => return Err(e),
                }
            }
        }

        Ok((SupToken::STR(out), residue))
    }
}


#[cfg(test)] mod test
{
    use crate::*;
    use super::*;

    #[test] fn test()
    {
        assert_lexes!( string().lex("") );
        assert_lexes!( string().lex("\"sup world\"") => SupToken::STR(str!("sup world")) );
        assert_lexes!( string().lex("\"\\n\"")       => SupToken::STR(str!("\\n")) );

        assert_lexes!( many0(string()).lex("\"sup\"\"2.0\"") => vec![
            SupToken::STR(str!("sup")),
            SupToken::STR(str!("2.0")),
        ]);
    }
}
