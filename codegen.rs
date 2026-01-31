use crate::ast::Program;

#[derive(Debug)]
pub struct IR {
    pub program: Program,
}

pub fn generate(program: Program) -> Result<IR, String> {
    Ok(IR {
        program,
    })
}
