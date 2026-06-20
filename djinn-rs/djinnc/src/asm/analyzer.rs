use std::collections::HashMap;

use crate::asm::{AssemblerError, Location, ProcessType, Result};

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

        let process_type = if name == "main" {
            ProcessType(1)
        } else {
            ProcessType(2)
        };

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
}
