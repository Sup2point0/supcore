use crate::*;


pub struct Lexup;

impl Lexup
{
    pub fn lex_supcode<'s>(source_code: &'s str) -> Result<Vec<SupToken>, LexError>
    {
        match Many0(Token).lex(source_code)
        {
            Ok((tokens, residue)) => {
                if !residue.is_empty() {
                    Err(LexError::UnusedInput(residue.to_string()))
                } else {
                    Ok(tokens.into_iter().filter(SupToken::keep).collect())
                }
            },
            Err(e) => Err(e),
        }
    }
}
