#[derive(Clone, PartialEq, Debug)]
pub enum SupToken
{
    // Control
    SKIP,
    UNKNOWN,

    // Punctuation
    L_PAREN,   R_PAREN,
    L_BRACKET, R_BRACKET,
    L_BRACE,   R_BRACE,
    L_ANGLE,   R_ANGLE,

    PIPE, DOT, COMMA,
    PLUS, MINUS, STAR, SLASH,
    TILDE,

    // Literals
    IDENT(String),
    INT(i32),
    FLOAT(f64),

    // Keywords
    SET, LET, CONST,
    ALT,
    OUT,

    IF, ELSE,
    LOOP, WHILE, UNTIL,
    TRY, EVADE, ENSURE,

    FUNC, STRUCT, ARCHETYPE,
    ACTIVATE, CREATE, EVOLVE,

    AUTO, WITH,

    IS,
}

impl SupToken
{
    pub fn keep(&self) -> bool
    {
        match self {
            Self::UNKNOWN => false,  // TEMP
            Self::SKIP    => false,
            _             => true,
        }
    }
}

impl std::fmt::Display for SupToken
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "{}", match self
        {
            Self::L_PAREN      => "(",
            Self::R_PAREN      => ")",
            // Self::L_BRACKET    => "",
            // Self::R_BRACKET    => "",
            // Self::L_BRACE      => "",
            // Self::R_BRACE      => "",
            // Self::L_ANGLE      => "",
            // Self::R_ANGLE      => "",
            Self::IDENT(ident) => ident,
            _ => return std::fmt::Debug::fmt(self, f),
        })
    }
}
