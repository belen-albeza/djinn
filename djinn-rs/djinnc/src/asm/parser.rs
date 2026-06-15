use crate::asm::lexer::Lexer;
use crate::asm::token::TokenKind;
use crate::asm::{AssemblerError, Opcode, Result};

// TODO: more defined type here
type ProcessNode = Vec<Opcode>;

#[derive(Debug, Clone, PartialEq)]
pub struct Parser {
    current_process: String,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            current_process: "main".to_string(),
        }
    }

    pub fn parse_process(&mut self, lexer: &mut Lexer) -> Result<ProcessNode> {
        let mut tokens = Vec::new();
        loop {
            let token = lexer.scan_token()?;
            let eof = token.kind == TokenKind::Eof;
            tokens.push(token);
            if eof {
                break;
            }
        }

        let instructions: Vec<Opcode> = tokens
            .into_iter()
            .map(|token| match token.kind {
                TokenKind::NoOp => Ok(Some(Opcode::NoOp)),
                TokenKind::Yield => Ok(Some(Opcode::Yield)),
                TokenKind::Eof => Ok(None),
                _ => Err(AssemblerError::UnexpectedToken(
                    token.location,
                    token.lexeme,
                )),
            })
            .collect::<std::result::Result<Vec<_>, _>>()?
            .into_iter()
            .flatten()
            .collect();

        Ok(instructions)
    }
}
