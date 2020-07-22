use crate::ident::Ident;
use crate::lit::{Lit, TyLit};
use crate::span::Span;
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TokenWithSpan {
    pub span: Span,
    pub token: Token,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Token {
    Eof,
    Not,
    NotEq,
    AndAnd,
    LeftParen,
    RightParen,
    Star,
    StarEq,
    Plus,
    PlusEq,
    Comma,
    Minus,
    MinusEq,
    Arrow,
    Dot,
    Slash,
    SlashEq,
    Colon,
    PathSep,
    Semi,
    Lt,
    LtEq,
    Eq,
    EqEq,
    Gt,
    GtEq,
    Question,
    Attribute,
    Break,
    Const,
    Continue,
    Else,
    For,
    Fn,
    From,
    If,
    Impl,
    In,
    Inout,
    Instance,
    Let,
    Return,
    Self_,
    Struct,
    Texture,
    To,
    Varying,
    Uniform,
    LeftBracket,
    RightBracket,
    LeftBrace,
    OrOr,
    RightBrace,
    Ident(Ident),
    Lit(Lit),
    TyLit(TyLit),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Eof => write!(f, "<eof>"),
            Token::Not => write!(f, "!"),
            Token::NotEq => write!(f, "!="),
            Token::AndAnd => write!(f, "&&"),
            Token::LeftParen => write!(f, "("),
            Token::RightParen => write!(f, ")"),
            Token::Star => write!(f, "*"),
            Token::StarEq => write!(f, "*="),
            Token::Plus => write!(f, "+"),
            Token::PlusEq => write!(f, "+="),
            Token::Comma => write!(f, ","),
            Token::Minus => write!(f, "-"),
            Token::Arrow => write!(f, "=>"),
            Token::Dot => write!(f, "."),
            Token::MinusEq => write!(f, "-="),
            Token::Slash => write!(f, "/"),
            Token::SlashEq => write!(f, "/="),
            Token::Colon => write!(f, ":"),
            Token::PathSep => write!(f, ":"),
            Token::Semi => write!(f, ";"),
            Token::Lt => write!(f, "<"),
            Token::LtEq => write!(f, "<="),
            Token::Eq => write!(f, "="),
            Token::EqEq => write!(f, "=="),
            Token::Gt => write!(f, ">"),
            Token::GtEq => write!(f, ">="),
            Token::Question => write!(f, "?"),
            Token::Attribute => write!(f, "attribute"),
            Token::Break => write!(f, "break"),
            Token::Const => write!(f, "const"),
            Token::Continue => write!(f, "continue"),
            Token::Else => write!(f, "else"),
            Token::Fn => write!(f, "fn"),
            Token::For => write!(f, "for"),
            Token::From => write!(f, "from"),
            Token::If => write!(f, "if"),
            Token::Impl => write!(f, "impl"),
            Token::In => write!(f, "in"),
            Token::Inout => write!(f, "inout"),
            Token::Instance => write!(f, "instance"),
            Token::Let => write!(f, "let"),
            Token::Return => write!(f, "return"),
            Token::Self_ => write!(f, "self"),
            Token::Struct => write!(f, "struct"),
            Token::Texture => write!(f, "texture"),
            Token::To => write!(f, "to"),
            Token::Uniform => write!(f, "uniform"),
            Token::Varying => write!(f, "varying"),
            Token::LeftBracket => write!(f, "["),
            Token::RightBracket => write!(f, "]"),
            Token::LeftBrace => write!(f, "{{"),
            Token::OrOr => write!(f, "||"),
            Token::RightBrace => write!(f, "}}"),
            Token::Ident(ident) => write!(f, "{}", ident),
            Token::Lit(lit) => write!(f, "{}", lit),
            Token::TyLit(ty_lit) => write!(f, "{}", ty_lit),
        }
    }
}
