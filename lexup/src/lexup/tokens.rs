#[derive(Clone, PartialEq, Debug)]
pub enum SupToken
{
    SKIP,
    UNKNOWN,

    L_PAREN,   R_PAREN,
    L_BRACKET, R_BRACKET,
    L_BRACE,   R_BRACE,
    L_ANGLE,   R_ANGLE,

    PIPE, DOT, COMMA,
    PLUS, MINUS, STAR, SLASH,
    TILDE,

    IDENT(String),
    INT(i32),
    FLOAT(f64),

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
