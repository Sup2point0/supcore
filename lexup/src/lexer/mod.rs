mod lexer; pub use lexer::Lexer;
mod error; pub use error::{ LexError, LexResult };

mod lexers {
    pub(crate) mod or;   pub use or::{ Or, or };
    pub(crate) mod and;  pub use and::And;
    pub(crate) mod many; pub use many::Many;
    // pub(crate) mod some; pub use some::Some;

    pub(crate) mod satisfies; pub use satisfies::Satisfies;
    pub(crate) mod any_char;  pub use any_char::AnyChar;
    pub(crate) mod one_char;  pub use one_char::Char;
    pub(crate) mod chars;     pub use chars::Chars;
    pub(crate) mod digit;     pub use digit::Digit;
}

pub use lexers::*;
