#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(non_camel_case_types)]

mod lexup;
pub use lexup::{ Lexup, SupToken };

mod lexer;
pub use lexer::{ LexError };
pub(crate) use lexer::*;


pub(crate) mod tests
{
    pub mod utils;

    #[cfg(test)] mod test_lex;
    #[cfg(test)] mod test_and;
}
