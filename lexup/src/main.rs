use lexup;
use lexup::lexer::*;
use lexup::Lexer;


fn main()
{
    let lexer = Many(
        Or(
            And(
                Char('i'),
                Char('s'),
                |l, r| format!("{l}-{r}")
            ),
            Char('s').map(|s| s.to_string())
        )
    );
    println!("{:?}", lexer.lex("issis"));
}
