use crate::lexer::Token;
use crate::ast::{Stmt, Expr};
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::OnceLock;

// 规则类型
enum RuleType {
    FunctionDefinition,
    IfStatement,
    WhileStatement,
    ReturnStatement,
    OutStatement,
    Assignment,
    Expression,
}

// 规则优先级
#[derive(Debug, PartialEq, PartialOrd)]
pub struct RulePriority {
    pub precedence: u32,
    pub frequency: u64,
}

// 规则信息
struct RuleInfo {
    rule_type: RuleType,
    priority: RulePriority,
    pattern: Vec<Token>,
    handler: fn(&[Token]) -> Option<Stmt>,
}

// 规则处理器
pub struct RuleProcessor {
    rules: HashMap<Token, Vec<RuleInfo>>,
    rule_index: HashMap<RuleType, HashSet<Token>>,
    frequency_map: HashMap<Token, u64>,
}

impl RuleProcessor {
    pub fn new() -> Self {
        let mut processor = RuleProcessor {
            rules: HashMap::new(),
            rule_index: HashMap::new(),
            frequency_map: HashMap::new(),
        };
        
        processor.initialize_rules();
        processor
    }
    
    // 初始化规则
    fn initialize_rules(&mut self) {
        // 函数定义规则
        self.add_rule(
            RuleType::FunctionDefinition,
            vec![Token::Def],
            RulePriority { precedence: 10, frequency: 100 },
            Self::handle_function_definition,
        );
        
        // if语句规则
        self.add_rule(
            RuleType::IfStatement,
            vec![Token::If],
            RulePriority { precedence: 9, frequency: 200 },
            Self::handle_if_statement,
        );
        
        // while语句规则
        self.add_rule(
            RuleType::WhileStatement,
            vec![Token::While],
            RulePriority { precedence: 9, frequency: 150 },
            Self::handle_while_statement,
        );
        
        // return语句规则
        self.add_rule(
            RuleType::ReturnStatement,
            vec![Token::Return],
            RulePriority { precedence: 8, frequency: 120 },
            Self::handle_return_statement,
        );
        
        // out语句规则
        self.add_rule(
            RuleType::OutStatement,
            vec![Token::Out],
            RulePriority { precedence: 8, frequency: 180 },
            Self::handle_out_statement,
        );
        
        // 赋值规则
        self.add_rule(
            RuleType::Assignment,
            vec![Token::Ident("dummy".to_string())],
            RulePriority { precedence: 7, frequency: 250 },
            Self::handle_assignment,
        );
    }
    
    // 添加规则
    fn add_rule(&mut self, rule_type: RuleType, pattern: Vec<Token>, priority: RulePriority, handler: fn(&[Token]) -> Option<Stmt>) {
        for token in &pattern {
            self.rules.entry(token.clone())
                .or_insert_with(Vec::new)
                .push(RuleInfo {
                    rule_type: rule_type.clone(),
                    priority,
                    pattern: pattern.clone(),
                    handler,
                });
            
            // 更新规则索引
            self.rule_index.entry(rule_type.clone())
                .or_insert_with(HashSet::new)
                .insert(token.clone());
            
            // 初始化频率计数
            self.frequency_map.entry(token.clone())
                .or_insert(0);
        }
    }
    
    // 预处理规则
    pub fn preprocess_rules(&mut self) {
        // 按优先级排序规则
        for (_, rules) in &mut self.rules {
            rules.sort_by(|a, b| {
                if a.priority.precedence != b.priority.precedence {
                    b.priority.precedence.cmp(&a.priority.precedence)
                } else {
                    b.priority.frequency.cmp(&a.priority.frequency)
                }
            });
        }
    }
    
    // 匹配规则
    pub fn match_rule(&mut self, tokens: &[Token]) -> Option<Stmt> {
        if tokens.is_empty() {
            return None;
        }
        
        let first_token = &tokens[0];
        
        // 更新频率计数
        *self.frequency_map.entry(first_token.clone()).or_insert(0) += 1;
        
        // 查找匹配的规则
        if let Some(rules) = self.rules.get(first_token) {
            for rule in rules {
                if self.matches_pattern(tokens, &rule.pattern) {
                    return (rule.handler)(tokens);
                }
            }
        }
        
        None
    }
    
    // 匹配模式
    fn matches_pattern(&self, tokens: &[Token], pattern: &[Token]) -> bool {
        if tokens.len() < pattern.len() {
            return false;
        }
        
        for (i, (token, pattern_token)) in tokens.iter().zip(pattern.iter()).enumerate() {
            match (token, pattern_token) {
                (Token::Ident(_), Token::Ident("dummy")) => {
                    // 匹配任意标识符
                    continue;
                }
                _ if token == pattern_token => {
                    continue;
                }
                _ => {
                    return false;
                }
            }
        }
        
        true
    }
    
    // 规则处理函数
    fn handle_function_definition(tokens: &[Token]) -> Option<Stmt> {
        // 简化实现，实际解析由parser.rs处理
        None
    }
    
    fn handle_if_statement(tokens: &[Token]) -> Option<Stmt> {
        None
    }
    
    fn handle_while_statement(tokens: &[Token]) -> Option<Stmt> {
        None
    }
    
    fn handle_return_statement(tokens: &[Token]) -> Option<Stmt> {
        None
    }
    
    fn handle_out_statement(tokens: &[Token]) -> Option<Stmt> {
        None
    }
    
    fn handle_assignment(tokens: &[Token]) -> Option<Stmt> {
        None
    }
    
    // 获取规则统计信息
    pub fn get_rule_stats(&self) -> HashMap<Token, u64> {
        self.frequency_map.clone()
    }
    
    // 优化规则排序
    pub fn optimize_rule_order(&mut self) {
        // 根据频率重新排序规则
        for (_, rules) in &mut self.rules {
            rules.sort_by(|a, b| {
                let freq_a = self.frequency_map.get(&a.pattern[0]).unwrap_or(&0);
                let freq_b = self.frequency_map.get(&b.pattern[0]).unwrap_or(&0);
                
                if freq_a != freq_b {
                    freq_b.cmp(freq_a)
                } else if a.priority.precedence != b.priority.precedence {
                    b.priority.precedence.cmp(&a.priority.precedence)
                } else {
                    b.priority.frequency.cmp(&a.priority.frequency)
                }
            });
        }
    }
}

// 全局规则处理器
static GLOBAL_RULE_PROCESSOR: OnceLock<RuleProcessor> = OnceLock::new();

// 获取全局规则处理器
pub fn get_rule_processor() -> &'static RuleProcessor {
    GLOBAL_RULE_PROCESSOR.get_or_init(|| {
        let mut processor = RuleProcessor::new();
        processor.preprocess_rules();
        processor
    })
}

// 规则缓存
pub struct RuleCache {
    cache: HashMap<Vec<Token>, Stmt>,
    lru: VecDeque<Vec<Token>>,
    capacity: usize,
}

impl RuleCache {
    pub fn new(capacity: usize) -> Self {
        RuleCache {
            cache: HashMap::with_capacity(capacity),
            lru: VecDeque::with_capacity(capacity),
            capacity,
        }
    }
    
    pub fn get(&mut self, key: &[Token]) -> Option<&Stmt> {
        let key_vec: Vec<Token> = key.to_vec();
        if let Some(stmt) = self.cache.get(&key_vec) {
            // 更新LRU
            self.lru.retain(|k| k != &key_vec);
            self.lru.push_back(key_vec);
            Some(stmt)
        } else {
            None
        }
    }
    
    pub fn put(&mut self, key: &[Token], value: Stmt) {
        let key_vec: Vec<Token> = key.to_vec();
        
        // 如果缓存已满，删除最久未使用的项
        if self.cache.len() >= self.capacity {
            if let Some(evicted) = self.lru.pop_front() {
                self.cache.remove(&evicted);
            }
        }
        
        self.cache.insert(key_vec.clone(), value);
        self.lru.push_back(key_vec);
    }
    
    pub fn size(&self) -> usize {
        self.cache.len()
    }
}

// 全局规则缓存
static GLOBAL_RULE_CACHE: OnceLock<RuleCache> = OnceLock::new();

// 获取全局规则缓存
pub fn get_rule_cache() -> &'static RuleCache {
    GLOBAL_RULE_CACHE.get_or_init(|| RuleCache::new(1000))
}

// 规则处理优化函数
pub fn optimize_rule_handling() {
    // 预处理规则
    let mut processor = RuleProcessor::new();
    processor.preprocess_rules();
    processor.optimize_rule_order();
    
    println!("Rule processing optimized");
    println!("Rule statistics:");
    for (token, frequency) in processor.get_rule_stats() {
        println!("{:?}: {}", token, frequency);
    }
}
