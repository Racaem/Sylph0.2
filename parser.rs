use crate::ast::{Expr, BinOpType, Stmt, Program};
use crate::lexer::Token;

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            pos: 0,
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn consume(&mut self) -> Option<Token> {
        if self.pos < self.tokens.len() {
            let token = self.tokens[self.pos].clone();
            self.pos += 1;
            Some(token)
        } else {
            None
        }
    }

    fn expect(&mut self, expected: Token) -> Result<(), String> {
        if let Some(token) = self.peek() {
            if token == &expected {
                self.consume();
                Ok(())
            } else {
                Err(format!("Expected {:?}, got {:?}", expected, token))
            }
        } else {
            Err("Unexpected end of input".to_string())
        }
    }

    fn parse_expr(&mut self) -> Result<Expr, String> {
        self.parse_bin_op(0)
    }

    fn parse_bin_op(&mut self, precedence: u32) -> Result<Expr, String> {
        let mut left = self.parse_primary()?;

        while let Some(token) = self.peek() {
            let (op_prec, op_type) = match token {
                Token::Plus => (1, BinOpType::Plus),
                Token::Minus => (1, BinOpType::Minus),
                Token::Le => (2, BinOpType::Le),
                _ => break,
            };

            if op_prec <= precedence {
                break;
            }

            self.consume();
            let right = self.parse_bin_op(op_prec)?;
            left = Expr::BinOp(Box::new(left), op_type, Box::new(right));
        }

        Ok(left)
    }

    fn parse_primary(&mut self) -> Result<Expr, String> {
        let token = self.tokens.get(self.pos).cloned();
        match token {
            Some(Token::Number(n)) => {
                self.consume();
                Ok(Expr::Number(n))
            }
            Some(Token::Ident(name)) => {
                self.consume();
                if let Some(Token::Minus) = self.peek() {
                    self.consume();
                    let right = self.parse_primary()?;
                    Ok(Expr::BinOp(Box::new(Expr::Ident(name)), BinOpType::Minus, Box::new(right)))
                } else {
                    Ok(Expr::Ident(name))
                }
            }
            _ => Err(format!("Expected primary expression, got {:?}", token)),
        }
    }

    fn parse_call(&mut self, name: String) -> Result<Expr, String> {
        let mut args = Vec::new();
        // 尝试解析参数表达式
        let arg = self.parse_expr()?;
        args.push(arg);
        Ok(Expr::Call(name, args))
    }

    fn parse_stmt(&mut self) -> Result<Stmt, String> {
        match self.peek() {
            Some(Token::Def) => {
                self.consume();
                if let Some(Token::Ident(name)) = self.peek() {
                    let func_name = name.clone();
                    self.consume();
                    if let Some(Token::Ident(param)) = self.peek() {
                        let param_name = param.clone();
                        self.consume();
                        let mut body = Vec::new();
                        while let Some(token) = self.peek() {
                            if token == &Token::End {
                                self.consume();
                                break;
                            }
                            body.push(self.parse_stmt()?);
                        }
                        Ok(Stmt::FuncDef(func_name, param_name, body))
                    } else {
                        Err("Expected function parameter".to_string())
                    }
                } else {
                    Err("Expected function name".to_string())
                }
            }
            Some(Token::If) => {
                self.consume();
                let cond = self.parse_expr()?;
                let mut body = Vec::new();
                while let Some(token) = self.peek() {
                    if token == &Token::End {
                        self.consume();
                        break;
                    }
                    body.push(self.parse_stmt()?);
                }
                Ok(Stmt::If(cond, body))
            }
            Some(Token::Return) => {
                self.consume();
                let expr = self.parse_expr()?;
                Ok(Stmt::Return(expr))
            }
            Some(Token::Out) => {
                self.consume();
                let expr = self.parse_expr()?;
                Ok(Stmt::Out(expr))
            }
            Some(Token::Ident(name)) => {
                let ident = name.clone();
                self.consume();
                if let Some(Token::Assign) = self.peek() {
                    self.consume();
                    let expr = self.parse_expr()?;
                    Ok(Stmt::Assign(ident, expr))
                } else {
                    // 这是一个表达式语句，尝试解析完整的表达式
                    let mut left = Expr::Ident(ident);
                    // 检查是否有二元操作符
                    while let Some(token) = self.peek() {
                        match token {
                            Token::Plus | Token::Minus | Token::Le => {
                                let op_type = match token {
                                    Token::Plus => BinOpType::Plus,
                                    Token::Minus => BinOpType::Minus,
                                    Token::Le => BinOpType::Le,
                                    _ => unreachable!(),
                                };
                                self.consume();
                                let right = self.parse_primary()?;
                                left = Expr::BinOp(Box::new(left), op_type, Box::new(right));
                            }
                            _ => break,
                        }
                    }
                    Ok(Stmt::Out(left))
                }
            }
            Some(Token::Minus) => {
                // 处理以减号开头的表达式
                self.consume();
                let right = self.parse_primary()?;
                let expr = Expr::BinOp(Box::new(Expr::Number(0)), BinOpType::Minus, Box::new(right));
                Ok(Stmt::Out(expr))
            }
            Some(Token::Number(_)) => {
                // 处理以数字开头的表达式
                let expr = self.parse_expr()?;
                Ok(Stmt::Out(expr))
            }
            Some(Token::End) => {
                // 跳过End token，它应该由调用者处理
                self.consume();
                Err("Unexpected End token".to_string())
            }
            _ => Err(format!("Expected statement, got {:?}", self.peek())),
        }
    }

    fn parse_program(&mut self) -> Result<Program, String> {
        let mut statements = Vec::new();

        while self.pos < self.tokens.len() {
            statements.push(self.parse_stmt()?);
        }

        Ok(Program {
            statements,
        })
    }
}

pub fn parse(tokens: Vec<Token>) -> Result<Program, String> {
    let mut parser = Parser::new(tokens);
    parser.parse_program()
}
