#[macro_export]
macro_rules! str {
    ($str:expr) => {
        String::from($str)
    };
}

pub use str;

#[macro_export]
macro_rules! assert_lexes
{
    ($lexer:expr) =>
    {
        assert_eq!( $lexer, Err(LexError::NoParse) );
    };

    ($lexer:expr => $product:expr) =>
    {
        assert_eq!( $lexer, Ok(($product, "")) );
    };
}

pub use assert_lexes;
