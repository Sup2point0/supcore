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

    PIPE, PRIME, DOT, COMMA,
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
    LOOP, WHILE, UNTIL, IN,
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
            // Self::UNKNOWN => false,  // TEMP
            Self::SKIP    => false,
            _             => true,
        }
    }

    pub fn is_keyword(&self) -> bool
    {
        match self {
            Self::SET       => true,
            Self::LET       => true,
            Self::CONST     => true,
            Self::ALT       => true,
            Self::OUT       => true,
            Self::IF        => true,
            Self::ELSE      => true,
            Self::LOOP      => true,
            Self::WHILE     => true,
            Self::UNTIL     => true,
            Self::IN        => true,
            Self::TRY       => true,
            Self::EVADE     => true,
            Self::ENSURE    => true,
            Self::FUNC      => true,
            Self::STRUCT    => true,
            Self::ARCHETYPE => true,
            Self::ACTIVATE  => true,
            Self::CREATE    => true,
            Self::EVOLVE    => true,
            Self::AUTO      => true,
            Self::WITH      => true,
            Self::IS        => true,
            _               => false,
        }
    }
}

impl std::fmt::Display for SupToken
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "{}", match self
        {
            Self::SKIP    => "·",
            Self::UNKNOWN => "?",

            Self::L_PAREN   => "(",
            Self::R_PAREN   => ")",
            Self::L_BRACKET => "[",
            Self::R_BRACKET => "]",
            Self::L_BRACE   => "\n{\n",
            Self::R_BRACE   => "\n}\n",
            Self::L_ANGLE   => " <",
            Self::R_ANGLE   => "> ",

            Self::PIPE  => " | ",
            Self::PRIME => "'",
            Self::DOT   => ".",
            Self::COMMA => ", ",
            Self::PLUS  => " + ",
            Self::MINUS => " - ",
            Self::STAR  => " * ",
            Self::SLASH => " / ",
            Self::TILDE => " ~",

            Self::EQ   => " = ",
            Self::EQQ  => " == ",
            Self::LTEQ => " =< ",
            Self::GTEQ => " >= ",
            Self::NEQ  => " != ",

            Self::IDENT(ident) => return write!(f, "{ident}"),
            Self::INT(int)     => return write!(f, "{int}"),
            Self::STR(str)     => return write!(f, "\"{str}\""),

            _ if self.is_keyword() => {
                std::fmt::Debug::fmt(self, f)?;
                return write!(f, " ");
            },

            _ => return std::fmt::Debug::fmt(self, f),
        })
    }
}
