use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::vec::Vec;

// Arena分配器
#[allow(dead_code)]
pub struct Arena {
    memory: Vec<Vec<u8>>,
    current_chunk: Vec<u8>,
    position: usize,
}

#[allow(dead_code)]
impl Arena {
    pub fn new() -> Self {
        Arena {
            memory: Vec::new(),
            current_chunk: Vec::with_capacity(4096),
            position: 0,
        }
    }
    
    pub fn allocate<T>(&mut self) -> &mut T {
        let size = std::mem::size_of::<T>();
        let align = std::mem::align_of::<T>();
        
        // 计算对齐后的位置
        let aligned_position = (self.position + align - 1) & !(align - 1);
        
        // 检查当前块是否有足够的空间
        if aligned_position + size > self.current_chunk.capacity() {
            // 保存当前块
            self.memory.push(std::mem::replace(&mut self.current_chunk, Vec::with_capacity(4096)));
            self.position = 0;
            return self.allocate::<T>();
        }
        
        // 分配内存
        let ptr = &mut self.current_chunk[aligned_position..aligned_position + size] as *mut [u8] as *mut T;
        let reference = unsafe { &mut *ptr };
        
        // 更新位置
        self.position = aligned_position + size;
        
        reference
    }
    
    pub fn clear(&mut self) {
        self.memory.clear();
        self.current_chunk.clear();
        self.current_chunk.reserve(4096);
        self.position = 0;
    }
}

// 泛型对象池
#[allow(dead_code)]
pub struct ObjectPool<T: Default + Clone + 'static> {
    objects: RefCell<Vec<T>>,
    in_use: RefCell<Vec<bool>>,
    capacity: usize,
}

#[allow(dead_code)]
impl<T: Default + Clone + 'static> ObjectPool<T> {
    pub fn new(capacity: usize) -> Self {
        let mut objects = Vec::with_capacity(capacity);
        let mut in_use = Vec::with_capacity(capacity);
        
        for _ in 0..capacity {
            objects.push(T::default());
            in_use.push(false);
        }
        
        ObjectPool {
            objects: RefCell::new(objects),
            in_use: RefCell::new(in_use),
            capacity,
        }
    }
    
    pub fn get(&self) -> Option<PooledObject<T>> {
        let mut in_use = self.in_use.borrow_mut();
        
        // 查找可用对象
        for (i, is_used) in in_use.iter_mut().enumerate() {
            if !*is_used {
                *is_used = true;
                return Some(PooledObject {
                    pool: Rc::new(ObjectPool {
                        objects: RefCell::new(self.objects.borrow().clone()),
                        in_use: RefCell::new(self.in_use.borrow().clone()),
                        capacity: self.capacity,
                    }),
                    index: i,
                });
            }
        }
        
        None
    }
    
    pub fn return_object(&self, index: usize) {
        if index < self.capacity {
            let mut in_use = self.in_use.borrow_mut();
            in_use[index] = false;
        }
    }
    
    pub fn get_object(&self, index: usize) -> Option<T> {
        if index < self.capacity {
            let objects = self.objects.borrow();
            Some(objects[index].clone())
        } else {
            None
        }
    }
    
    pub fn get_object_mut(&self, index: usize) -> Option<T> {
        if index < self.capacity {
            let objects = self.objects.borrow();
            Some(objects[index].clone())
        } else {
            None
        }
    }
}

// 池化对象
#[allow(dead_code)]
pub struct PooledObject<T: Default + Clone + 'static> {
    pool: Rc<dyn ObjectPoolTrait<T>>,
    index: usize,
}

#[allow(dead_code)]
impl<T: Default + Clone + 'static> Drop for PooledObject<T> {
    fn drop(&mut self) {
        self.pool.return_object(self.index);
    }
}

#[allow(dead_code)]
impl<T: Default + Clone + 'static> PooledObject<T> {
    pub fn get(&self) -> Option<T> {
        self.pool.get_object(self.index)
    }
    
    pub fn get_mut(&mut self) -> Option<T> {
        self.pool.get_object_mut(self.index)
    }
}

// 对象池 trait
#[allow(dead_code)]
pub trait ObjectPoolTrait<T: Default + Clone + 'static> {
    fn return_object(&self, index: usize);
    fn get_object(&self, index: usize) -> Option<T>;
    fn get_object_mut(&self, index: usize) -> Option<T>;
}

#[allow(dead_code)]
impl<T: Default + Clone + 'static> ObjectPoolTrait<T> for ObjectPool<T> {
    fn return_object(&self, index: usize) {
        if index < self.capacity {
            let mut in_use = self.in_use.borrow_mut();
            in_use[index] = false;
        }
    }
    
    fn get_object(&self, index: usize) -> Option<T> {
        if index < self.capacity {
            let objects = self.objects.borrow();
            Some(objects[index].clone())
        } else {
            None
        }
    }
    
    fn get_object_mut(&self, index: usize) -> Option<T> {
        if index < self.capacity {
            let objects = self.objects.borrow();
            Some(objects[index].clone())
        } else {
            None
        }
    }
}

// 线程本地存储的对象池
#[allow(dead_code)]
pub struct ThreadLocalObjectPool<T: Default + Clone + 'static> {
    pool: RefCell<ObjectPool<T>>,
}

#[allow(dead_code)]
impl<T: Default + Clone + 'static> ThreadLocalObjectPool<T> {
    pub fn new(capacity: usize) -> Self {
        ThreadLocalObjectPool {
            pool: RefCell::new(ObjectPool::new(capacity)),
        }
    }
    
    pub fn get(&self) -> Option<PooledObject<T>> {
        self.pool.borrow().get()
    }
}

// 全局对象池
use std::thread_local;

thread_local! {
    #[allow(dead_code)]
    static EXPR_POOL: ThreadLocalObjectPool<u64> = ThreadLocalObjectPool::new(100);
}

// 示例：使用对象池获取表达式
pub fn get_expr_pool() -> Option<PooledObject<u64>> {
    EXPR_POOL.with(|pool| pool.get())
}

// 为BytecodeInterpreter提供内存池
pub struct InterpreterMemoryPool {
    stack_pool: ObjectPool<Vec<u64>>,
    variables_pool: ObjectPool<HashMap<String, u64>>,
}

impl InterpreterMemoryPool {
    pub fn new() -> Self {
        InterpreterMemoryPool {
            stack_pool: ObjectPool::new(10),  // 10个栈对象
            variables_pool: ObjectPool::new(10),  // 10个变量映射对象
        }
    }
    
    // 获取栈对象
    pub fn get_stack(&self) -> Option<PooledObject<Vec<u64>>> {
        self.stack_pool.get()
    }
    
    // 获取变量映射对象
    pub fn get_variables(&self) -> Option<PooledObject<HashMap<String, u64>>> {
        self.variables_pool.get()
    }
}

// 全局解释器内存池
thread_local! {
    static INTERPRETER_POOL: InterpreterMemoryPool = InterpreterMemoryPool::new();
}

// 获取解释器内存池
pub fn get_interpreter_pool() -> InterpreterMemoryPool {
    InterpreterMemoryPool::new()
}

// 智能指针，用于管理AST节点的生命周期
#[allow(dead_code)]
pub struct AstNodePtr<T> {
    ptr: *mut T,
    arena: Rc<RefCell<Arena>>,
}

#[allow(dead_code)]
impl<T> AstNodePtr<T> {
    pub fn new(arena: Rc<RefCell<Arena>>, value: T) -> Self {
        let arena_ref = arena.clone();
        let mut arena_mut = arena.borrow_mut();
        let ptr = arena_mut.allocate::<T>();
        *ptr = value;
        
        AstNodePtr {
            ptr,
            arena: arena_ref,
        }
    }
    
    pub fn get(&self) -> &T {
        unsafe { &*self.ptr }
    }
    
    pub fn get_mut(&mut self) -> &mut T {
        unsafe { &mut *self.ptr }
    }
}

#[allow(dead_code)]
impl<T> Drop for AstNodePtr<T> {
    fn drop(&mut self) {
        // 不需要手动释放内存，arena会在clear时一起释放
    }
}

// 内存使用统计
#[allow(dead_code)]
pub struct MemoryStats {
    pub arena_allocation: usize,
    pub object_pool_hits: usize,
    pub object_pool_misses: usize,
    pub clone_operations: usize,
}

#[allow(dead_code)]
impl MemoryStats {
    pub fn new() -> Self {
        MemoryStats {
            arena_allocation: 0,
            object_pool_hits: 0,
            object_pool_misses: 0,
            clone_operations: 0,
        }
    }
    
    pub fn reset(&mut self) {
        self.arena_allocation = 0;
        self.object_pool_hits = 0;
        self.object_pool_misses = 0;
        self.clone_operations = 0;
    }
    
    pub fn print(&self) {
        println!("=== Memory Stats ===");
        println!("Arena allocation: {} bytes", self.arena_allocation);
        println!("Object pool hits: {}", self.object_pool_hits);
        println!("Object pool misses: {}", self.object_pool_misses);
        println!("Clone operations: {}", self.clone_operations);
        println!("==================");
    }
}

// 全局内存统计
#[allow(dead_code)]
pub static mut MEMORY_STATS: Option<MemoryStats> = None;

#[allow(dead_code)]
pub fn init_memory_stats() {
    unsafe {
        MEMORY_STATS = Some(MemoryStats::new());
    }
}

#[allow(dead_code)]
pub fn get_memory_stats() -> Option<&'static mut MemoryStats> {
    unsafe {
        let ptr = std::ptr::addr_of_mut!(MEMORY_STATS);
        (*ptr).as_mut()
    }
}

#[allow(dead_code)]
pub fn record_clone() {
    if let Some(stats) = get_memory_stats() {
        stats.clone_operations += 1;
    }
}
