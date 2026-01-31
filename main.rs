use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use clap::{Parser, Subcommand};

mod lexer;
mod parser;
mod ast;
mod semantic;
mod codegen;
mod jit;
mod executor;
mod plugin;

#[derive(Parser)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Run {
        file: PathBuf,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { file } => {
            let mut f = File::open(file)?;
            let mut code = String::new();
            f.read_to_string(&mut code)?;

            // 词法分析
            let tokens = lexer::tokenize(&code)?;
            println!("Tokens: {:?}", tokens);

            // 语法分析
            let ast = parser::parse(tokens)?;
            println!("AST: {:?}", ast);

            // 语义分析
            let semantic_ast = semantic::analyze(ast)?;
            println!("Semantic AST: {:?}", semantic_ast);

            // 中间代码生成
            let ir = codegen::generate(semantic_ast)?;
            println!("IR generated successfully");

            // JIT编译和执行
            let result = executor::execute(ir)?;
            println!("Execution result: {:?}", result);
        }
    }

    Ok(())
}
