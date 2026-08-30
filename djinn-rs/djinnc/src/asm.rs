mod analyzer;
mod error;
mod lexer;
mod parser;
mod token;

use analyzer::Analyzer;
use djinn_core::asm::{
    Instruction, Location, Number, Opcode, ProcessDefinition, ProcessType, Value, BUILTIN_LOCALS
};
use djinn_core::cart::Rom;
pub use error::{AssemblerError, Result};
use lexer::Lexer;
use parser::{Parser, ProcessNode};

impl From<ProcessNode> for ProcessDefinition {
    fn from(process: ProcessNode) -> Self {
        ProcessDefinition::new(
            process.process_type,
            process
                .instructions
                .into_iter()
                .map(|statement| Instruction::new(statement.raw_opcode, statement.location))
                .collect(),
            process.args,
        )
    }
}

pub fn compile(source_code: &str) -> Result<Rom> {
    let mut lexer = Lexer::new(source_code);
    let mut parser = Parser::new();
    let mut analyzer = Analyzer::new();

    let mut processes: Vec<ProcessNode> = vec![];

    while let Some(process) = parser.parse_process(&mut lexer, &mut analyzer)? {
        processes.push(process);
    }
    analyzer.check_main_process_exists(lexer.current_location())?;

    let rom = processes
        .into_iter()
        .map(|process| (process.process_type, process.into()))
        .collect();
    Ok(Rom::new(rom))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_returns_no_main_process_error() {
        let res = compile("");
        assert_eq!(
            res,
            Err(AssemblerError::MainProcessNotFound(Location {
                line: 1,
                column: 1
            }))
        );
    }

    #[test]
    fn test_returns_rom_with_all_processes() {
        let res = compile("~foo:\nnoop\n~main:\nnoop\n~bar:\nnoop");
        assert_eq!(
            res,
            Ok(Rom::new(HashMap::from([
                (
                    ProcessType(2),
                    ProcessDefinition::new(
                        ProcessType(2),
                        vec![Instruction::new(
                            Opcode::NoOp,
                            Location { line: 2, column: 1 }
                        )],
                        vec![],
                    )
                ),
                (
                    ProcessType(1),
                    ProcessDefinition::new(
                        ProcessType(1),
                        vec![Instruction::new(
                            Opcode::NoOp,
                            Location { line: 4, column: 1 }
                        )],
                        vec![],
                    )
                ),
                (
                    ProcessType(3),
                    ProcessDefinition::new(
                        ProcessType(3),
                        vec![Instruction::new(
                            Opcode::NoOp,
                            Location { line: 6, column: 1 }
                        )],
                        vec![],
                    )
                ),
            ])))
        );
    }
}
