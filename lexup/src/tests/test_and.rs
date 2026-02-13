use crate::*;
use crate::resolvers as res;


#[test] fn bitand()
{
    let lexer = (digit() & digit())
        .merge(res::chars::join);

    assert_eq!( lexer.lex("12"), Ok(("12".to_string(), "")) );
}

#[test] fn bitand_assoc_right()
{
    let lexer = (digit() & (digit() & char1('s')))
        .reduce("".to_string(), res::chars::joins);

    assert_eq!( lexer.lex("12"),  Err(LexError::NoParse)      );
    assert_eq!( lexer.lex("12s"), Ok(("12s".to_string(), "")) );
}

#[test] fn bitand_assoc_left()
{
    let lexer = ((digit() & digit()) & char1('s'))
        .reduce("".to_string(), res::chars::joins);

    assert_eq!( lexer.lex("12"),  Err(LexError::NoParse)      );
    assert_eq!( lexer.lex("12s"), Ok(("12s".to_string(), "")) );
}

#[test] fn bitand_assoc()
{
    let lexer = (digit() & digit() & char1('s'))
        .reduce("".to_string(), res::chars::joins);

    assert_eq!( lexer.lex("12"),  Err(LexError::NoParse)      );
    assert_eq!( lexer.lex("12s"), Ok(("12s".to_string(), "")) );
    
    let lexer = (char1('.') & digit() & digit() & char1('s'))
        .reduce("".to_string(), res::chars::joins);

    assert_eq!( lexer.lex(".12"),  Err(LexError::NoParse)      );
    assert_eq!( lexer.lex("12s"),  Err(LexError::NoParse)      );
    assert_eq!( lexer.lex(".12s"), Ok((".12s".to_string(), "")) );
}

#[test] fn bitand_nesting()
{
    let lexer = ((digit() & digit()) & (char1('s') & char1('u')))
        .reduce("".to_string(), res::chars::joins);

    assert_eq!( lexer.lex("12s"),  Err(LexError::NoParse)      );
    assert_eq!( lexer.lex("12su"), Ok(("12su".to_string(), "")) );
}
