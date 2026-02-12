pub type LexResult<'s, Output> = Result<(Output, &'s str), LexError>;


#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum LexError
{
    NoParse,
    UnexpectedInput(String),
}

impl std::fmt::Display for LexError
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        match self
        {
            Self::NoParse              => write!(f, "No parse"),
            Self::UnexpectedInput(msg) => write!(f, "Unexpected input: {msg}"),
        }
    }
}

impl std::error::Error for LexError {}
