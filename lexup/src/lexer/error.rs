#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum LexError
{
    NoParse,

    UnknownCharacter(char),
    UnexpectedCharacter(char),
    UnusedInput(String),

    UnknownError,
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
            Self::UnusedInput(str)       => write!(f, "Unconsumed input: {str}"),
            Self::UnknownError           => write!(f, "Unknown error"),
        }
    }
}

impl std::error::Error for LexError {}
