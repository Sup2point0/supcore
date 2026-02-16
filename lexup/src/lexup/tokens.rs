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

    EQ, EQQ,
    LTEQ, GTEQ,
    NEQ,

    // Literals
    IDENT(String),
    INT(i32),
    FLOAT(f64),
    STR(String),

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
            Self::L_PAREN   => "(",
            Self::R_PAREN   => ")",
            Self::L_BRACKET => "Ln[",
            Self::R_BRACKET => "\n]",
            Self::L_BRACE   => "\n{",
            Self::R_BRACE   => "\n}",
            Self::L_ANGLE   => "<",
            Self::R_ANGLE   => ">",

            Self::PIPE  => "|",
            Self::DOT   => ".",
            Self::COMMA => ",",
            Self::PLUS  => "+",
            Self::MINUS => "-",
            Self::STAR  => "*",
            Self::SLASH => "/",
            Self::TILDE => "~",

            Self::EQ   => "=",
            Self::EQQ  => "==",
            Self::LTEQ => "=<",
            Self::GTEQ => ">=",
            Self::NEQ  => "!=",

            Self::IDENT(ident) => return write!(f, "'{ident}'"),
            Self::INT(int)     => return write!(f, "{int}"),

            _ => return std::fmt::Debug::fmt(self, f),
        })
    }
}
