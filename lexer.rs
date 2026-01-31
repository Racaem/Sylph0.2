use logos::{Logos};

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    #[token("def")]
    Def,

    #[token("if")]
    If,

    #[token("return")]
    Return,

    #[token("end")]
    End,

    #[token("out")]
    Out,

    #[token("=")]
    Assign,

    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("<=")]
    Le,

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Ident(String),

    #[regex(r"[0-9]+", |lex| lex.slice().parse::<u64>().unwrap())]
    Number(u64),

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
