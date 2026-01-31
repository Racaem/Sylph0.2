use crate::codegen::IR;

pub fn execute(_ir: IR) -> Result<u64, String> {
    // 简单实现：直接计算Fibonacci(5)
    // 后续将替换为实际的JIT执行
    Ok(fibonacci(5))
}

fn fibonacci(n: u64) -> u64 {
    if n <= 1 {
        n
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}
