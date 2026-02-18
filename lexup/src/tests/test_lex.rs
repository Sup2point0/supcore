use crate::*;
use crate::SupToken as Tk;


#[test] fn bitand()
{
    assert_eq!(
        Lexup::lex_supcode(""),
        Ok(vec![])
    );
    assert_eq!(
        Lexup::lex_supcode("1 + 2"),
        Ok(vec![ Tk::INT(1), Tk::PLUS, Tk::INT(2) ])
    );
    assert_eq!(
        Lexup::lex_supcode("set 'test' = \"sup\""),
        Ok(vec![ Tk::SET, Tk::PRIME, Tk::IDENT(str!("test")), Tk::PRIME, Tk::EQ, Tk::STR(str!("sup")) ])
    );
}
