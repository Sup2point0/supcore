use lexup::*;


fn main()
{
    let lexer = Many0
    (
        (
            Char('i') & Char('s')
        ).merge(
            |l, r| format!("{l}-{r}")
        )
        |
            Char('s').map(|s| s.to_string())
    );
    println!("{:?}", lexer.lex("issis"));
}
