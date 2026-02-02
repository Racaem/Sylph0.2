use crate::ast::{Expr, Stmt, Program};
use crate::bytecode::{Bytecode, BytecodeProgram};
use std::collections::{HashMap, VecDeque};
use std::hash::{Hash, Hasher};
use std::time::Instant;

// 缓存键的类型
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CacheKey {
    Statement(String),      // 语句缓存键
    Function(String),       // 函数缓存键
    Rule(String),            // 规则缓存键
    Expression(String),      // 表达式缓存键
}

// 缓存值的类型
#[derive(Debug, Clone)]
pub enum CacheValue {
    Statement(Stmt),         // 语句缓存值
    Function((String, Vec<Stmt>)),  // 函数缓存值
    Rule(Bytecode),          // 规则缓存值
    Expression(Expr),        // 表达式缓存值
    ExecutionResult(u64),    // 执行结果缓存值
}

// 缓存项
#[derive(Debug, Clone)]
pub struct CacheItem {
    key: CacheKey,
    value: CacheValue,
    timestamp: Instant,
    access_count: u64,
}

// 缓存接口
pub trait Cache {
    fn get(&mut self, key: &CacheKey) -> Option<CacheValue>;
    fn put(&mut self, key: CacheKey, value: CacheValue);
    fn remove(&mut self, key: &CacheKey);
    fn clear(&mut self);
    fn size(&self) -> usize;
}

// LRU缓存实现
pub struct LRUCache {
    capacity: usize,
    cache: HashMap<CacheKey, CacheItem>,
    lru: VecDeque<CacheKey>,
}

impl LRUCache {
    pub fn new(capacity: usize) -> Self {
        LRUCache {
            capacity,
            cache: HashMap::with_capacity(capacity),
            lru: VecDeque::with_capacity(capacity),
        }
    }
}

impl Cache for LRUCache {
    fn get(&mut self, key: &CacheKey) -> Option<CacheValue> {
        if let Some(item) = self.cache.get_mut(key) {
            // 更新访问时间和访问次数
            item.timestamp = Instant::now();
            item.access_count += 1;
            
            // 将键移到LRU队列的末尾
            self.lru.retain(|k| k != key);
            self.lru.push_back(key.clone());
            
            Some(item.value.clone())
        } else {
            None
        }
    }
    
    fn put(&mut self, key: CacheKey, value: CacheValue) {
        // 如果缓存已满，删除最久未使用的项
        if self.cache.len() >= self.capacity {
            if let Some(evicted_key) = self.lru.pop_front() {
                self.cache.remove(&evicted_key);
            }
        }
        
        // 添加新项
        let item = CacheItem {
            key: key.clone(),
            value,
            timestamp: Instant::now(),
            access_count: 1,
        };
        
        self.cache.insert(key.clone(), item);
        self.lru.push_back(key);
    }
    
    fn remove(&mut self, key: &CacheKey) {
        self.cache.remove(key);
        self.lru.retain(|k| k != key);
    }
    
    fn clear(&mut self) {
        self.cache.clear();
        self.lru.clear();
    }
    
    fn size(&self) -> usize {
        self.cache.len()
    }
}

// 多级缓存
pub struct MultiLevelCache {
    l1: LRUCache,  // 一级缓存，容量小，访问快
    l2: LRUCache,  // 二级缓存，容量大，访问慢
}

impl MultiLevelCache {
    pub fn new(l1_capacity: usize, l2_capacity: usize) -> Self {
        MultiLevelCache {
            l1: LRUCache::new(l1_capacity),
            l2: LRUCache::new(l2_capacity),
        }
    }
}

impl Cache for MultiLevelCache {
    fn get(&mut self, key: &CacheKey) -> Option<CacheValue> {
        // 先从一级缓存获取
        if let Some(value) = self.l1.get(key) {
            return Some(value);
        }
        
        // 再从二级缓存获取
        if let Some(value) = self.l2.get(key) {
            // 将值提升到一级缓存
            self.l1.put(key.clone(), value.clone());
            return Some(value);
        }
        
        None
    }
    
    fn put(&mut self, key: CacheKey, value: CacheValue) {
        // 先放入一级缓存
        self.l1.put(key.clone(), value.clone());
        
        // 再放入二级缓存
        self.l2.put(key, value);
    }
    
    fn remove(&mut self, key: &CacheKey) {
        self.l1.remove(key);
        self.l2.remove(key);
    }
    
    fn clear(&mut self) {
        self.l1.clear();
        self.l2.clear();
    }
    
    fn size(&self) -> usize {
        self.l1.size() + self.l2.size()
    }
}

// 全局缓存管理器
pub struct CacheManager {
    statement_cache: MultiLevelCache,
    function_cache: MultiLevelCache,
    rule_cache: MultiLevelCache,
    expression_cache: MultiLevelCache,
}

impl CacheManager {
    pub fn new() -> Self {
        CacheManager {
            statement_cache: MultiLevelCache::new(100, 1000),
            function_cache: MultiLevelCache::new(50, 500),
            rule_cache: MultiLevelCache::new(200, 2000),
            expression_cache: MultiLevelCache::new(150, 1500),
        }
    }
    
    // 语句缓存操作
    pub fn get_statement(&mut self, key: &str) -> Option<Stmt> {
        let cache_key = CacheKey::Statement(key.to_string());
        match self.statement_cache.get(&cache_key) {
            Some(CacheValue::Statement(stmt)) => Some(stmt),
            _ => None,
        }
    }
    
    pub fn put_statement(&mut self, key: &str, stmt: Stmt) {
        let cache_key = CacheKey::Statement(key.to_string());
        let cache_value = CacheValue::Statement(stmt);
        self.statement_cache.put(cache_key, cache_value);
    }
    
    // 函数缓存操作
    pub fn get_function(&mut self, key: &str) -> Option<(String, Vec<Stmt>)> {
        let cache_key = CacheKey::Function(key.to_string());
        match self.function_cache.get(&cache_key) {
            Some(CacheValue::Function(func)) => Some(func),
            _ => None,
        }
    }
    
    pub fn put_function(&mut self, key: &str, func: (String, Vec<Stmt>)) {
        let cache_key = CacheKey::Function(key.to_string());
        let cache_value = CacheValue::Function(func);
        self.function_cache.put(cache_key, cache_value);
    }
    
    // 规则缓存操作
    pub fn get_rule(&mut self, key: &str) -> Option<Bytecode> {
        let cache_key = CacheKey::Rule(key.to_string());
        match self.rule_cache.get(&cache_key) {
            Some(CacheValue::Rule(rule)) => Some(rule),
            _ => None,
        }
    }
    
    pub fn put_rule(&mut self, key: &str, rule: Bytecode) {
        let cache_key = CacheKey::Rule(key.to_string());
        let cache_value = CacheValue::Rule(rule);
        self.rule_cache.put(cache_key, cache_value);
    }
    
    // 表达式缓存操作
    pub fn get_expression(&mut self, key: &str) -> Option<Expr> {
        let cache_key = CacheKey::Expression(key.to_string());
        match self.expression_cache.get(&cache_key) {
            Some(CacheValue::Expression(expr)) => Some(expr),
            _ => None,
        }
    }
    
    pub fn put_expression(&mut self, key: &str, expr: Expr) {
        let cache_key = CacheKey::Expression(key.to_string());
        let cache_value = CacheValue::Expression(expr);
        self.expression_cache.put(cache_key, cache_value);
    }
    
    // 执行结果缓存操作
    pub fn get_execution_result(&mut self, key: &str) -> Option<u64> {
        let cache_key = CacheKey::Expression(key.to_string());
        match self.expression_cache.get(&cache_key) {
            Some(CacheValue::ExecutionResult(result)) => Some(result),
            _ => None,
        }
    }
    
    pub fn put_execution_result(&mut self, key: &str, result: u64) {
        let cache_key = CacheKey::Expression(key.to_string());
        let cache_value = CacheValue::ExecutionResult(result);
        self.expression_cache.put(cache_key, cache_value);
    }
    
    // 清除所有缓存
    pub fn clear(&mut self) {
        self.statement_cache.clear();
        self.function_cache.clear();
        self.rule_cache.clear();
        self.expression_cache.clear();
    }
    
    // 获取缓存大小
    pub fn size(&self) -> usize {
        self.statement_cache.size() +
        self.function_cache.size() +
        self.rule_cache.size() +
        self.expression_cache.size()
    }
}

// 全局缓存实例
use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref GLOBAL_CACHE: Mutex<CacheManager> = Mutex::new(CacheManager::new());
}

// 缓存使用统计
pub struct CacheStats {
    pub hits: usize,
    pub misses: usize,
    pub puts: usize,
    pub evictions: usize,
    pub total_size: usize,
}

impl CacheStats {
    pub fn new() -> Self {
        CacheStats {
            hits: 0,
            misses: 0,
            puts: 0,
            evictions: 0,
            total_size: 0,
        }
    }
    
    pub fn reset(&mut self) {
        self.hits = 0;
        self.misses = 0;
        self.puts = 0;
        self.evictions = 0;
        self.total_size = 0;
    }
    
    pub fn print(&self) {
        println!("=== Cache Stats ===");
        println!("Hits: {}", self.hits);
        println!("Misses: {}", self.misses);
        println!("Puts: {}", self.puts);
        println!("Evictions: {}", self.evictions);
        println!("Total size: {}", self.total_size);
        println!("Hit rate: {:.2}%", if self.hits + self.misses > 0 {
            (self.hits as f64 / (self.hits + self.misses) as f64) * 100.0
        } else {
            0.0
        });
        println!("==================");
    }
}

// 全局缓存统计
lazy_static::lazy_static! {
    static ref CACHE_STATS: Mutex<CacheStats> = Mutex::new(CacheStats::new());
}

// 缓存操作辅助函数
pub fn get_global_cache() -> &'static Mutex<CacheManager> {
    &GLOBAL_CACHE
}

pub fn get_cache_stats() -> &'static Mutex<CacheStats> {
    &CACHE_STATS
}

// 示例：使用缓存获取函数
pub fn get_cached_function(name: &str) -> Option<(String, Vec<Stmt>)> {
    let mut cache = get_global_cache().lock().unwrap();
    let result = cache.get_function(name);
    
    let mut stats = get_cache_stats().lock().unwrap();
    if result.is_some() {
        stats.hits += 1;
    } else {
        stats.misses += 1;
    }
    
    result
}

pub fn put_cached_function(name: &str, func: (String, Vec<Stmt>)) {
    let mut cache = get_global_cache().lock().unwrap();
    cache.put_function(name, func);
    
    let mut stats = get_cache_stats().lock().unwrap();
    stats.puts += 1;
    stats.total_size = cache.size();
}

// 缓存预热
pub fn warmup_cache(program: &Program) {
    let mut cache = get_global_cache().lock().unwrap();
    
    // 预热函数缓存
    for stmt in &program.statements {
        if let Stmt::FuncDef(name, param, body) = stmt {
            cache.put_function(name, (param.clone(), body.clone()));
        }
    }
    
    println!("Cache warmed up with {} functions", program.statements.len());
}
