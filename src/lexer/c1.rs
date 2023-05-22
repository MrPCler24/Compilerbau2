use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum C1Token {
    // TODO: Define variants and their token/regex

    //SW
    #[token("bool")]
    KwBoolean,

    #[token("do")]
    KwDo,

    #[token("else")]
    KwElse,

    #[token("float")]
    KwFloat,

    #[token("for")]
    KwFor,

    #[token("if")]
    KwIf,
    
    #[token("int")]
    KwInt,
    
    #[token("printf")]
    KwPrintf,
    
    #[token("return")]
    KwReturn,
    
    #[token("void")]
    KwVoid,
    
    #[token("while")]
    KwWhile,
    
    //OP
    #[token("+")]
    Plus,
    
    #[token("-")]
    Minus,
    
    #[token("*")]
    Asterisk,
    
    #[token("/")]
    Slash,
    
    #[token("=")]
    Assign,
    
    #[token("==")]
    Eq,
    
    #[token("!=")]
    Neq,
    
    #[token("<")]
    Lss,
    
    #[token(">")]
    Grt,
    
    #[token("<=")]
    Leq,
    
    #[token("=>")]
    Geq,
    
    #[token("&&")]
    And,
    
    #[token("||")]
    Or,
    
    //TOK
    #[token(",")]
    Comma,
    
    #[token(";")]
    Semicolon,

    #[token("(")]
    LParen,
    
    #[token(")")]
    RParen,
    
    #[token("{")]
    LBrace,
    
    #[token("}")]
    RBrace,
    
    //TERMV
    #[regex(r#"[0-9]+"#)]
    ConstInt,

    #[regex(r#"(((([0-9]+\.[0-9]+)|(\.[0-9]+))((e|E)(-|\+)?[0-9]+)?)|([0-9]+(e|E)(-|\+)?[0-9]+))"#)]
    ConstFloat,

    #[regex(r#"(true)|(false)"#)]
    ConstBoolean,

    #[regex(r#""[^\n"]*""#)]
    ConstString,

    #[regex(r#"[a-zA-Z]+([0-9]|[a-zA-Z])*"#)]
    Id,

    //SKP
    #[regex(r#"[\s]+"#, logos::skip)]
    SkipWhite,

    #[regex(r#"/\*([^*]|(\*[^/]))*\*/"#, logos::skip)]
    SkipCComm,

    #[regex(r#"//[^\n]*"#, logos::skip)]
    SkipCPPComm,

    // Logos requires one token variant to handle errors,
    // it can be named anything you wish.
    #[error]
    Error,
}
