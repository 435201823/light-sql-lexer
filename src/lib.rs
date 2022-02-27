mod token;

#[cfg(test)]
mod tests {
    use crate::token::Token;
    use logos::Logos;

    #[test]
    fn it_works() {

        let mut lex = Token::lexer("select");
        let a = lex.next();
        println!("{:?}",a);
        let a = lex.next();
        println!("{:?}",a);

    }

    #[test]
    fn test1() {
        let mut lex = Token::lexer("SELECT `table`.* FROM `table` WHERE `id` = 'secret' and `other` = 'something';");

        loop {
            let token = lex.next();
            if token.is_none(){
                break;
            }
            println!("{:?}",token);
        }
    }
}
