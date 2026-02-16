use crate::*;
use crate::SupToken as Tk;
use super::*;


pub struct Token;

impl Lexes for Token
{
    type Output = SupToken;

    fn lex<'s>(&self, source: &'s str) -> LexResult<'s, Self::Output>
    {
        (
          produces! { Tk::SKIP => char1(' ') }

        // Comment (single-line)
        | produces! { Tk::SKIP =>
            (
                (chars("\\") & (char1(' ') | char1('\n'))).right()
                & (many0(except('\n')) & char1('\n')).right()
            ).right()
        }
        // Comment (multi-line)
        | produces! { Tk::SKIP =>
            (
                (chars("\\\\") & (char1(' ') | char1('\n'))).right()
                & (many0(except('\\')) & chars("\\\\")).right()
            ).right()
        }

        | produces! { Tk::NEQ  => chars("!=") }
        | produces! { Tk::EQQ  => chars("==") }
        | produces! { Tk::LTEQ => chars("=<") }
        | produces! { Tk::GTEQ => chars(">=") }

        | produces! { Tk::L_PAREN   => char1('(') }
        | produces! { Tk::R_PAREN   => char1(')') }
        | produces! { Tk::L_BRACKET => char1('[') }
        | produces! { Tk::R_BRACKET => char1(']') }
        | produces! { Tk::L_BRACE   => char1('{') }
        | produces! { Tk::R_BRACE   => char1('}') }
        | produces! { Tk::L_ANGLE   => char1('<') }
        | produces! { Tk::R_ANGLE   => char1('>') }

        | produces! { Tk::EQ    => char1('=') }
        | produces! { Tk::PIPE  => char1('|') }
        | produces! { Tk::PRIME => char1('\'') }
        | produces! { Tk::DOT   => char1('.') }
        | produces! { Tk::COMMA => char1(',') }
        | produces! { Tk::PLUS  => char1('+') }
        | produces! { Tk::MINUS => char1('-') }
        | produces! { Tk::STAR  => char1('*') }
        | produces! { Tk::SLASH => char1('/') }
        | produces! { Tk::TILDE => char1('~') }

        | many1(digit()).map(|cs| Tk::INT(cs.into_iter().collect::<String>().parse::<i32>().unwrap()))

        | keyword_or_identifier()

        | string()

        | produces! { Tk::SKIP     => char1('\n') }
        | produces! { Tk::UNKNOWN  => satisfies(|_| true) }
        
        ).lex(source)
    }
}
