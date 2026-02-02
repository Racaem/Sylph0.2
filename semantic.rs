use crate::ast::{Expr, Stmt, Program};
use std::sync::Arc;
use rayon::prelude::*;

#[derive(Debug)]
pub struct SemanticAnalyzer {
    functions: Arc<std::collections::HashMap<String, (Vec<String>, &'static Vec<Stmt>)>>,
    variables: std::collections::HashSet<String>,
    expr_cache: std::collections::HashMap<u64, Result<(), String>>,
}

impl SemanticAnalyzer {
    fn new() -> Self {
        SemanticAnalyzer {
            functions: Arc::new(std::collections::HashMap::new()),
            variables: std::collections::HashSet::new(),
            expr_cache: std::collections::HashMap::new(),
        }
    }

    fn expr_hash(expr: &Expr) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        match expr {
            Expr::Number(n) => {
                "Number".hash(&mut hasher);
                n.hash(&mut hasher);
            }
            Expr::TypedNumber(_) => {
                "TypedNumber".hash(&mut hasher);
            }
            Expr::Ident(name) => {
                "Ident".hash(&mut hasher);
                name.hash(&mut hasher);
            }
            Expr::BinOp(left, op, right) => {
                "BinOp".hash(&mut hasher);
                Self::expr_hash(left).hash(&mut hasher);
                op.hash(&mut hasher);
                Self::expr_hash(right).hash(&mut hasher);
            }
            Expr::Call(name, args) => {
                "Call".hash(&mut hasher);
                name.hash(&mut hasher);
                args.len().hash(&mut hasher);
                for arg in args {
                    Self::expr_hash(arg).hash(&mut hasher);
                }
            }
        }
        hasher.finish()
    }

    fn analyze_expr(&mut self, expr: &Expr) -> Result<(), String> {
        // 检查缓存中是否已有结果
        let cache_key = Self::expr_hash(expr);
        if let Some(result) = self.expr_cache.get(&cache_key) {
            return result.clone();
        }
        
        // 使用函数指针映射进行快速表达式分析
        type ExprAnalyzer = fn(&mut SemanticAnalyzer, &Expr) -> Result<(), String>;
        
        // 静态映射表，只初始化一次
        static EXPR_ANALYZERS: std::sync::OnceLock<std::collections::HashMap<&'static str, ExprAnalyzer>> = std::sync::OnceLock::new();
        
        let map = EXPR_ANALYZERS.get_or_init(|| {
            let mut map = std::collections::HashMap::new();
            map.insert("Number", Self::analyze_number as ExprAnalyzer);
            map.insert("Ident", Self::analyze_ident as ExprAnalyzer);
            map.insert("BinOp", Self::analyze_bin_op as ExprAnalyzer);
            map.insert("Call", Self::analyze_call as ExprAnalyzer);
            map
        });
        
        // 根据表达式类型选择分析函数
        let analyzer = match expr {
            Expr::Number(_) => map.get("Number").unwrap(),
            Expr::TypedNumber(_) => map.get("Number").unwrap(), // 复用 Number 分析函数
            Expr::Ident(_) => map.get("Ident").unwrap(),
            Expr::BinOp(_, _, _) => map.get("BinOp").unwrap(),
            Expr::Call(_, _) => map.get("Call").unwrap(),
        };
        
        let result = analyzer(self, expr);
        
        // 缓存结果
        let cache_key = Self::expr_hash(expr);
        self.expr_cache.insert(cache_key, result.clone());
        result
    }
    
    // 分析数字表达式
    fn analyze_number(&mut self, _expr: &Expr) -> Result<(), String> {
        Ok(())
    }
    
    // 分析标识符表达式
    fn analyze_ident(&mut self, expr: &Expr) -> Result<(), String> {
        if let Expr::Ident(name) = expr {
            if !self.variables.contains(name) && !self.functions.contains_key(name) {
                Err(format!("Undefined variable or function: {}", name))
            } else {
                Ok(())
            }
        } else {
            Err("Expected identifier".to_string())
        }
    }
    
    // 分析二元操作表达式
    fn analyze_bin_op(&mut self, expr: &Expr) -> Result<(), String> {
        if let Expr::BinOp(left, _, right) = expr {
            self.analyze_expr(left)?;
            self.analyze_expr(right)?;
            Ok(())
        } else {
            Err("Expected binary operation".to_string())
        }
    }
    
    // 分析函数调用表达式
    fn analyze_call(&mut self, expr: &Expr) -> Result<(), String> {
        if let Expr::Call(name, args) = expr {
            if !self.functions.contains_key(name) {
                Err(format!("Undefined function: {}", name))
            } else {
                for arg in args {
                    self.analyze_expr(arg)?;
                }
                Ok(())
            }
        } else {
            Err("Expected function call".to_string())
        }
    }

    fn analyze_stmt(&mut self, stmt: &Stmt) -> Result<(), String> {
        // 使用函数指针映射进行快速语句分析
        type StmtAnalyzer = fn(&mut SemanticAnalyzer, &Stmt) -> Result<(), String>;
        
        // 静态映射表，只初始化一次
        static STMT_ANALYZERS: std::sync::OnceLock<std::collections::HashMap<&'static str, StmtAnalyzer>> = std::sync::OnceLock::new();
        
        let map = STMT_ANALYZERS.get_or_init(|| {
            let mut map = std::collections::HashMap::new();
            map.insert("Assign", Self::analyze_assign as StmtAnalyzer);
            map.insert("If", Self::analyze_if as StmtAnalyzer);
            map.insert("While", Self::analyze_while as StmtAnalyzer);
            map.insert("Return", Self::analyze_return as StmtAnalyzer);
            map.insert("Out", Self::analyze_out as StmtAnalyzer);
            map.insert("FuncDef", Self::analyze_func_def as StmtAnalyzer);
            map
        });
        
        // 根据语句类型选择分析函数
        let analyzer = match stmt {
            Stmt::Assign(_, _) => map.get("Assign").unwrap(),
            Stmt::If(_, _) => map.get("If").unwrap(),
            Stmt::While(_, _) => map.get("While").unwrap(),
            Stmt::Return(_) => map.get("Return").unwrap(),
            Stmt::Out(_) => map.get("Out").unwrap(),
            Stmt::FuncDef(_, _, _) => map.get("FuncDef").unwrap(),
        };
        
        analyzer(self, stmt)
    }
    
    // 分析赋值语句
    fn analyze_assign(&mut self, stmt: &Stmt) -> Result<(), String> {
        if let Stmt::Assign(name, expr) = stmt {
            self.analyze_expr(expr)?;
            self.variables.insert(name.clone());
            Ok(())
        } else {
            Err("Expected assignment".to_string())
        }
    }
    
    // 分析if语句
    fn analyze_if(&mut self, stmt: &Stmt) -> Result<(), String> {
        if let Stmt::If(cond, body) = stmt {
            self.analyze_expr(cond)?;
            for stmt in body {
                self.analyze_stmt(stmt)?;
            }
            Ok(())
        } else {
            Err("Expected if statement".to_string())
        }
    }
    
    // 分析while语句
    fn analyze_while(&mut self, stmt: &Stmt) -> Result<(), String> {
        if let Stmt::While(cond, body) = stmt {
            self.analyze_expr(cond)?;
            for stmt in body {
                self.analyze_stmt(stmt)?;
            }
            Ok(())
        } else {
            Err("Expected while statement".to_string())
        }
    }
    
    // 分析return语句
    fn analyze_return(&mut self, stmt: &Stmt) -> Result<(), String> {
        if let Stmt::Return(expr) = stmt {
            self.analyze_expr(expr)?;
            Ok(())
        } else {
            Err("Expected return statement".to_string())
        }
    }
    
    // 分析out语句
    fn analyze_out(&mut self, stmt: &Stmt) -> Result<(), String> {
        if let Stmt::Out(expr) = stmt {
            self.analyze_expr(expr)?;
            Ok(())
        } else {
            Err("Expected out statement".to_string())
        }
    }
    
    // 分析函数定义语句
    fn analyze_func_def(&mut self, stmt: &Stmt) -> Result<(), String> {
        if let Stmt::FuncDef(name, params, body) = stmt {
            if self.functions.contains_key(name) {
                return Err(format!("Function already defined: {}", name));
            }
            // 先注册函数，处理前向引用
            let static_body: &'static Vec<Stmt> = unsafe {
                std::mem::transmute(body)
            };
            let functions_map = Arc::make_mut(&mut self.functions);
            functions_map.insert(name.clone(), (params.clone(), static_body));
            Ok(())
        } else {
            Err("Expected function definition".to_string())
        }
    }

    fn analyze_program(&mut self, program: &Program) -> Result<(), String> {
        // 使用并行分析
        self.analyze_program_parallel(program)
    }

    fn analyze_program_parallel(&mut self, program: &Program) -> Result<(), String> {
        // 第一遍：注册所有函数（顺序执行，处理函数依赖）
        let functions_map = Arc::make_mut(&mut self.functions);
        for stmt in &program.statements {
            if let Stmt::FuncDef(name, params, body) = stmt {
                if !functions_map.contains_key(name) {
                    let static_body: &'static Vec<Stmt> = unsafe {
                        std::mem::transmute(body)
                    };
                    functions_map.insert(name.clone(), (params.clone(), static_body));
                }
            }
        }
        
        // 收集需要分析的函数体
        let mut function_bodies = Vec::new();
        let mut non_function_stmts = Vec::new();
        
        for stmt in &program.statements {
            match stmt {
                Stmt::FuncDef(_name, params, body) => {
                    function_bodies.push((params.clone(), body));
                }
                _ => {
                    non_function_stmts.push(stmt);
                }
            }
        }
        
        // 并行分析函数体（函数体之间是独立的）
        let functions_clone = Arc::clone(&self.functions);
        let analysis_results: Vec<Result<(), String>> = function_bodies
            .par_iter()
            .map(|(params, body)| {
                let mut local_analyzer = SemanticAnalyzer {
                    functions: Arc::clone(&functions_clone),
                    variables: std::collections::HashSet::new(),
                    expr_cache: std::collections::HashMap::new(),
                };
                // 注册所有参数
                for param in params {
                    local_analyzer.variables.insert(param.clone());
                }
                // 分析函数体
                for stmt in *body {
                    if let Err(err) = local_analyzer.analyze_stmt(stmt) {
                        return Err(err);
                    }
                }
                Ok(())
            })
            .collect();
        
        // 检查并行分析的结果
        for result in analysis_results {
            result?;
        }
        
        // 顺序分析非函数语句（保持变量定义顺序）
        for stmt in non_function_stmts {
            self.analyze_stmt(stmt)?;
        }
        
        Ok(())
    }
}

pub fn analyze(program: Program) -> Result<Program, String> {
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze_program(&program)?;
    Ok(program)
}
