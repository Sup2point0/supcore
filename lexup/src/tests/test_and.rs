use crate::*;


#[test] fn bitand()
{
    let lexer = (
        digit() & digit()
    )
    .merge(|l, r| [l, r].iter().collect());

    assert_eq!(
        lexer.lex("12"),
        Ok(("12".to_string(), ""))
    );
}

#[test] fn bitand_assoc_right()
{
    let lexer = (
        digit() & (digit() & char1('s'))
    )
    .reduce("".to_string(), |mut acc, prod| {
        acc.push(prod);
        return acc;
    });

    assert_eq!( lexer.lex("12"),  Err(LexError::NoParse)      );
    assert_eq!( lexer.lex("12s"), Ok(("12s".to_string(), "")) );
}

#[test] fn bitand_assoc_left()
{
    let lexer = (
        (digit() & digit()) & char1('s')
    )
    .reduce("".to_string(), |mut acc, prod| {
        acc.push(prod);
        return acc;
    });

    assert_eq!( lexer.lex("12"),  Err(LexError::NoParse)      );
    assert_eq!( lexer.lex("12s"), Ok(("12s".to_string(), "")) );
}
