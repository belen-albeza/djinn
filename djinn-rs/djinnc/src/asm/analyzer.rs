use std::collections::HashMap;

use crate::asm::parser::StatementNode;
use crate::asm::{AssemblerError, Location, Opcode, ProcessType, Result};

#[derive(Debug, Clone, PartialEq)]
struct ProcessMetadata {
    location: Location,
    process_type: ProcessType,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Analyzer {
    processes: HashMap<String, ProcessMetadata>,
}

impl Analyzer {
    pub fn new() -> Self {
        Self {
            processes: HashMap::new(),
        }
    }

    pub fn add_process(&mut self, name: &str, location: Location) -> Result<ProcessType> {
        if self.processes.contains_key(name) {
            return Err(AssemblerError::ProcessAlreadyDefined(
                location,
                name.to_string(),
            ));
        }

        let process_type = self.process_type_for(name);

        let metadata = ProcessMetadata {
            location,
            process_type,
        };

        self.processes.insert(name.to_string(), metadata);
        Ok(process_type)
    }

    pub fn check_main_process_exists(&self, location: Location) -> Result<()> {
        if !self.processes.contains_key("main") {
            return Err(AssemblerError::MainProcessNotFound(location));
        }
        Ok(())
    }

    pub fn fill_args(&self, instructions: &mut Vec<StatementNode>) -> Result<()> {
        for instruction in instructions {
            #[allow(clippy::single_match)] // we will need this for more opcodes later
            match instruction.raw_opcode {
                Opcode::Spawn(_) => {
                    let alias = instruction
                        .raw_args
                        .first()
                        .ok_or(AssemblerError::MissingArgument(instruction.location))?;
                    let process_type =
                        self.processes
                            .get(alias)
                            .ok_or(AssemblerError::UnknownAlias(
                                instruction.location,
                                alias.clone(),
                            ))?;
                    instruction.raw_opcode = Opcode::Spawn(process_type.process_type);
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn process_type_for(&self, name: &str) -> ProcessType {
        if name == "main" {
            ProcessType(1)
        } else {
            if self.processes.contains_key("main") {
                ProcessType((self.processes.len() + 1) as u32)
            } else {
                ProcessType((self.processes.len() + 2) as u32)
            }
        }
    }
}
