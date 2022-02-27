use crate::keyword::Keyword;

#[derive(Debug, PartialEq)]
pub enum Token {
    Keyword(Keyword),
    Identifier,
    Constant,
    Operators, //操作符
    Special,   //特殊字符
    Comments,  //注释
}

#[derive(Debug, PartialEq)]
pub enum Constant {
    Numeric,
}
