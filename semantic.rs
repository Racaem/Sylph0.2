use crate::ast::{Expr, Stmt, Program};

#[derive(Debug)]
pub struct SemanticAnalyzer {
    functions: std::collections::HashMap<String, (String, &'static Vec<Stmt>)>,
    variables: std::collections::HashSet<String>,
}

impl SemanticAnalyzer {
    fn new() -> Self {
        SemanticAnalyzer {
            functions: std::collections::HashMap::new(),
            variables: std::collections::HashSet::new(),
        }
    }

    fn analyze_expr(&mut self, expr: &Expr) -> Result<(), String> {
        match expr {
            Expr::Number(_) => Ok(()),
            Expr::Ident(name) => {
                if !self.variables.contains(name) && !self.functions.contains_key(name) {
                    Err(format!("Undefined variable or function: {}", name))
                } else {
                    Ok(())
                }
            }
            Expr::BinOp(left, _, right) => {
                self.analyze_expr(left)?;
                self.analyze_expr(right)?;
                Ok(())
            }
            Expr::Call(name, args) => {
                if !self.functions.contains_key(name) {
                    Err(format!("Undefined function: {}", name))
                } else {
                    for arg in args {
                        self.analyze_expr(arg)?;
                    }
                    Ok(())
                }
            }
        }
    }

    fn analyze_stmt(&mut self, stmt: &Stmt) -> Result<(), String> {
        match stmt {
            Stmt::Assign(name, expr) => {
                self.analyze_expr(expr)?;
                self.variables.insert(name.clone());
                Ok(())
            }
            Stmt::If(cond, body) => {
                self.analyze_expr(cond)?;
                for stmt in body {
                    self.analyze_stmt(stmt)?;
                }
                Ok(())
            }
            Stmt::Return(expr) => {
                self.analyze_expr(expr)?;
                Ok(())
            }
            Stmt::Out(expr) => {
                self.analyze_expr(expr)?;
                Ok(())
            }
            Stmt::FuncDef(name, param, body) => {
                if self.functions.contains_key(name) {
                    return Err(format!("Function already defined: {}", name));
                }
                // 使用 unsafe 来处理生命周期，实际项目中应使用更安全的方式
                let static_body: &'static Vec<Stmt> = unsafe {
                    std::mem::transmute(body)
                };
                self.functions.insert(name.clone(), (param.clone(), static_body));
                // 分析函数体
                let mut local_analyzer = SemanticAnalyzer {
                    functions: self.functions.clone(),
                    variables: std::collections::HashSet::new(),
                };
                local_analyzer.variables.insert(param.clone());
                for stmt in body {
                    local_analyzer.analyze_stmt(stmt)?;
                }
                Ok(())
            }
        }
    }

    fn analyze_program(&mut self, program: &Program) -> Result<(), String> {
        for stmt in &program.statements {
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
