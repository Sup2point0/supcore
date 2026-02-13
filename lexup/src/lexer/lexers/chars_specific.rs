use crate::*;
use crate::lexer::*;


pub fn chars(str: &str) -> Lexer<Chars>
{
    Lexer(Chars(str.chars().collect()))
}


pub struct Chars(pub Vec<char>);

impl Lexes for Chars
{
    type Output = String;

    fn lex<'s>(&self, source: &'s str) -> LexResult<'s, Self::Output>
    {
        let mut char_indices = source.char_indices();

        /* NOTE: Need manual iteration here so we can call `char_indices.next()` once done with iteration */
        for target in self.0.iter() {
            match char_indices.next()
            {
                None => Err(LexError::NoParse)?,
                
                Some((_, received)) => {
                    if *target != received {
                        Err(LexError::NoParse)?
                    }
                }
            }
        }

        let residue = {
            if let Some((i, _)) = char_indices.next() {
                &source[i..]
            } else {
                ""
            }
        };

        Ok((self.0.iter().collect(), residue))
    }
}
