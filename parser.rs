use crate::ast::{Expr, BinOpType, Stmt, Program};
use crate::lexer::Token;

// Parser implementation with function identification during parsing
// Changes made to fix function call identification issue:
// 1. Added a `functions` HashSet to track defined functions
// 2. Added a `scan_functions` method to pre-scan and register all function definitions
// 3. Modified `parse_primary` to check if an identifier is a registered function before treating it as a function call
// 4. Modified `parse_ident_stmt` to use the same function checking logic
// This ensures that only actual functions are treated as function calls, preventing incorrect argument parsing
// for non-function identifiers.

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    functions: std::collections::HashSet<String>,
    function_locations: std::collections::HashMap<String, usize>,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        let mut parser = Parser {
            tokens,
            pos: 0,
            functions: std::collections::HashSet::new(),
            function_locations: std::collections::HashMap::new(),
        };
        parser.scan_function_locations();
        parser
    }

    fn scan_function_locations(&mut self) {
        let original_pos = self.pos;
        self.pos = 0;
        
        while self.pos < self.tokens.len() {
            if let Some(Token::Def) = self.tokens.get(self.pos) {
                self.pos += 1;
                if let Some(Token::Ident(name)) = self.tokens.get(self.pos) {
                    self.functions.insert(name.clone());
                    self.function_locations.insert(name.clone(), self.pos - 1);
                    // Skip the rest of the function definition
                    while self.pos < self.tokens.len() {
                        if let Some(Token::End) = self.tokens.get(self.pos) {
                            self.pos += 1;
                            break;
                        }
                        self.pos += 1;
                    }
                }
            } else {
                self.pos += 1;
            }
        }
        
        self.pos = original_pos;
    }

    fn is_function(&self, name: &str) -> bool {
        self.functions.contains(name)
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
    
    fn consume_no_clone(&mut self) {
        if self.pos < self.tokens.len() {
            self.pos += 1;
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
                Token::Mul | Token::Mod => (3, match token {
                    Token::Mul => BinOpType::Mul,
                    Token::Mod => BinOpType::Mod,
                    _ => unreachable!(),
                }),
                Token::Plus | Token::Minus => (2, match token {
                    Token::Plus => BinOpType::Plus,
                    Token::Minus => BinOpType::Minus,
                    _ => unreachable!(),
                }),
                Token::Lt => (1, BinOpType::Lt),
                Token::Le => (1, BinOpType::Le),
                Token::Gt => (1, BinOpType::Gt),
                Token::Ge => (1, BinOpType::Ge),
                Token::Eq => (0, BinOpType::Eq),
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
            Some(Token::TypedNumber(value)) => {
                self.consume();
                Ok(Expr::TypedNumber(value))
            }
            Some(Token::TypedNumber16(value)) => {
                self.consume();
                Ok(Expr::TypedNumber(value))
            }
            Some(Token::TypedNumber32(value)) => {
                self.consume();
                Ok(Expr::TypedNumber(value))
            }
            Some(Token::TypedNumber64(value)) => {
                self.consume();
                Ok(Expr::TypedNumber(value))
            }
            Some(Token::TypedNumber128(value)) => {
                self.consume();
                Ok(Expr::TypedNumber(value))
            }
            Some(Token::TypedNumberBigInt(value)) => {
                self.consume();
                Ok(Expr::TypedNumber(value))
            },
            Some(Token::Ident(name)) => {
                self.consume();
                // 检查是否是函数调用
                if let Some(next_token) = self.peek() {
                    match next_token {
                        Token::Ident(_) => {
                            // 特殊检查：如果标识符后面是赋值操作符，则不是函数参数
                            // 例如: `b = mo` 后面是 `c = 5`，不应将 `c` 作为 `mo` 的参数
                            if self.pos + 1 < self.tokens.len() {
                                if let Some(Token::Assign) = self.tokens.get(self.pos + 1) {
                                    // 下一个标识符后面是 `=`，所以它不是参数，而是新语句
                                    return Ok(Expr::Ident(name));
                                }
                            }
                            // 只有当标识符是已定义的函数时，才视为函数调用
                            if self.is_function(&name) {
                                // 这是一个带参数的函数调用
                                let mut args = Vec::new();
                                let arg = self.parse_expr()?;
                                args.push(arg);
                                // 检查是否有更多参数
                                while let Some(Token::Comma) = self.peek() {
                                    self.consume();
                                    let arg = self.parse_expr()?;
                                    args.push(arg);
                                }
                                Ok(Expr::Call(name, args))
                            } else {
                                // 这只是一个普通的标识符
                                Ok(Expr::Ident(name))
                            }
                        }
                        Token::Number(_) | Token::Minus => {
                            // 只有当标识符是已定义的函数时，才视为函数调用
                            if self.is_function(&name) {
                                // 这是一个带参数的函数调用
                                let mut args = Vec::new();
                                let arg = self.parse_expr()?;
                                args.push(arg);
                                // 检查是否有更多参数
                                while let Some(Token::Comma) = self.peek() {
                                    self.consume();
                                    let arg = self.parse_expr()?;
                                    args.push(arg);
                                }
                                Ok(Expr::Call(name, args))
                            } else {
                                // 这只是一个普通的标识符
                                Ok(Expr::Ident(name))
                            }
                        }
                        _ => {
                            // 这只是一个普通的标识符
                            Ok(Expr::Ident(name))
                        }
                    }
                } else {
                    // 这只是一个普通的标识符
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
        // 使用函数指针映射进行快速token查找
        type StmtParser = fn(&mut Parser) -> Result<Stmt, String>;
        
        // 静态映射表，只初始化一次
        static STMT_PARSERS: std::sync::OnceLock<std::collections::HashMap<Token, StmtParser>> = std::sync::OnceLock::new();
        
        let map = STMT_PARSERS.get_or_init(|| {
            let mut map = std::collections::HashMap::new();
            map.insert(Token::Def, Parser::parse_func_def as StmtParser);
            map.insert(Token::If, Parser::parse_if_stmt as StmtParser);
            map.insert(Token::While, Parser::parse_while_stmt as StmtParser);
            map.insert(Token::Return, Parser::parse_return_stmt as StmtParser);
            map.insert(Token::Out, Parser::parse_out_stmt as StmtParser);
            map.insert(Token::Ident("dummy".to_string()), Parser::parse_ident_stmt as StmtParser);
            map.insert(Token::Minus, Parser::parse_minus_expr as StmtParser);
            map.insert(Token::Number(crate::types::IntegerValue::I8(0)), Parser::parse_number_expr as StmtParser);
            map
        });
        
        let current_token = self.peek().cloned();
        match &current_token {
            Some(token) => {
                // 根据token类型选择解析函数
                match token {
                    Token::Ident(_) => {
                        // 处理标识符特殊情况
                        Self::parse_ident_stmt(self)
                    }
                    Token::Number(_) => {
                        // 处理数字特殊情况
                        Self::parse_number_expr(self)
                    }
                    Token::Minus => {
                        // 处理减号特殊情况
                        Self::parse_minus_expr(self)
                    }
                    _ => {
                        // 使用映射表查找解析函数
                        match token {
                            Token::Def | Token::If | Token::While | Token::Return | Token::Out => {
                                if let Some(parser) = map.get(&token) {
                                    parser(self)
                                } else {
                                    Err(format!("Expected statement, got {:?}", current_token))
                                }
                            }
                            _ => {
                                Err(format!("Expected statement, got {:?}", current_token))
                            }
                        }
                    }
                }
            }
            None => Err("Unexpected end of input".to_string()),
        }
    }
    
    // 解析函数定义
    fn parse_func_def(&mut self) -> Result<Stmt, String> {
        self.consume_no_clone();
        if let Some(Token::Ident(name)) = self.peek() {
            let func_name = name.clone();
            self.consume_no_clone();
            // 解析参数列表
            let mut params = Vec::new();
            while let Some(token) = self.peek() {
                match token {
                    Token::Ident(param) => {
                        params.push(param.clone());
                        self.consume_no_clone();
                        // 检查是否有逗号
                        if let Some(Token::Comma) = self.peek() {
                            self.consume_no_clone();
                        } else {
                            break;
                        }
                    }
                    _ => {
                        break;
                    }
                }
            }
            let mut body = Vec::new();
            while self.pos < self.tokens.len() {
                if let Some(Token::End) = self.peek() {
                    self.consume_no_clone();
                    break;
                }
                match self.parse_stmt() {
                    Ok(stmt) => body.push(stmt),
                    Err(err) => {
                        println!("Warning: {}", err);
                        if self.pos < self.tokens.len() {
                            self.pos += 1;
                        }
                    }
                }
            }
            Ok(Stmt::FuncDef(func_name, params, body))
        } else {
            Err("Expected function name".to_string())
        }
    }
    
    // 解析if语句
    fn parse_if_stmt(&mut self) -> Result<Stmt, String> {
        self.consume_no_clone();
        let cond = self.parse_expr()?;
        let mut body = Vec::new();
        while self.pos < self.tokens.len() {
            if let Some(Token::End) = self.peek() {
                self.consume_no_clone();
                break;
            }
            match self.parse_stmt() {
                Ok(stmt) => body.push(stmt),
                Err(err) => {
                    println!("Warning: {}", err);
                    if self.pos < self.tokens.len() {
                        self.pos += 1;
                    }
                }
            }
        }
        Ok(Stmt::If(cond, body))
    }
    
    // 解析while语句
    fn parse_while_stmt(&mut self) -> Result<Stmt, String> {
        self.consume_no_clone();
        let cond = self.parse_expr()?;
        let mut body = Vec::new();
        while self.pos < self.tokens.len() {
            if let Some(Token::End) = self.peek() {
                self.consume_no_clone();
                break;
            }
            match self.parse_stmt() {
                Ok(stmt) => body.push(stmt),
                Err(err) => {
                    println!("Warning: {}", err);
                    if self.pos < self.tokens.len() {
                        self.pos += 1;
                    }
                }
            }
        }
        Ok(Stmt::While(cond, body))
    }
    
    // 解析return语句
    fn parse_return_stmt(&mut self) -> Result<Stmt, String> {
        self.consume_no_clone();
        let expr = self.parse_expr()?;
        Ok(Stmt::Return(expr))
    }
    
    // 解析out语句
    fn parse_out_stmt(&mut self) -> Result<Stmt, String> {
        self.consume_no_clone();
        let expr = self.parse_expr()?;
        Ok(Stmt::Out(expr))
    }
    
    // 解析标识符语句
    fn parse_ident_stmt(&mut self) -> Result<Stmt, String> {
        if let Some(Token::Ident(name)) = self.peek() {
            let ident = name.clone();
            self.consume();
            
            // 检查是否是赋值或复合赋值
            match self.peek() {
                Some(Token::Assign) => {
                    self.consume();
                    let expr = self.parse_expr()?;
                    Ok(Stmt::Assign(ident, expr))
                }
                Some(Token::PlusAssign) => {
                    self.consume();
                    let right = self.parse_expr()?;
                    let expr = Expr::BinOp(
                        Box::new(Expr::Ident(ident.clone())),
                        BinOpType::Plus,
                        Box::new(right)
                    );
                    Ok(Stmt::Assign(ident, expr))
                }
                Some(Token::MinusAssign) => {
                    self.consume();
                    let right = self.parse_expr()?;
                    let expr = Expr::BinOp(
                        Box::new(Expr::Ident(ident.clone())),
                        BinOpType::Minus,
                        Box::new(right)
                    );
                    Ok(Stmt::Assign(ident, expr))
                }
                Some(Token::MulAssign) => {
                    self.consume();
                    let right = self.parse_expr()?;
                    let expr = Expr::BinOp(
                        Box::new(Expr::Ident(ident.clone())),
                        BinOpType::Mul,
                        Box::new(right)
                    );
                    Ok(Stmt::Assign(ident, expr))
                }
                Some(Token::ModAssign) => {
                    self.consume();
                    let right = self.parse_expr()?;
                    let expr = Expr::BinOp(
                        Box::new(Expr::Ident(ident.clone())),
                        BinOpType::Mod,
                        Box::new(right)
                    );
                    Ok(Stmt::Assign(ident, expr))
                }
                _ => {
                    // 检查是否是函数调用
                    if let Some(token) = self.peek() {
                        match token {
                            Token::Ident(_) | Token::Number(_) | Token::Minus => {
                                // 只有当标识符是已定义的函数时，才视为函数调用
                                if self.is_function(&ident) {
                                    // 这是一个带参数的函数调用
                                    let mut args = Vec::new();
                                    let arg = self.parse_expr()?;
                                    args.push(arg);
                                    // 检查是否有更多参数
                                    while let Some(Token::Comma) = self.peek() {
                                        self.consume();
                                        let arg = self.parse_expr()?;
                                        args.push(arg);
                                    }
                                    let call_expr = Expr::Call(ident, args);
                                    Ok(Stmt::Out(call_expr))
                                } else {
                                    // 这只是一个普通的标识符
                                    let mut left = Expr::Ident(ident);
                                    // 检查是否有二元操作符
                                    while let Some(token) = self.peek() {
                                        match token {
                                            Token::Plus | Token::Minus | Token::Le | Token::Lt => {
                                                let op_type = match token {
                                                    Token::Plus => BinOpType::Plus,
                                                    Token::Minus => BinOpType::Minus,
                                                    Token::Le => BinOpType::Le,
                                                    Token::Lt => BinOpType::Lt,
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
                            _ => {
                                // 检查是否是无参数函数调用
                                // 这里需要特殊处理，因为无参数函数调用在语法上与普通标识符相同
                                // 我们暂时将其视为普通标识符，在语义分析阶段再处理
                                let mut left = Expr::Ident(ident);
                                // 检查是否有二元操作符
                                while let Some(token) = self.peek() {
                                    match token {
                                        Token::Plus | Token::Minus | Token::Le | Token::Lt => {
                                            let op_type = match token {
                                                Token::Plus => BinOpType::Plus,
                                                Token::Minus => BinOpType::Minus,
                                                Token::Le => BinOpType::Le,
                                                Token::Lt => BinOpType::Lt,
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
                    } else {
                        // 只有一个标识符，作为表达式语句
                        Ok(Stmt::Out(Expr::Ident(ident)))
                    }
                }
            }
        } else {
            Err("Expected identifier".to_string())
        }
    }
    
    // 解析以减号开头的表达式
    fn parse_minus_expr(&mut self) -> Result<Stmt, String> {
        self.consume();
        let right = self.parse_primary()?;
        let expr = Expr::BinOp(Box::new(Expr::Number(crate::types::IntegerValue::I8(0))), BinOpType::Minus, Box::new(right));
        Ok(Stmt::Out(expr))
    }
    
    // 解析以数字开头的表达式
    fn parse_number_expr(&mut self) -> Result<Stmt, String> {
        let expr = self.parse_expr()?;
        Ok(Stmt::Out(expr))
    }

    fn parse_program(&mut self) -> Result<Program, String> {
        let mut statements = Vec::new();
        let original_pos = self.pos;

        self.pos = 0;
        while self.pos < self.tokens.len() {
            let current_token = self.peek();
            match current_token {
                Some(Token::Def) => {
                    // 解析函数定义并添加到statements中
                    if let Ok(func_def) = self.parse_func_def() {
                        statements.push(func_def);
                    } else {
                        // 解析失败，跳过当前标记
                        self.pos += 1;
                    }
                }
                Some(_) => {
                    // 解析非函数定义的语句
                    match self.parse_stmt() {
                        Ok(stmt) => {
                            statements.push(stmt);
                        }
                        Err(err) => {
                            println!("Warning: {}", err);
                            if self.pos < self.tokens.len() {
                                self.pos += 1;
                            }
                        }
                    }
                }
                None => {
                    break;
                }
            }
        }

        self.pos = original_pos;
        Ok(Program {
            statements,
        })
    }

    fn parse_function_on_demand(&mut self, name: &str) -> Result<Stmt, String> {
        if let Some(&location) = self.function_locations.get(name) {
            let original_pos = self.pos;
            self.pos = location;
            
            // 解析函数定义
            let result = self.parse_func_def();
            
            self.pos = original_pos;
            result
        } else {
            Err(format!("Function {} not found", name))
        }
    }
}

pub fn parse(tokens: Vec<Token>) -> Result<Program, String> {
    let mut parser = Parser::new(tokens);
    parser.parse_program()
}
