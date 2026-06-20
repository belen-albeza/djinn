use std::fmt;

mod analyzer;
mod error;
mod lexer;
mod parser;
mod token;

use analyzer::Analyzer;
use djinn_core::asm::{Opcode, ProcessDefinition, ProcessType};
use djinn_core::cart::Rom;
pub use error::{AssemblerError, Result};
use lexer::Lexer;
use parser::{Parser, ProcessNode};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Location {
    pub line: usize,
    pub column: usize,
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Ln {}, Col {}", self.line, self.column)
    }
}

impl Default for Location {
    fn default() -> Self {
        Self { line: 1, column: 1 }
    }
}

impl From<ProcessNode> for ProcessDefinition {
    fn from(process: ProcessNode) -> Self {
        ProcessDefinition::new(
            process.process_type,
            process
                .instructions
                .into_iter()
                .map(|statement| statement.raw_opcode)
                .collect(),
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
    use std::collections::HashMap;

    use super::*;

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
        let res = compile("~main:\nnoop\n~secondary:\nnoop");
        assert_eq!(
            res,
            Ok(Rom::new(HashMap::from([
                (
                    ProcessType(1),
                    ProcessDefinition::new(ProcessType(1), vec![Opcode::NoOp])
                ),
                (
                    ProcessType(2),
                    ProcessDefinition::new(ProcessType(2), vec![Opcode::NoOp])
                ),
            ])))
        );
    }
}
