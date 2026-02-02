use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::time::Instant;

use clap::{Parser, Subcommand};

mod lexer;
mod parser;
mod ast;
mod semantic;
mod codegen;
mod jit;
mod executor;
mod plugin;
mod profiler;
mod memory;
mod bytecode;
mod types;

#[derive(Parser)]
pub struct Cli {
    #[clap(long, short, help = "Specify the syl file to run")]
    pub file: Option<PathBuf>,
    
    #[clap(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Run {
        file: Option<PathBuf>,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start_time = Instant::now();
    
    // 初始化内存统计
    memory::init_memory_stats();
    
    let cli = Cli::parse();
    // 启用全局分析器
    profiler::enable_profiling();

    // 确定要运行的文件路径
    let file_path = cli.file.or_else(|| {
        if let Some(Commands::Run { file: run_file }) = &cli.command {
            run_file.clone()
        } else {
            None
        }
    }).ok_or_else(|| "No file specified. Use --file or run subcommand with file argument.")?;
    
    // 跨平台文件路径处理
    let normalized_path = file_path.as_path();
    let mut f = File::open(normalized_path)?;
    let mut code = String::new();
    f.read_to_string(&mut code)?;

    let tokens = profiler::profile("tokenization", || {
        lexer::tokenize(&code)
    })?;
    //println!("Tokens: {:?}", tokens);

    let ast = profiler::profile("parsing", || {
        parser::parse(tokens)
    })?;
   // println!("AST: {:#?}", ast);

    let semantic_ast = profiler::profile("semantic_analysis", || {
        semantic::analyze(ast)
    })?;
    //println!("Semantic AST: {:?}", semantic_ast);

    let ir = profiler::profile("code_generation", || {
        codegen::generate(semantic_ast)
    })?;
    println!("IR generated successfully\n");

    println!();
    let (result, output) = profiler::profile("execution", || {
        executor::execute(ir)
    })?;
    
    // 打印分析结果
    println!("========================================");
    println!("             DEBUG INFORMATION");
    println!("========================================");
    profiler::print_profiling_results();
    println!("Execution result: {:?}\n", result);
    
    let total_time = start_time.elapsed();
    println!("========================================");
    println!("            PROGRAM OUTPUT");
    println!("========================================");
    for line in output {
        println!("{}", line);
    }
    println!();
    println!("Total execution time: {} ms", total_time.as_millis());
    // if let Some(stats) = memory::get_memory_stats() {
    //     println!();
    //     stats.print();
    // }

    Ok(())
}
