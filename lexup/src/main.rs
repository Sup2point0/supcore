use lexup::*;


fn main()
{
    let lexer = many0
    (
        (
            char1('i') & char1('s')
        ).merge(
            |l, r| format!("{l}-{r}")
        )
        |
            char1('s').map(|s| s.to_string())
    );
    println!("{:?}", lexer.lex("issis"));
}
