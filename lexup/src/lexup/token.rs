use crate::*;
use crate::SupToken as Tk;


pub struct Token;

impl Lexes for Token
{
    type Output = SupToken;

    fn lex<'s>(&self, source: &'s str) -> LexResult<'s, Self::Output>
    {
        (
          produces! { Tk::SKIP => char1(' ') }

        | produces! { Tk::L_PAREN   => char1('(') }
        | produces! { Tk::R_PAREN   => char1(')') }
        | produces! { Tk::L_BRACKET => char1('[') }
        | produces! { Tk::R_BRACKET => char1(']') }
        | produces! { Tk::L_BRACE   => char1('{') }
        | produces! { Tk::R_BRACE   => char1('}') }
        | produces! { Tk::L_ANGLE   => char1('<') }
        | produces! { Tk::R_ANGLE   => char1('>') }

        | produces! { Tk::PIPE  => char1('|') }
        | produces! { Tk::DOT   => char1('.') }
        | produces! { Tk::COMMA => char1(',') }
        | produces! { Tk::PLUS  => char1('+') }
        | produces! { Tk::MINUS => char1('-') }
        | produces! { Tk::STAR  => char1('*') }
        | produces! { Tk::SLASH => char1('/') }
        | produces! { Tk::TILDE => char1('~') }

        | many1(digit()).map(|cs| Tk::INT(cs.into_iter().collect::<String>().parse::<i32>().unwrap()))

        | produces! { Tk::IF     => chars("if") }
        | produces! { Tk::ELSE   => chars("else") }
        | produces! { Tk::LOOP   => chars("loop") }
        | produces! { Tk::WHILE  => chars("while") }
        | produces! { Tk::UNTIL  => chars("until") }
        | produces! { Tk::TRY    => chars("try") }
        | produces! { Tk::EVADE  => chars("evade") }
        | produces! { Tk::ENSURE => chars("ensure") }
        
        | produces! { Tk::FUNC      => chars("func") }
        | produces! { Tk::STRUCT    => chars("struct") }
        | produces! { Tk::ARCHETYPE => chars("archetype") }
        | produces! { Tk::ACTIVATE  => chars("activate") }
        | produces! { Tk::CREATE    => chars("create") }
        | produces! { Tk::EVOLVE    => chars("evolve") }

        | produces! { Tk::UNKNOWN  => satisfies(|_| true) }
        
        ).lex(source)
    }
}
