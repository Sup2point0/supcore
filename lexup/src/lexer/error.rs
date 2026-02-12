#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum LexError
{
    NoParse,
    UnknownCharacter(char),
    UnexpectedCharacter(char),
}

impl std::fmt::Display for LexError
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        match self
        {
            Self::NoParse                => write!(f, "No parse"),
            Self::UnknownCharacter(c)    => write!(f, "Unknown character: {c}"),
            Self::UnexpectedCharacter(c) => write!(f, "Unexpected input: {c}"),
        }
    }
}

impl std::error::Error for LexError {}
