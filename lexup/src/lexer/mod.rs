mod lexer; pub use lexer::Lexer;
mod error; pub use error::{ LexError, LexResult };

mod lexers {
    mod mapped; pub use mapped::Mapped;

    mod or;   pub use or::Or;
    mod and;  pub use and::And;
    mod many; pub use many::Many;
    // mod some; pub use some::Some;

    mod satisfies; pub use satisfies::Satisfies;
    mod any_char;  pub use any_char::AnyChar;
    mod one_char;  pub use one_char::Char;
    mod chars;     pub use chars::Chars;
    mod digit;     pub use digit::Digit;
}
pub use lexers::*;

mod macros {
    mod bitand; pub use bitand::impl_bitand;
    mod bitor;  pub use bitor::impl_bitor;
}
pub use macros::*;
