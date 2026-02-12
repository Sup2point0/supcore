use lexup;
use lexup::lexer as lx;
use lexup::Lexer;


fn main()
{
    println!("{:?}", lx::Many(lx::AnyChar).lex("suppety"));
}
