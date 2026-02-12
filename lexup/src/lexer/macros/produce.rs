#[macro_export]
macro_rules! produces {
    ($product:expr => $lexer:expr) =>
    {
        $lexer.produce($product)
    };
}

pub use produces;
