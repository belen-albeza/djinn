use std::collections::HashMap;

use crate::asm::parser::StatementNode;
use crate::asm::{AssemblerError, BUILTIN_LOCALS, Location, Opcode, ProcessType, Result};

#[derive(Debug, Clone, PartialEq)]
struct ProcessMetadata {
    pub location: Location,
    pub process_type: ProcessType,
    pub locals: HashMap<String, usize>,
}

impl ProcessMetadata {
    pub fn new(location: Location, process_type: ProcessType) -> Self {
        Self {
            location,
            process_type,
            locals: BUILTIN_LOCALS
                .iter()
                .enumerate()
                .map(|(i, (name, _))| (name.to_string(), i))
                .collect(),
        }
    }
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

        let metadata = ProcessMetadata::new(location, process_type);

        self.processes.insert(name.to_string(), metadata);
        Ok(process_type)
    }

    pub fn add_local(&mut self, process_alias: &str, local: String) -> Result<usize> {
        let process = self
            .processes
            .get_mut(process_alias)
            .expect("Process does not exist");
        let count = process.locals.len();
        let index = process.locals.entry(local).or_insert(count);

        Ok(*index)
    }

    pub fn check_main_process_exists(&self, location: Location) -> Result<()> {
        if !self.processes.contains_key("main") {
            return Err(AssemblerError::MainProcessNotFound(location));
        }
        Ok(())
    }

    pub fn resolve_process_refs(&self, instructions: &mut Vec<StatementNode>) -> Result<()> {
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
