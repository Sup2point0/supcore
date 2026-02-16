use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::*;
use crate::SupToken as Tk;


macro_rules! record {
    ($( $key:expr => $val:expr ),* $(,)?) =>
    {
        HashMap::from([
            $( ($key.to_string(), $val) ),*
        ])
    };
}


lazy_static! {
    pub static ref SUPCODE_KEYWORDS: HashMap<String, SupToken> = record! {
            "set"   => Tk::SET,
            "let"   => Tk::LET,
            "const" => Tk::CONST,
            "alt"   => Tk::ALT,
            "out"   => Tk::OUT,

            "if"     => Tk::IF,
            "else"   => Tk::ELSE,
            "loop"   => Tk::LOOP,
            "while"  => Tk::WHILE,
            "until"  => Tk::UNTIL,
            "try"    => Tk::TRY,
            "evade"  => Tk::EVADE,
            "ensure" => Tk::ENSURE,

            "func"      => Tk::FUNC,
            "struct"    => Tk::STRUCT,
            "archetype" => Tk::ARCHETYPE,
            "activate"  => Tk::ACTIVATE,
            "create"    => Tk::CREATE,
            "evolve"    => Tk::EVOLVE,

            "auto"      => Tk::AUTO,
            "with"      => Tk::WITH,

            "is"        => Tk::IS,
    };
}
