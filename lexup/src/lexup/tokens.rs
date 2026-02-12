#[derive(Clone, PartialEq)]
pub enum SupToken
{
    L_PAREN,   R_PAREN,
    L_BRACKET, R_BRACKET,
    L_BRACE,   R_BRACE,

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
