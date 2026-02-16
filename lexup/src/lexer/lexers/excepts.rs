use crate::*;


pub fn excepts(str: &str) -> Lexer<ExceptChars>
{
    Lexer(ExceptChars(str.chars().collect()))
}


pub struct ExceptChars(pub(super) std::collections::HashSet<char>);

impl Lexes for ExceptChars
{
    type Output = char;

    fn lex<'s>(&self, source: &'s str) -> LexResult<'s, Self::Output>
    {
        Satisfies(|c| !self.0.contains(c))
            .lex(source)
    }
}
