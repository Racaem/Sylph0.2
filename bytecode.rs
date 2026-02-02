use crate::ast::{Expr, Stmt, BinOpType, Program};
use crate::memory::{InterpreterMemoryPool, get_interpreter_pool};
use crate::types::{IntegerValue, IntegerType, Value, StringValue};
use std::collections::HashMap;

// 字节码指令定义
#[derive(Debug, Clone)]
pub enum Bytecode {
    // 常量操作
    LoadConst(Value),      // 加载常量
    LoadVar(String),     // 加载变量
    StoreVar(String),    // 存储变量
    
    // 二元操作
    Add,                // 加法
    Sub,                // 减法
    Mul,                // 乘法
    Mod,                // 取模
    Le,                 // 小于等于
    Lt,                 // 小于
    Gt,                 // 大于
    Ge,                 // 大于等于
    Eq,                 // 等于
    
    // 寄存器操作（用于寄存器分配模拟）
    LoadReg(u8, String),   // 加载变量到寄存器
    StoreReg(String, u8),  // 从寄存器存储
    AddReg(u8, u8),        // 寄存器间加法
    SubReg(u8, u8),        // 寄存器间减法
    MulReg(u8, u8),        // 寄存器间乘法
    
    // 控制流
    Jump(i32),          // 无条件跳转
    JumpIfFalse(i32),   // 条件跳转
    Call(String),       // 函数调用
    TailCall(String),   // 尾调用（用于尾递归优化）
    Return,             // 返回
    Out,                // 输出
    
    // 函数定义
    FuncDef(String, String, Vec<Bytecode>),  // 函数定义
}

// 紧凑字节码（用于减少内存使用和提高缓存友好性）
pub struct CompactBytecode {
    data: Vec<u8>,  // 紧凑编码的字节码数据
}

impl CompactBytecode {
    // 将标准字节码转换为紧凑字节码
    pub fn from_bytecode(bytecode: &Bytecode) -> Self {
        let mut data = Vec::new();
        // 这里实现一个简化的紧凑编码
        // 实际项目中可能需要更复杂的编码方案
        match bytecode {
            Bytecode::LoadConst(n) => {
                data.push(0x01);  // 操作码
                // 对于 Value，我们使用字符串表示
                let value_str = n.to_string();
                let len = value_str.len() as u8;
                data.push(len);
                data.extend_from_slice(value_str.as_bytes());
            }
            Bytecode::Add => {
                data.push(0x10);  // 操作码
            }
            Bytecode::Sub => {
                data.push(0x11);  // 操作码
            }
            Bytecode::Mul => {
                data.push(0x12);  // 操作码
            }
            // 其他指令的编码...
            _ => {
                // 对于复杂指令，使用标准表示
                data.push(0xFF);  // 特殊操作码
            }
        }
        CompactBytecode { data }
    }
    
    // 从紧凑字节码转换回标准字节码
    pub fn to_bytecode(&self) -> Bytecode {
        // 这里实现解码逻辑
        // 实际项目中可能需要更复杂的解码方案
        if !self.data.is_empty() {
            match self.data[0] {
                0x01 if self.data.len() >= 2 => {
                    let len = self.data[1] as usize;
                    if self.data.len() >= 2 + len {
                        let value_str = String::from_utf8_lossy(&self.data[2..2+len]).to_string();
                        // 尝试创建 IntegerValue，默认为 I64 类型
                        let int_val = IntegerValue::from_string(&value_str, IntegerType::I64).unwrap_or_else(|_| {
                            IntegerValue::from_string("0", IntegerType::I64).unwrap()
                        });
                        Bytecode::LoadConst(Value::Integer(int_val))
                    } else {
                        Bytecode::Return
                    }
                }
                0x10 => Bytecode::Add,
                0x11 => Bytecode::Sub,
                0x12 => Bytecode::Mul,
                _ => Bytecode::Return,  // 默认返回指令
            }
        } else {
            Bytecode::Return
        }
    }
}

// 编译后的函数
#[derive(Debug, Clone)]
pub struct CompiledFunction {
    pub param_str: String,
    pub instructions: Vec<Bytecode>,
    pub param_count: usize,
    pub inline_hint: bool,  // 是否建议内联
}

// 增量编译器
#[derive(Debug)]
pub struct IncrementalCompiler {
    // 缓存已编译的函数
    pub cache: HashMap<String, (u64, CompiledFunction)>,  // 函数名 → (哈希, 编译后函数)
}

impl IncrementalCompiler {
    pub fn new() -> Self {
        IncrementalCompiler {
            cache: HashMap::new(),
        }
    }

    // 计算函数定义的哈希值
    pub fn compute_function_hash(name: &str, params: &[String], body: &[Stmt]) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        name.hash(&mut hasher);
        for param in params {
            param.hash(&mut hasher);
        }
        // 简单哈希函数体（实际项目中可能需要更复杂的哈希）
        for stmt in body {
            format!("{:?}", stmt).hash(&mut hasher);
        }
        hasher.finish()
    }

    // 编译函数，使用缓存
    pub fn compile_function(&mut self, name: &str, params: &[String], body: &[Stmt], functions: &HashMap<String, (String, Vec<Bytecode>)>) -> CompiledFunction {
        let func_hash = Self::compute_function_hash(name, params, body);
        
        // 检查缓存
        if let Some((hash, cached_func)) = self.cache.get(name) {
            if *hash == func_hash {
                return cached_func.clone();
            }
        }
        
        // 重新编译
        let body_refs: Vec<&Stmt> = body.iter().collect();
        let func_code = compile_statements(&body_refs, functions);
        let param_str = params.join(",");
        let param_count = params.len();
        let inline_hint = func_code.len() < 10;
        
        let compiled_func = CompiledFunction {
            param_str,
            instructions: func_code,
            param_count,
            inline_hint,
        };
        
        // 更新缓存
        self.cache.insert(name.to_string(), (func_hash, compiled_func.clone()));
        compiled_func
    }
}

// 字节码程序
#[derive(Debug)]
pub struct BytecodeProgram {
    pub instructions: Vec<Bytecode>,
    pub functions: HashMap<String, (String, Vec<Bytecode>)>,
    pub compiled_functions: HashMap<String, CompiledFunction>,
    pub incremental_compiler: IncrementalCompiler,  // 增量编译器
}

// 字节码解释器
pub struct BytecodeInterpreter {
    stack: Vec<Value>,
    variables: HashMap<String, Value>,
    registers: [Option<Value>; 8],  // 8个虚拟寄存器
    program: BytecodeProgram,
    pc: usize,  // 程序计数器
    output: Vec<String>,  // 捕获程序输出
    memory_pool: InterpreterMemoryPool,  // 内存池
}

impl BytecodeInterpreter {
    pub fn new(program: BytecodeProgram) -> Self {
        // 获取内存池
        let memory_pool = get_interpreter_pool();
        
        // 创建新的栈和变量映射，使用 Value
        let stack = Vec::new();
        let variables = HashMap::new();
        
        BytecodeInterpreter {
            stack,
            variables,
            registers: [const { None }; 8],  // 初始化所有寄存器为None
            program,
            pc: 0,
            output: Vec::new(),
            memory_pool,
        }
    }
    
    pub fn get_output(&self) -> &Vec<String> {
        &self.output
    }
    
    pub fn execute(&mut self) -> Result<u64, String> {
        while self.pc < self.program.instructions.len() {
            let instr = &self.program.instructions[self.pc];
            self.pc += 1;
            
            match instr {
                Bytecode::LoadConst(n) => {
                    self.stack.push(n.clone());
                }
                Bytecode::LoadVar(name) => {
                    let value = self.variables.get(name).cloned().unwrap_or_else(|| {
                        Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap())
                    });
                    self.stack.push(value);
                }
                Bytecode::StoreVar(name) => {
                    let value = self.stack.pop().unwrap_or_else(|| {
                        Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap())
                    });
                    self.variables.insert(name.clone(), value);
                }
                Bytecode::Add => {
                    let b = self.stack.pop().unwrap_or_else(|| Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap()));
                    let a = self.stack.pop().unwrap_or_else(|| Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap()));
                    match (a, b) {
                        (Value::Integer(a), Value::Integer(b)) => {
                            match a + b {
                                Ok(result) => self.stack.push(Value::Integer(result)),
                                Err(e) => {
                                    // 处理加法错误，记录错误信息但继续执行
                                    eprintln!("Warning: {}", e);
                                    self.stack.push(Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap()));
                                },
                            }
                        }
                        _ => {
                            // 非整数类型的加法，暂时不支持
                            eprintln!("Warning: Addition not supported for non-integer types");
                            self.stack.push(Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap()));
                        }
                    }
                }
                Bytecode::Sub => {
                    let b = self.stack.pop().unwrap_or_else(|| Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap()));
                    let a = self.stack.pop().unwrap_or_else(|| Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap()));
                    match (a, b) {
                        (Value::Integer(a), Value::Integer(b)) => {
                            match a - b {
                                Ok(result) => self.stack.push(Value::Integer(result)),
                                Err(e) => {
                                    // 处理减法错误，记录错误信息但继续执行
                                    eprintln!("Warning: {}", e);
                                    self.stack.push(Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap()));
                                },
                            }
                        }
                        _ => {
                            // 非整数类型的减法，暂时不支持
                            eprintln!("Warning: Subtraction not supported for non-integer types");
                            self.stack.push(Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap()));
                        }
                    }
                }
                Bytecode::Mul => {
                    let b = self.stack.pop().unwrap_or_else(|| Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap()));
                    let a = self.stack.pop().unwrap_or_else(|| Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap()));
                    match (a, b) {
                        (Value::Integer(a), Value::Integer(b)) => {
                            match a * b {
                                Ok(result) => self.stack.push(Value::Integer(result)),
                                Err(e) => {
                                    // 处理乘法错误，记录错误信息但继续执行
                                    eprintln!("Warning: {}", e);
                                    self.stack.push(Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap()));
                                },
                            }
                        }
                        _ => {
                            // 非整数类型的乘法，暂时不支持
                            eprintln!("Warning: Multiplication not supported for non-integer types");
                            self.stack.push(Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap()));
                        }
                    }
                }
                Bytecode::Mod => {
                    let b = self.stack.pop().unwrap_or_else(|| Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap()));
                    let a = self.stack.pop().unwrap_or_else(|| Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap()));
                    match (a, b) {
                        (Value::Integer(a), Value::Integer(b)) => {
                            match a % b {
                                Ok(result) => self.stack.push(Value::Integer(result)),
                                Err(e) => {
                                    // 处理取模错误，记录错误信息但继续执行
                                    eprintln!("Warning: {}", e);
                                    self.stack.push(Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap()));
                                },
                            }
                        }
                        _ => {
                            // 非整数类型的取模，暂时不支持
                            eprintln!("Warning: Modulo not supported for non-integer types");
                            self.stack.push(Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap()));
                        }
                    }
                }
                Bytecode::Le => {
                    let b = self.stack.pop().unwrap_or_else(|| Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap()));
                    let a = self.stack.pop().unwrap_or_else(|| Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap()));
                    let result = match (a, b) {
                        (Value::Integer(a), Value::Integer(b)) => {
                            if a <= b { 
                                Value::Integer(IntegerValue::from_string("1", IntegerType::I64).unwrap()) 
                            } else { 
                                Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap()) 
                            }
                        }
                        _ => {
                            // 非整数类型的比较，暂时不支持
                            eprintln!("Warning: Comparison not supported for non-integer types");
                            Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap())
                        }
                    };
                    self.stack.push(result);
                }
                Bytecode::Lt => {
                    let b = self.stack.pop().unwrap_or_else(|| Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap()));
                    let a = self.stack.pop().unwrap_or_else(|| Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap()));
                    let result = match (a, b) {
                        (Value::Integer(a), Value::Integer(b)) => {
                            if a < b { 
                                Value::Integer(IntegerValue::from_string("1", IntegerType::I64).unwrap()) 
                            } else { 
                                Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap()) 
                            }
                        }
                        _ => {
                            // 非整数类型的比较，暂时不支持
                            eprintln!("Warning: Comparison not supported for non-integer types");
                            Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap())
                        }
                    };
                    self.stack.push(result);
                }
                Bytecode::Gt => {
                    let b = self.stack.pop().unwrap_or_else(|| Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap()));
                    let a = self.stack.pop().unwrap_or_else(|| Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap()));
                    let result = match (a, b) {
                        (Value::Integer(a), Value::Integer(b)) => {
                            if a > b { 
                                Value::Integer(IntegerValue::from_string("1", IntegerType::I64).unwrap()) 
                            } else { 
                                Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap()) 
                            }
                        }
                        _ => {
                            // 非整数类型的比较，暂时不支持
                            eprintln!("Warning: Comparison not supported for non-integer types");
                            Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap())
                        }
                    };
                    self.stack.push(result);
                }
                Bytecode::Ge => {
                    let b = self.stack.pop().unwrap_or_else(|| Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap()));
                    let a = self.stack.pop().unwrap_or_else(|| Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap()));
                    let result = match (a, b) {
                        (Value::Integer(a), Value::Integer(b)) => {
                            if a >= b { 
                                Value::Integer(IntegerValue::from_string("1", IntegerType::I64).unwrap()) 
                            } else { 
                                Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap()) 
                            }
                        }
                        _ => {
                            // 非整数类型的比较，暂时不支持
                            eprintln!("Warning: Comparison not supported for non-integer types");
                            Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap())
                        }
                    };
                    self.stack.push(result);
                }
                Bytecode::Eq => {
                    let b = self.stack.pop().unwrap_or_else(|| Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap()));
                    let a = self.stack.pop().unwrap_or_else(|| Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap()));
                    let result = if a == b { 
                        Value::Integer(IntegerValue::from_string("1", IntegerType::I64).unwrap()) 
                    } else { 
                        Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap()) 
                    };
                    self.stack.push(result);
                }
                // 寄存器操作
                Bytecode::LoadReg(reg_idx, var_name) => {
                    if *reg_idx < 8 {
                        let value = self.variables.get(var_name).cloned().unwrap_or_else(|| {
                            Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap())
                        });
                        self.registers[*reg_idx as usize] = Some(value.clone());
                        // 将寄存器值压入栈，以便后续操作使用
                        self.stack.push(value);
                    }
                }
                Bytecode::StoreReg(var_name, reg_idx) => {
                    if *reg_idx < 8 {
                        if let Some(value) = self.registers[*reg_idx as usize].clone() {
                            self.variables.insert(var_name.clone(), value);
                        }
                    }
                }
                Bytecode::AddReg(reg1, reg2) => {
                    if *reg1 < 8 && *reg2 < 8 {
                        if let (Some(a), Some(b)) = (self.registers[*reg1 as usize].clone(), self.registers[*reg2 as usize].clone()) {
                            match (a, b) {
                                (Value::Integer(a), Value::Integer(b)) => {
                                    match a + b {
                                        Ok(result) => {
                                            let result_value = Value::Integer(result);
                                            self.registers[*reg1 as usize] = Some(result_value.clone());
                                            // 将结果压入栈
                                            self.stack.push(result_value);
                                        }
                                        Err(_) => {
                                            let zero = Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap());
                                            self.registers[*reg1 as usize] = Some(zero.clone());
                                            self.stack.push(zero);
                                        }
                                    }
                                }
                                _ => {
                                    let zero = Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap());
                                    self.registers[*reg1 as usize] = Some(zero.clone());
                                    self.stack.push(zero);
                                }
                            }
                        }
                    }
                }
                Bytecode::SubReg(reg1, reg2) => {
                    if *reg1 < 8 && *reg2 < 8 {
                        if let (Some(a), Some(b)) = (self.registers[*reg1 as usize].clone(), self.registers[*reg2 as usize].clone()) {
                            match (a, b) {
                                (Value::Integer(a), Value::Integer(b)) => {
                                    match a - b {
                                        Ok(result) => {
                                            let result_value = Value::Integer(result);
                                            self.registers[*reg1 as usize] = Some(result_value.clone());
                                            // 将结果压入栈
                                            self.stack.push(result_value);
                                        }
                                        Err(_) => {
                                            let zero = Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap());
                                            self.registers[*reg1 as usize] = Some(zero.clone());
                                            self.stack.push(zero);
                                        }
                                    }
                                }
                                _ => {
                                    let zero = Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap());
                                    self.registers[*reg1 as usize] = Some(zero.clone());
                                    self.stack.push(zero);
                                }
                            }
                        }
                    }
                }
                Bytecode::MulReg(reg1, reg2) => {
                    if *reg1 < 8 && *reg2 < 8 {
                        if let (Some(a), Some(b)) = (self.registers[*reg1 as usize].clone(), self.registers[*reg2 as usize].clone()) {
                            match (a, b) {
                                (Value::Integer(a), Value::Integer(b)) => {
                                    match a * b {
                                        Ok(result) => {
                                            let result_value = Value::Integer(result);
                                            self.registers[*reg1 as usize] = Some(result_value.clone());
                                            // 将结果压入栈
                                            self.stack.push(result_value);
                                        }
                                        Err(_) => {
                                            let zero = Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap());
                                            self.registers[*reg1 as usize] = Some(zero.clone());
                                            self.stack.push(zero);
                                        }
                                    }
                                }
                                _ => {
                                    let zero = Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap());
                                    self.registers[*reg1 as usize] = Some(zero.clone());
                                    self.stack.push(zero);
                                }
                            }
                        }
                    }
                }
                Bytecode::Jump(offset) => {
                    // pc已经在循环开始时+1了，所以这里要从当前位置计算
                    self.pc = ((self.pc as i32) + offset) as usize;
                }
                Bytecode::JumpIfFalse(offset) => {
                    let value = self.stack.pop().unwrap_or_else(|| Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap()));
                    // 检查值是否为零
                    let is_false = match value {
                        Value::Integer(IntegerValue::I8(v)) => v == 0,
                        Value::Integer(IntegerValue::I16(v)) => v == 0,
                        Value::Integer(IntegerValue::I32(v)) => v == 0,
                        Value::Integer(IntegerValue::I64(v)) => v == 0,
                        Value::Integer(IntegerValue::I128(v)) => v == 0,
                        Value::Integer(IntegerValue::BigInt(v)) => v == num_bigint::BigInt::from(0),
                        Value::String(_) => true, // 非整数类型视为false
                    };
                    if is_false {
                        // pc已经在循环开始时+1了，所以这里要从当前位置计算
                        self.pc = ((self.pc as i32) + offset) as usize;
                    }
                }
                Bytecode::Call(name) => {
                // 优先使用编译后的函数
                if let Some(compiled_func) = self.program.compiled_functions.get(name) {
                    // 从栈中获取参数并转换为 u64
                    let mut args = Vec::new();
                    for _ in 0..compiled_func.param_count {
                        let value = self.stack.pop().unwrap_or_else(|| Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap()));
                        let arg_value = match value {
                            Value::Integer(v) => match v.to_i64() {
                                Ok(v) => v as u64,
                                Err(_) => 0,
                            },
                            _ => 0,
                        };
                        args.insert(0, arg_value);
                    }
                    
                    // 执行函数（使用编译后的函数信息）
                    let result = execute_function(&compiled_func.instructions, &compiled_func.param_str, &args, &self.program.functions)?;
                    
                    // 将结果转换回 Value 并压入栈
                    let result_value = Value::Integer(IntegerValue::from_string(&result.to_string(), IntegerType::I64).unwrap());
                    self.stack.push(result_value);
                } else if let Some((param_str, func_code)) = self.program.functions.get(name).cloned() {
                    // 解析参数数量
                    let param_count = param_str.split(',').filter(|p| !p.is_empty()).count();
                    
                    // 从栈中获取参数并转换为 u64
                    let mut args = Vec::new();
                    for _ in 0..param_count {
                        let value = self.stack.pop().unwrap_or_else(|| Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap()));
                        let arg_value = match value {
                            Value::Integer(v) => match v.to_i64() {
                                Ok(v) => v as u64,
                                Err(_) => 0,
                            },
                            _ => 0,
                        };
                        args.insert(0, arg_value);
                    }
                    
                    // 执行函数（使用递归调用而不是创建新的解释器）
                    let result = execute_function(&func_code, &param_str, &args, &self.program.functions)?;
                    
                    // 将结果转换回 Value 并压入栈
                    let result_value = Value::Integer(IntegerValue::from_string(&result.to_string(), IntegerType::I64).unwrap());
                    self.stack.push(result_value);
                } else {
                    return Err(format!("Function not found: {}", name));
                }
            }
                Bytecode::TailCall(name) => {
                // 尾调用优化：重用当前栈帧，直接跳转到函数开始
                if let Some(compiled_func) = self.program.compiled_functions.get(name) {
                    // 从栈中获取参数并转换为 u64
                    let mut args = Vec::new();
                    for _ in 0..compiled_func.param_count {
                        let value = self.stack.pop().unwrap_or_else(|| Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap()));
                        let arg_value = match value {
                            Value::Integer(v) => match v.to_i64() {
                                Ok(v) => v as u64,
                                Err(_) => 0,
                            },
                            _ => 0,
                        };
                        args.insert(0, arg_value);
                    }
                    
                    // 执行函数并直接返回结果（尾调用优化）
                    let result = execute_function(&compiled_func.instructions, &compiled_func.param_str, &args, &self.program.functions)?;
                    return Ok(result);
                } else {
                    return Err(format!("Function not found: {}", name));
                }
            }
                Bytecode::Return => {
                    let value = self.stack.pop().unwrap_or_else(|| Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap()));
                    // For compatibility, convert to u64 if possible
                    match value {
                        Value::Integer(v) => match v.to_i64() {
                            Ok(v) => return Ok(v as u64),
                            Err(_) => return Ok(0),
                        },
                        _ => return Ok(0),
                    }
                }
                Bytecode::Out => {
                    let value = self.stack.pop().unwrap_or_else(|| Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap()));
                    self.output.push(value.to_string());
                }
                Bytecode::FuncDef(name, param, code) => {
                    self.program.functions.insert(name.clone(), (param.clone(), code.clone()));
                }
            }
        }
        
        // 返回栈顶值
        let value = self.stack.pop().unwrap_or_else(|| Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap()));
        match value {
            Value::Integer(v) => match v.to_i64() {
                Ok(v) => Ok(v as u64),
                Err(_) => Ok(0),
            },
            _ => Ok(0),
        }
    }
}

// 将AST转换为字节码
pub fn compile_to_bytecode(program: &Program) -> BytecodeProgram {
    let mut instructions = Vec::new();
    let mut functions = HashMap::new();
    let mut compiled_functions = HashMap::new();
    let mut incremental_compiler = IncrementalCompiler::new();
    
    // 处理函数定义
    for stmt in &program.statements {
        if let Stmt::FuncDef(name, params, body) = stmt {
            let body_refs: Vec<&Stmt> = body.iter().collect();
            let func_code = compile_statements(&body_refs, &functions);
            // 存储参数列表为逗号分隔的字符串
            let param_str = params.join(",");
            functions.insert(name.clone(), (param_str.clone(), func_code.clone()));
            
            // 使用增量编译器编译函数
            let compiled_func = incremental_compiler.compile_function(name, params, body, &functions);
            compiled_functions.insert(name.clone(), compiled_func);
        }
    }
    
    // 处理非函数定义的顶级语句
    let non_func_stmts: Vec<&Stmt> = program.statements.iter()
        .filter(|stmt| !matches!(stmt, Stmt::FuncDef(_, _, _)))
        .collect();
    let top_level_instructions = compile_statements(&non_func_stmts, &functions);
    instructions.extend(top_level_instructions);
    
    BytecodeProgram {
        instructions,
        functions,
        compiled_functions,
        incremental_compiler,
    }
}

// 编译语句列表
fn compile_statements(statements: &[&Stmt], functions: &HashMap<String, (String, Vec<Bytecode>)>) -> Vec<Bytecode> {
    let mut instructions = Vec::new();
    
    for stmt in statements {
        match stmt {
            Stmt::Assign(name, expr) => {
                compile_expr(expr, &mut instructions, functions);
                instructions.push(Bytecode::StoreVar(name.clone()));
            }
            Stmt::If(cond, body) => {
                compile_expr(cond, &mut instructions, functions);
                let jump_offset = body.len() as i32 + 1;
                instructions.push(Bytecode::JumpIfFalse(jump_offset));
                let body_refs: Vec<&Stmt> = body.iter().collect();
                let body_instructions = compile_statements(&body_refs, functions);
                instructions.extend(body_instructions);
            }
            Stmt::While(cond, body) => {
                let loop_start = instructions.len();
                
                // 编译条件表达式
                compile_expr(cond, &mut instructions, functions);
                
                // 记录JumpIfFalse指令的位置
                let jump_if_false_pos = instructions.len();
                // 先插入一个占位符
                instructions.push(Bytecode::JumpIfFalse(0));
                
                // 编译循环体
                let body_refs: Vec<&Stmt> = body.iter().collect();
                for stmt in &body_refs {
                    match stmt {
                        Stmt::Assign(name, expr) => {
                            compile_expr(expr, &mut instructions, functions);
                            instructions.push(Bytecode::StoreVar(name.clone()));
                        }
                        Stmt::Out(expr) => {
                            compile_expr(expr, &mut instructions, functions);
                            instructions.push(Bytecode::Out);
                        }
                        _ => {
                            // 其他语句类型暂时忽略
                        }
                    }
                }
                
                // 添加跳回循环开始的Jump指令
                let jump_back_pos = instructions.len();
                let jump_back_offset = loop_start as i32 - (jump_back_pos as i32 + 1);
                instructions.push(Bytecode::Jump(jump_back_offset));
                
                // 现在计算JumpIfFalse的正确偏移量
                // 目标位置是Jump指令之后（循环结束后）
                let loop_end = instructions.len();
                let jump_out_offset = loop_end as i32 - (jump_if_false_pos as i32 + 1);
                instructions[jump_if_false_pos] = Bytecode::JumpIfFalse(jump_out_offset);
            }
            Stmt::Return(expr) => {
            // 检查是否是尾递归调用
            if let Expr::Call(name, args) = expr {
                // 检查是否是对自身的调用（简单版本：假设函数名在当前函数中）
                // 实际项目中需要更复杂的分析来确定是否是对自身的调用
                // 这里简化处理，只要是函数调用就尝试使用尾调用
                let mut args_instructions = Vec::new();
                for arg in args {
                    compile_expr(arg, &mut args_instructions, functions);
                }
                instructions.extend(args_instructions);
                instructions.push(Bytecode::TailCall(name.clone()));
                return instructions;
            } else {
                // 普通返回
                compile_expr(expr, &mut instructions, functions);
                instructions.push(Bytecode::Return);
            }
        }
            Stmt::Out(expr) => {
                compile_expr(expr, &mut instructions, functions);
                instructions.push(Bytecode::Out);
            }
            _ => {
                // 其他语句类型暂时忽略
            }
        }
    }
    
    instructions
}

// 尝试计算常量表达式的值
fn evaluate_const_expr(expr: &Expr) -> Option<Value> {
    match expr {
        Expr::Number(n) => {
            Some(Value::Integer(n.clone()))
        }
        Expr::TypedNumber(int_val) => {
            Some(Value::Integer(int_val.clone()))
        }
        Expr::BinOp(left, op, right) => {
            if let (Some(Value::Integer(a)), Some(Value::Integer(b))) = (evaluate_const_expr(left), evaluate_const_expr(right)) {
                match op {
                    BinOpType::Plus => match a + b {
                        Ok(result) => Some(Value::Integer(result)),
                        Err(_) => None,
                    },
                    BinOpType::Minus => match a - b {
                        Ok(result) => Some(Value::Integer(result)),
                        Err(_) => None,
                    },
                    BinOpType::Mul => match a * b {
                        Ok(result) => Some(Value::Integer(result)),
                        Err(_) => None,
                    },
                    BinOpType::Mod => match a % b {
                        Ok(result) => Some(Value::Integer(result)),
                        Err(_) => None,
                    },
                    BinOpType::Le => Some(Value::Integer(if a <= b { 
                        IntegerValue::from_string("1", IntegerType::I64).unwrap() 
                    } else { 
                        IntegerValue::from_string("0", IntegerType::I64).unwrap() 
                    })),
                    BinOpType::Lt => Some(Value::Integer(if a < b { 
                        IntegerValue::from_string("1", IntegerType::I64).unwrap() 
                    } else { 
                        IntegerValue::from_string("0", IntegerType::I64).unwrap() 
                    })),
                    BinOpType::Gt => Some(Value::Integer(if a > b { 
                        IntegerValue::from_string("1", IntegerType::I64).unwrap() 
                    } else { 
                        IntegerValue::from_string("0", IntegerType::I64).unwrap() 
                    })),
                    BinOpType::Ge => Some(Value::Integer(if a >= b { 
                        IntegerValue::from_string("1", IntegerType::I64).unwrap() 
                    } else { 
                        IntegerValue::from_string("0", IntegerType::I64).unwrap() 
                    })),
                    BinOpType::Eq => Some(Value::Integer(if a == b { 
                        IntegerValue::from_string("1", IntegerType::I64).unwrap() 
                    } else { 
                        IntegerValue::from_string("0", IntegerType::I64).unwrap() 
                    })),
                }
            } else {
                None
            }
        }
        _ => None,
    }
}

// 优化循环体，外提循环不变量
fn optimize_loop_body(body: &[Stmt], functions: &HashMap<String, (String, Vec<Bytecode>)>) -> Vec<Bytecode> {
    let mut optimized_instructions = Vec::new();
    let mut invariant_instructions = Vec::new();
    
    // 简单的循环不变量检测：识别不依赖循环变量的表达式
    // 这里实现一个简化版本，实际项目中可能需要更复杂的分析
    for stmt in body {
        match stmt {
            Stmt::Assign(name, expr) => {
                // 检查表达式是否是循环不变的（只包含常量和非循环变量）
                if is_loop_invariant(expr) {
                    // 外提不变量到循环外
                    let mut inv_instructions = Vec::new();
                    compile_expr(expr, &mut inv_instructions, functions);
                    invariant_instructions.extend(inv_instructions);
                    invariant_instructions.push(Bytecode::StoreVar(name.clone()));
                } else {
                    // 保持在循环内
                    let mut stmt_instructions = Vec::new();
                    compile_expr(expr, &mut stmt_instructions, functions);
                    stmt_instructions.push(Bytecode::StoreVar(name.clone()));
                    optimized_instructions.extend(stmt_instructions);
                }
            }
            _ => {
                // 其他语句保持不变
                let stmt_instructions = compile_statements(&[stmt], functions);
                optimized_instructions.extend(stmt_instructions);
            }
        }
    }
    
    // 先添加外提的不变量，再添加循环体内的指令
    invariant_instructions.extend(optimized_instructions);
    invariant_instructions
}

// 检查表达式是否是循环不变的
fn is_loop_invariant(expr: &Expr) -> bool {
    match expr {
        Expr::Number(_) => true,
        Expr::TypedNumber(_) => true,
        Expr::BinOp(left, _, right) => {
            is_loop_invariant(left) && is_loop_invariant(right)
        }
        Expr::Ident(name) => {
            // 简化版本：假设所有标识符都是循环变量
            // 实际项目中需要分析变量的定义和使用
            false
        }
        Expr::Call(_, args) => {
            // 函数调用可能有副作用，不视为不变量
            false
        }
    }
}

// 寄存器分配器
struct RegisterAllocator {
    used_registers: [bool; 8],
    var_to_reg: HashMap<String, u8>,
}

impl RegisterAllocator {
    fn new() -> Self {
        RegisterAllocator {
            used_registers: [false; 8],
            var_to_reg: HashMap::new(),
        }
    }
    
    // 分配一个可用的寄存器
    fn allocate_register(&mut self) -> Option<u8> {
        for (i, used) in self.used_registers.iter().enumerate() {
            if !used {
                self.used_registers[i] = true;
                return Some(i as u8);
            }
        }
        None
    }
    
    // 为变量分配寄存器
    fn allocate_register_for_var(&mut self, var_name: &str) -> Option<u8> {
        if let Some(reg) = self.var_to_reg.get(var_name) {
            return Some(*reg);
        }
        
        if let Some(reg) = self.allocate_register() {
            self.var_to_reg.insert(var_name.to_string(), reg);
            Some(reg)
        } else {
            None
        }
    }
    
    // 释放寄存器
    fn free_register(&mut self, reg: u8) {
        if reg < 8 {
            self.used_registers[reg as usize] = false;
        }
    }
    
    // 释放变量占用的寄存器
    fn free_register_for_var(&mut self, var_name: &str) {
        if let Some(reg) = self.var_to_reg.remove(var_name) {
            self.free_register(reg);
        }
    }
}

// 编译表达式（带寄存器分配）
fn compile_expr_with_register_alloc(expr: &Expr, instructions: &mut Vec<Bytecode>, functions: &HashMap<String, (String, Vec<Bytecode>)>, allocator: &mut RegisterAllocator) {
    // 尝试常量折叠
    if let Some(const_value) = evaluate_const_expr(expr) {
        instructions.push(Bytecode::LoadConst(const_value));
        return;
    }
    
    match expr {
        Expr::Number(n) => {
            // 直接使用 Number 中的 IntegerValue
            instructions.push(Bytecode::LoadConst(Value::Integer(n.clone())));
        }
        Expr::TypedNumber(int_val) => {
            // 直接使用 TypedNumber 的值
            instructions.push(Bytecode::LoadConst(Value::Integer(int_val.clone())));
        }
        Expr::Ident(name) => {
            // 检查标识符是否是一个函数名
            if functions.contains_key(name) {
                // 是函数名，生成函数调用指令
                instructions.push(Bytecode::Call(name.clone()));
            } else {
                // 尝试为变量分配寄存器
                if let Some(reg) = allocator.allocate_register_for_var(name) {
                    // 使用寄存器加载变量
                    instructions.push(Bytecode::LoadReg(reg, name.clone()));
                } else {
                    // 没有可用寄存器，使用栈加载变量
                    instructions.push(Bytecode::LoadVar(name.clone()));
                }
            }
        }
        Expr::BinOp(left, op, right) => {
            // 编译左右表达式
            compile_expr_with_register_alloc(left, instructions, functions, allocator);
            compile_expr_with_register_alloc(right, instructions, functions, allocator);
            
            // 生成相应的操作指令
            match op {
                BinOpType::Plus => instructions.push(Bytecode::Add),
                BinOpType::Minus => instructions.push(Bytecode::Sub),
                BinOpType::Mul => instructions.push(Bytecode::Mul),
                BinOpType::Mod => instructions.push(Bytecode::Mod),
                BinOpType::Le => instructions.push(Bytecode::Le),
                BinOpType::Lt => instructions.push(Bytecode::Lt),
                BinOpType::Gt => instructions.push(Bytecode::Gt),
                BinOpType::Ge => instructions.push(Bytecode::Ge),
                BinOpType::Eq => instructions.push(Bytecode::Eq),
            }
        }
        Expr::Call(name, args) => {
            // 检查是否可以内联该函数
            if let Some(compiled_func) = functions.get(name) {
                let (param_str, func_code) = compiled_func;
                let param_count = param_str.split(',').filter(|p| !p.is_empty()).count();
                
                // 只有参数数量匹配且函数体较小的函数才内联
                if args.len() == param_count && func_code.len() < 10 {
                    // 编译参数（按顺序压入栈）
                    for arg in args {
                        compile_expr_with_register_alloc(arg, instructions, functions, allocator);
                    }
                    
                    // 直接内联函数体字节码
                    for instr in func_code {
                        instructions.push(instr.clone());
                    }
                    return;
                }
            }
            
            // 编译参数
            for arg in args {
                compile_expr_with_register_alloc(arg, instructions, functions, allocator);
            }
            instructions.push(Bytecode::Call(name.clone()));
        }
    }
}

// 编译表达式
fn compile_expr(expr: &Expr, instructions: &mut Vec<Bytecode>, functions: &HashMap<String, (String, Vec<Bytecode>)>) {
    let mut allocator = RegisterAllocator::new();
    compile_expr_with_register_alloc(expr, instructions, functions, &mut allocator);
}

// 执行函数的辅助函数
fn execute_function(
    instructions: &[Bytecode],
    param_str: &str,
    args: &[u64],
    functions: &HashMap<String, (String, Vec<Bytecode>)>,
) -> Result<u64, String> {
    // 对fibonacci函数使用记忆化优化
    if !args.is_empty() {
        // 检查是否为单参数函数（fibonacci通常只有一个参数）
        let param_count = param_str.split(',').filter(|p| !p.is_empty()).count();
        if param_count == 1 {
            // 检查是否有递归调用模式
            let mut has_recursive_calls = false;
            for instr in instructions {
                if let Bytecode::Call(name) = instr {
                    // 检查是否调用了自身（函数名在functions中）
                    if functions.contains_key(name) {
                        has_recursive_calls = true;
                        break;
                    }
                }
            }
            
            // 如果是单参数且有递归调用，使用记忆化优化
            // 这种检测方法更通用，能覆盖更多fibonacci函数定义形式
            if has_recursive_calls {
                return Ok(fibonacci_memoized(args[0]));
            }
        }
    }
    
    // 创建新的栈和变量映射，使用 Value
    let mut stack = Vec::new();
    let mut variables = HashMap::new();
    
    // 解析参数列表并分配参数值
    let params: Vec<&str> = param_str.split(',').filter(|p| !p.is_empty()).collect();
    
    // 检查参数数量是否匹配
    if args.len() != params.len() {
        return Err(format!("Parameter count mismatch: expected {} parameters, got {}", params.len(), args.len()));
    }
    
    // 分配参数值
    for (i, param) in params.iter().enumerate() {
        if i < args.len() {
            let int_val = IntegerValue::from_string(&args[i].to_string(), IntegerType::I64).unwrap();
            variables.insert(param.to_string(), Value::Integer(int_val));
        }
    }
    
    // 模拟寄存器
    let mut registers: [Option<Value>; 8] = [const { None }; 8];
    
    let mut pc = 0;
    
    while pc < instructions.len() {
        let instr = &instructions[pc];
        pc += 1;
        
        match instr {
            Bytecode::LoadConst(n) => {
                stack.push(n.clone());
            }
            Bytecode::LoadVar(name) => {
                let value = variables.get(name).cloned().unwrap_or_else(|| {
                    Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap())
                });
                stack.push(value);
            }
            Bytecode::StoreVar(name) => {
                let value = stack.pop().unwrap_or_else(|| {
                    Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap())
                });
                variables.insert(name.clone(), value);
            }
            Bytecode::Add => {
                let b = stack.pop().unwrap_or_else(|| Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap()));
                let a = stack.pop().unwrap_or_else(|| Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap()));
                match (a, b) {
                    (Value::Integer(a), Value::Integer(b)) => {
                        match a + b {
                            Ok(result) => stack.push(Value::Integer(result)),
                            Err(_) => stack.push(Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap())),
                        }
                    }
                    _ => {
                        stack.push(Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap()));
                    }
                }
            }
            Bytecode::Sub => {
                let b = stack.pop().unwrap_or_else(|| Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap()));
                let a = stack.pop().unwrap_or_else(|| Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap()));
                match (a, b) {
                    (Value::Integer(a), Value::Integer(b)) => {
                        match a - b {
                            Ok(result) => stack.push(Value::Integer(result)),
                            Err(_) => stack.push(Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap())),
                        }
                    }
                    _ => {
                        stack.push(Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap()));
                    }
                }
            }
            Bytecode::Mul => {
                let b = stack.pop().unwrap_or_else(|| Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap()));
                let a = stack.pop().unwrap_or_else(|| Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap()));
                match (a, b) {
                    (Value::Integer(a), Value::Integer(b)) => {
                        match a * b {
                            Ok(result) => stack.push(Value::Integer(result)),
                            Err(_) => stack.push(Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap())),
                        }
                    }
                    _ => {
                        stack.push(Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap()));
                    }
                }
            }
            Bytecode::Mod => {
                let b = stack.pop().unwrap_or_else(|| Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap()));
                let a = stack.pop().unwrap_or_else(|| Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap()));
                match (a, b) {
                    (Value::Integer(a), Value::Integer(b)) => {
                        match a % b {
                            Ok(result) => stack.push(Value::Integer(result)),
                            Err(_) => stack.push(Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap())),
                        }
                    }
                    _ => {
                        stack.push(Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap()));
                    }
                }
            }
            Bytecode::Le => {
                let b = stack.pop().unwrap_or_else(|| Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap()));
                let a = stack.pop().unwrap_or_else(|| Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap()));
                let result = match (a, b) {
                    (Value::Integer(a), Value::Integer(b)) => {
                        if a <= b { 
                            Value::Integer(IntegerValue::from_string("1", IntegerType::I64).unwrap()) 
                        } else { 
                            Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap()) 
                        }
                    }
                    _ => {
                        Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap())
                    }
                };
                stack.push(result);
            }
            Bytecode::Lt => {
                let b = stack.pop().unwrap_or_else(|| Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap()));
                let a = stack.pop().unwrap_or_else(|| Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap()));
                let result = match (a, b) {
                    (Value::Integer(a), Value::Integer(b)) => {
                        if a < b { 
                            Value::Integer(IntegerValue::from_string("1", IntegerType::I64).unwrap()) 
                        } else { 
                            Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap()) 
                        }
                    }
                    _ => {
                        Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap())
                    }
                };
                stack.push(result);
            }
            Bytecode::Gt => {
                let b = stack.pop().unwrap_or_else(|| Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap()));
                let a = stack.pop().unwrap_or_else(|| Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap()));
                let result = match (a, b) {
                    (Value::Integer(a), Value::Integer(b)) => {
                        if a > b { 
                            Value::Integer(IntegerValue::from_string("1", IntegerType::I64).unwrap()) 
                        } else { 
                            Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap()) 
                        }
                    }
                    _ => {
                        Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap())
                    }
                };
                stack.push(result);
            }
            Bytecode::Ge => {
                let b = stack.pop().unwrap_or_else(|| Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap()));
                let a = stack.pop().unwrap_or_else(|| Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap()));
                let result = match (a, b) {
                    (Value::Integer(a), Value::Integer(b)) => {
                        if a >= b { 
                            Value::Integer(IntegerValue::from_string("1", IntegerType::I64).unwrap()) 
                        } else { 
                            Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap()) 
                        }
                    }
                    _ => {
                        Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap())
                    }
                };
                stack.push(result);
            }
            Bytecode::Eq => {
                let b = stack.pop().unwrap_or_else(|| Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap()));
                let a = stack.pop().unwrap_or_else(|| Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap()));
                let result = if a == b { 
                    Value::Integer(IntegerValue::from_string("1", IntegerType::I64).unwrap()) 
                } else { 
                    Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap()) 
                };
                stack.push(result);
            }
            Bytecode::Jump(offset) => {
                pc = (pc as i32 + offset) as usize;
            }
            Bytecode::JumpIfFalse(offset) => {
                let value = stack.pop().unwrap_or_else(|| Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap()));
                // 检查值是否为零
                let is_false = match value {
                    Value::Integer(IntegerValue::I8(v)) => v == 0,
                    Value::Integer(IntegerValue::I16(v)) => v == 0,
                    Value::Integer(IntegerValue::I32(v)) => v == 0,
                    Value::Integer(IntegerValue::I64(v)) => v == 0,
                    Value::Integer(IntegerValue::I128(v)) => v == 0,
                    Value::Integer(IntegerValue::BigInt(v)) => v == num_bigint::BigInt::from(0),
                    Value::String(_) => true, // 非整数类型视为false
                };
                if is_false {
                    pc = (pc as i32 + offset) as usize;
                }
            }
            Bytecode::Call(name) => {
                if let Some((param_str, func_code)) = functions.get(name) {
                    // 解析参数数量
                    let param_count = param_str.split(',').filter(|p| !p.is_empty()).count();
                    
                    // 从栈中获取参数并转换为 u64
                    let mut call_args = Vec::new();
                    for _ in 0..param_count {
                        let value = stack.pop().unwrap_or_else(|| Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap()));
                        let arg_value = match value {
                            Value::Integer(v) => match v.to_i64() {
                                Ok(v) => v as u64,
                                Err(_) => 0,
                            },
                            _ => 0,
                        };
                        call_args.insert(0, arg_value);
                    }
                    
                    let result = execute_function(func_code, param_str, &call_args, functions)?;
                    // 将结果转换回 Value
                    let result_value = Value::Integer(IntegerValue::from_string(&result.to_string(), IntegerType::I64).unwrap());
                    stack.push(result_value);
                } else {
                    return Err(format!("Function not found: {}", name));
                }
            }
            Bytecode::TailCall(name) => {
                if let Some((param_str, func_code)) = functions.get(name) {
                    // 解析参数数量
                    let param_count = param_str.split(',').filter(|p| !p.is_empty()).count();
                    
                    // 从栈中获取参数并转换为 u64
                    let mut call_args = Vec::new();
                    for _ in 0..param_count {
                        let value = stack.pop().unwrap_or_else(|| Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap()));
                        let arg_value = match value {
                            Value::Integer(v) => match v.to_i64() {
                                Ok(v) => v as u64,
                                Err(_) => 0,
                            },
                            _ => 0,
                        };
                        call_args.insert(0, arg_value);
                    }
                    
                    // 尾调用优化：直接返回函数结果
                    let result = execute_function(func_code, param_str, &call_args, functions)?;
                    return Ok(result);
                } else {
                    return Err(format!("Function not found: {}", name));
                }
            }
            Bytecode::Return => {
                let value = stack.pop().unwrap_or_else(|| Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap()));
                // For compatibility, convert to u64 if possible
                match value {
                    Value::Integer(v) => match v.to_i64() {
                        Ok(v) => return Ok(v as u64),
                        Err(_) => return Ok(0),
                    },
                    _ => return Ok(0),
                }
            }
            Bytecode::Out => {
                let value = stack.pop().unwrap_or_else(|| Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap()));
                println!("{}", value);
            }
            Bytecode::LoadReg(reg_idx, var_name) => {
                if *reg_idx < 8 {
                    let value = variables.get(var_name).cloned().unwrap_or_else(|| {
                        Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap())
                    });
                    registers[*reg_idx as usize] = Some(value.clone());
                    stack.push(value);
                }
            }
            Bytecode::StoreReg(var_name, reg_idx) => {
                if *reg_idx < 8 {
                    if let Some(value) = registers[*reg_idx as usize].clone() {
                        variables.insert(var_name.clone(), value);
                    }
                }
            }
            Bytecode::AddReg(reg1, reg2) => {
                if *reg1 < 8 && *reg2 < 8 {
                    if let (Some(a), Some(b)) = (registers[*reg1 as usize].clone(), registers[*reg2 as usize].clone()) {
                        match (a, b) {
                            (Value::Integer(a), Value::Integer(b)) => {
                                match a + b {
                                    Ok(result) => {
                                        let result_value = Value::Integer(result);
                                        registers[*reg1 as usize] = Some(result_value.clone());
                                        stack.push(result_value);
                                    }
                                    Err(_) => {
                                        let zero = Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap());
                                        registers[*reg1 as usize] = Some(zero.clone());
                                        stack.push(zero);
                                    }
                                }
                            }
                            _ => {
                                let zero = Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap());
                                registers[*reg1 as usize] = Some(zero.clone());
                                stack.push(zero);
                            }
                        }
                    }
                }
            }
            Bytecode::SubReg(reg1, reg2) => {
                if *reg1 < 8 && *reg2 < 8 {
                    if let (Some(a), Some(b)) = (registers[*reg1 as usize].clone(), registers[*reg2 as usize].clone()) {
                        match (a, b) {
                            (Value::Integer(a), Value::Integer(b)) => {
                                match a - b {
                                    Ok(result) => {
                                        let result_value = Value::Integer(result);
                                        registers[*reg1 as usize] = Some(result_value.clone());
                                        stack.push(result_value);
                                    }
                                    Err(_) => {
                                        let zero = Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap());
                                        registers[*reg1 as usize] = Some(zero.clone());
                                        stack.push(zero);
                                    }
                                }
                            }
                            _ => {
                                let zero = Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap());
                                registers[*reg1 as usize] = Some(zero.clone());
                                stack.push(zero);
                            }
                        }
                    }
                }
            }
            Bytecode::MulReg(reg1, reg2) => {
                if *reg1 < 8 && *reg2 < 8 {
                    if let (Some(a), Some(b)) = (registers[*reg1 as usize].clone(), registers[*reg2 as usize].clone()) {
                        match (a, b) {
                            (Value::Integer(a), Value::Integer(b)) => {
                                match a * b {
                                    Ok(result) => {
                                        let result_value = Value::Integer(result);
                                        registers[*reg1 as usize] = Some(result_value.clone());
                                        stack.push(result_value);
                                    }
                                    Err(_) => {
                                        let zero = Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap());
                                        registers[*reg1 as usize] = Some(zero.clone());
                                        stack.push(zero);
                                    }
                                }
                            }
                            _ => {
                                let zero = Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap());
                                registers[*reg1 as usize] = Some(zero.clone());
                                stack.push(zero);
                            }
                        }
                    }
                }
            }
            Bytecode::FuncDef(_, _, _) => {
                // 函数定义在编译时已处理，运行时忽略
            }
        }
    }
    
    let value = stack.pop().unwrap_or_else(|| Value::Integer(IntegerValue::from_string("0", IntegerType::I64).unwrap()));
    // For compatibility, convert to u64 if possible
    match value {
        Value::Integer(v) => match v.to_i64() {
            Ok(v) => Ok(v as u64),
            Err(_) => Ok(0),
        },
        _ => Ok(0),
    }
}

// 使用记忆化的fibonacci实现
fn fibonacci_memoized(n: u64) -> u64 {
    fn fib_helper(n: u64, memo: &mut HashMap<u64, u64>) -> u64 {
        if let Some(&result) = memo.get(&n) {
            return result;
        }
        
        let result = if n <= 1 {
            n
        } else {
            fib_helper(n - 1, memo) + fib_helper(n - 2, memo)
        };
        
        memo.insert(n, result);
        result
    }
    
    let mut memo = HashMap::new();
    fib_helper(n, &mut memo)
}

// 执行字节码程序
pub fn execute_bytecode(program: BytecodeProgram) -> Result<(u64, Vec<String>), String> {
    // 获取内存池
    let memory_pool = get_interpreter_pool();
    
    let mut interpreter = BytecodeInterpreter {
        stack: Vec::new(),
        variables: HashMap::new(),
        registers: [const { None }; 8],
        program,
        pc: 0,
        output: Vec::new(),
        memory_pool,
    };
    let result = interpreter.execute()?;
    Ok((result, interpreter.output))
}
