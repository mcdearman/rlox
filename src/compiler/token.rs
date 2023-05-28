use std::{
    fmt::Display,
    ops::{Index, Range},
};

use logos::Logos;

#[derive(Logos, Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Single character tokens
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token(",")]
    Comma,
    #[token(".")]
    Dot,
    #[token("+")]
    Add,
    #[token("-")]
    Sub,
    #[token("*")]
    Mul,
    #[token("/")]
    Div,
    #[token(";")]
    Semicolon,

    // One or two character tokens
    #[token("!")]
    Not,
    #[token("!=")]
    Neq,
    #[token("=")]
    Assign,
    #[token("==")]
    Eq,
    #[token(">")]
    Gt,
    #[token(">=")]
    Gte,
    #[token("<")]
    Lt,
    #[token("<=")]
    Lte,

    // Literals
    #[regex("[0-9]+")]
    Number,
    #[regex(r#""[^"]*""#)]
    String,
    #[regex("[a-zA-Z][a-zA-Z0-9]*")]
    Ident,

    // Keywords
    #[token("and")]
    And,
    #[token("class")]
    Class,
    #[token("else")]
    Else,
    #[token("false")]
    False,
    #[token("for")]
    For,
    #[token("fun")]
    Fun,
    #[token("if")]
    If,
    #[token("nil")]
    Nil,
    #[token("or")]
    Or,
    #[token("print")]
    Print,
    #[token("return")]
    Return,
    #[token("super")]
    Super,
    #[token("this")]
    This,
    #[token("true")]
    True,
    #[token("var")]
    Var,
    #[token("while")]
    While,

    #[error]
    Error,
    Eof,
}

#[macro_export]
macro_rules! T {
    ["("] => {
        TokenKind::LParen
    };
    [")"] => {
        TokenKind::RParen
    };
    ["{"] => {
        TokenKind::LBrace
    };
    ["}"] => {
        TokenKind::RBrace
    };
    [,] => {
        TokenKind::Comma
    };
    ["."] => {
        TokenKind::Dot
    };
    ["+"] => {
        TokenKind::Add
    };
    ["-"] => {
        TokenKind::Sub
    };
    ["*"] => {
        TokenKind::Mul
    };
    ["/"] => {
        TokenKind::Div
    };
    [";"] => {
        TokenKind::Semicolon
    };
    ["!"] => {
        TokenKind::Not
    };
    ["!="] => {
        TokenKind::Neq
    };
    ["="] => {
        TokenKind::Assign
    };
    ["=="] => {
        TokenKind::Eq
    };
    [">"] => {
        TokenKind::Gt
    };
    [">="] => {
        TokenKind::Gte
    };
    ["<"] => {
        TokenKind::Lt
    };
    ["<="] => {
        TokenKind::Lte
    };
    [num] => {
        TokenKind::Number
    };
    [str] => {
        TokenKind::String
    };
    [ident] => {
        TokenKind::Ident
    };
    [and] => {
        TokenKind::And
    };
    [class] => {
        TokenKind::Class
    };
    [else] => {
        TokenKind::Else
    };
    [false] => {
        TokenKind::False
    };
    [for] => {
        TokenKind::For
    };
    [fun] => {
        TokenKind::Fun
    };
    [if] => {
        TokenKind::If
    };
    [nil] => {
        TokenKind::Nil
    };
    [or] => {
        TokenKind::Or
    };
    [print] => {
        TokenKind::Print
    };
    [return] => {
        TokenKind::Return
    };
    [super] => {
        TokenKind::Super
    };
    [this] => {
        TokenKind::This
    };
    [true] => {
        TokenKind::True
    };
    [var] => {
        TokenKind::Var
    };
    [while] => {
        TokenKind::While
    };
    [err] => {
        TokenKind::Error
    };
    [EOF] => {
        TokenKind::Eof
    };
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            T!["("] => write!(f, "("),
            T![")"] => write!(f, ")"),
            T!["{"] => write!(f, "{{"),
            T!["}"] => write!(f, "}}"),
            T![,] => write!(f, ","),
            T!["."] => write!(f, "."),
            T!["+"] => write!(f, "+"),
            T!["-"] => write!(f, "-"),
            T!["*"] => write!(f, "*"),
            T!["/"] => write!(f, "/"),
            T![";"] => write!(f, ";"),
            T!["!"] => write!(f, "!"),
            T!["!="] => write!(f, "!="),
            T!["="] => write!(f, "="),
            T!["=="] => write!(f, "=="),
            T![">"] => write!(f, ">"),
            T![">="] => write!(f, ">="),
            T!["<"] => write!(f, "<"),
            T!["<="] => write!(f, "<="),
            T![num] => write!(f, "Number"),
            T![str] => write!(f, "String"),
            T![ident] => write!(f, "Ident"),
            T![and] => write!(f, "and"),
            T![class] => write!(f, "class"),
            T![else] => write!(f, "else"),
            T![false] => write!(f, "false"),
            T![for] => write!(f, "for"),
            T![fun] => write!(f, "fun"),
            T![if] => write!(f, "if"),
            T![nil] => write!(f, "nil"),
            T![or] => write!(f, "or"),
            T![print] => write!(f, "print"),
            T![return] => write!(f, "return"),
            T![super] => write!(f, "super"),
            T![this] => write!(f, "this"),
            T![true] => write!(f, "true"),
            T![var] => write!(f, "var"),
            T![while] => write!(f, "while"),
            T![err] => write!(f, "Error"),
            T![EOF] => write!(f, "EOF"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Span {
    pub start: u32,
    pub end: u32,
}

impl Span {
    pub fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }
}

impl From<Range<usize>> for Span {
    fn from(range: Range<usize>) -> Self {
        Self {
            start: range.start as u32,
            end: range.end as u32,
        }
    }
}

impl From<Span> for Range<usize> {
    fn from(span: Span) -> Self {
        Self {
            start: span.start as usize,
            end: span.end as usize,
        }
    }
}

impl Index<Span> for str {
    type Output = str;

    fn index(&self, index: Span) -> &Self::Output {
        &self[Range::<usize>::from(index)]
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }

    pub fn lit<'a>(&self, src: &'a str) -> &'a str {
        &src[self.span]
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {:?}", self.kind, self.span)
    }
}
