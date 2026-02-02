use logos::{Logos};
use crate::types::{IntegerValue, StringValue, Value};

#[derive(Logos, Debug, PartialEq, Eq, Hash, Clone)]
pub enum Token {
    #[token("def")]
    Def,

    #[token("if")]
    If,

    #[token("while")]
    While,

    #[token("return")]
    Return,

    #[token("end")]
    End,

    #[token("out")]
    Out,

    #[token("=")]
    Assign,

    // 复合赋值运算符（必须在单字符运算符之前定义，以确保优先匹配）
    #[token("+=")]
    PlusAssign,

    #[token("-=")]
    MinusAssign,

    #[token("*=")]
    MulAssign,

    #[token("%=")]
    ModAssign,

    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Mul,

    #[token("%")]
    Mod,

    #[token("<")]
    Lt,

    #[token("<=")]
    Le,

    #[token(">" )]
    Gt,

    #[token(">=")]
    Ge,

    #[token("==")]
    Eq,

    #[token(",")]
    Comma,

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Ident(String),

    #[regex(r"[0-9]+", |lex| {
        let value_str = lex.slice();
        if let Ok(value) = value_str.parse::<i8>() {
            IntegerValue::I8(value)
        } else if let Ok(value) = value_str.parse::<i16>() {
            IntegerValue::I16(value)
        } else if let Ok(value) = value_str.parse::<i32>() {
            IntegerValue::I32(value)
        } else if let Ok(value) = value_str.parse::<i64>() {
            IntegerValue::I64(value)
        } else if let Ok(value) = value_str.parse::<i128>() {
            IntegerValue::I128(value)
        } else {
            // 对于超过i128范围的大整数，使用BigInt类型
            IntegerValue::BigInt(num_bigint::BigInt::parse_bytes(value_str.as_bytes(), 10).unwrap())
        }
    })]
    Number(IntegerValue),

    #[regex(r"[0-9]+i8", |lex| {
        let value = lex.slice().trim_end_matches("i8").parse::<i8>().unwrap();
        IntegerValue::I8(value)
    })]
    TypedNumber(IntegerValue),

    #[regex(r"[0-9]+i16", |lex| {
        let value = lex.slice().trim_end_matches("i16").parse::<i16>().unwrap();
        IntegerValue::I16(value)
    })]
    TypedNumber16(IntegerValue),

    #[regex(r"[0-9]+i32", |lex| {
        let value = lex.slice().trim_end_matches("i32").parse::<i32>().unwrap();
        IntegerValue::I32(value)
    })]
    TypedNumber32(IntegerValue),

    #[regex(r"[0-9]+i64", |lex| {
        let value = lex.slice().trim_end_matches("i64").parse::<i64>().unwrap();
        IntegerValue::I64(value)
    })]
    TypedNumber64(IntegerValue),

    #[regex(r"[0-9]+i128", |lex| {
        let value = lex.slice().trim_end_matches("i128").parse::<i128>().unwrap();
        IntegerValue::I128(value)
    })]
    TypedNumber128(IntegerValue),

    #[regex(r"[0-9]+bigint", |lex| {
        let value_str = lex.slice().trim_end_matches("bigint");
        IntegerValue::BigInt(num_bigint::BigInt::parse_bytes(value_str.as_bytes(), 10).unwrap())
    })]
    TypedNumberBigInt(IntegerValue),

    // 暂时注释掉字符串字面量支持，直到正则表达式问题解决
    // #[regex(r"\"([^\"\\]|\\.)*\"")] 
    // String(StringValue),

    #[regex(r"\s+", logos::skip)]
    Whitespace,

    #[regex(r"//.*", logos::skip)]
    Comment,
}

pub fn tokenize(code: &str) -> Result<Vec<Token>, String> {
    let mut lexer = Token::lexer(code);
    let mut tokens = Vec::new();

    while let Some(token) = lexer.next() {
        match token {
            Ok(token) => {
                tokens.push(token);
            }
            Err(_) => {
                let span = lexer.span();
                let error_char = &code[span.clone()];
                return Err(format!("Unexpected character: '{}' at position {}", error_char, span.start));
            }
        }
    }

    Ok(tokens)
}
