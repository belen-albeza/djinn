use crate::asm::analyzer::Analyzer;
use crate::asm::lexer::Lexer;
use crate::asm::token::{Token, TokenKind};
use crate::asm::{AssemblerError, Location, Number, Opcode, ProcessType, Result, Value};

#[derive(Debug, Clone, PartialEq)]
pub struct StatementNode {
    pub raw_opcode: Opcode,
    pub location: Location,
    // TODO: arg
}

impl StatementNode {
    pub fn new(raw_opcode: Opcode, location: Location) -> Self {
        Self {
            raw_opcode,
            location,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ProcessNode {
    pub instructions: Vec<StatementNode>,
    pub process_type: ProcessType,
    pub name: String,
    pub location: Location,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parser {
    current_process: String,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            current_process: "".to_string(),
        }
    }

    pub fn parse_process(
        &mut self,
        lexer: &mut Lexer,
        analyzer: &mut Analyzer,
    ) -> Result<Option<ProcessNode>> {
        // Stop parsing at EOF
        if self.consume_peeked(lexer, TokenKind::Eof)?.is_some() {
            return Ok(None);
        }

        let (location, name, process_type) = self.parse_process_declaration(lexer, analyzer)?;

        let instructions = self.parse_statements(lexer)?;

        Ok(Some(ProcessNode {
            instructions,
            process_type,
            name,
            location,
        }))
    }

    fn consume(&mut self, lexer: &mut Lexer, expected: &[TokenKind]) -> Result<Token> {
        let token = lexer.scan_token()?;
        if !expected.contains(&token.kind) {
            return Err(AssemblerError::UnexpectedToken {
                location: token.location,
                token: token.lexeme,
                expected: expected.to_vec(),
            });
        }
        Ok(token)
    }

    // Peeks token and consumes it if it matches the expected kind.
    // Returns Ok(None) if the kind does not match.
    // Return Err if there's a lexing error
    fn consume_peeked(
        &mut self,
        lexer: &mut Lexer,
        expected_kind: TokenKind,
    ) -> Result<Option<Token>> {
        let token = lexer.peek_token()?;
        if token.kind == expected_kind {
            lexer.scan_token()?;
            return Ok(Some(token));
        }
        Ok(None)
    }

    fn consume_value(&mut self, lexer: &mut Lexer) -> Result<Value> {
        let token = lexer.scan_token()?;
        let value = match token.kind {
            TokenKind::Int(value) => Value::Numeric(Number::Int(value)),
            TokenKind::Float(value) => Value::Numeric(Number::Float(value)),
            TokenKind::Bool(value) => Value::Bool(value),
            _ => {
                return Err(AssemblerError::UnexpectedToken {
                    location: token.location,
                    token: token.lexeme,
                    expected: vec![
                        TokenKind::Int(0),
                        TokenKind::Float(0.0),
                        TokenKind::Bool(false),
                    ],
                });
            }
        };
        Ok(value)
    }

    fn parse_process_declaration(
        &mut self,
        lexer: &mut Lexer,
        analyzer: &mut Analyzer,
    ) -> Result<(Location, String, ProcessType)> {
        let tilde = self.consume(lexer, &[TokenKind::Tilde])?;

        let identifier = self.consume(lexer, &[TokenKind::Id])?;
        self.current_process = identifier.lexeme.clone();
        let process_type = analyzer.add_process(&self.current_process, tilde.location)?;

        self.consume(lexer, &[TokenKind::Colon])?;

        Ok((tilde.location, identifier.lexeme, process_type))
    }

    fn parse_statements(&mut self, lexer: &mut Lexer) -> Result<Vec<StatementNode>> {
        let mut res = Vec::new();
        while let Some(statement) = self.parse_single_statement(lexer)? {
            res.push(statement);
            // TODO: increment PC for labels
        }

        Ok(res)
    }

    fn parse_single_statement(&mut self, lexer: &mut Lexer) -> Result<Option<StatementNode>> {
        // stop at process declaration or EOF
        let peeked = lexer.peek_token()?;
        if peeked.kind == TokenKind::Tilde || peeked.kind == TokenKind::Eof {
            return Ok(None);
        }

        let token = lexer.scan_token()?;
        match token.kind {
            TokenKind::NoOp => Ok(Some(StatementNode::new(Opcode::NoOp, token.location))),
            TokenKind::Yield => Ok(Some(StatementNode::new(Opcode::Yield, token.location))),
            TokenKind::Pop => Ok(Some(StatementNode::new(Opcode::Pop, token.location))),
            TokenKind::Dup => Ok(Some(StatementNode::new(Opcode::Dup, token.location))),
            TokenKind::Push => self.parse_push(lexer, token.location),
            TokenKind::Hash => self.parse_push(lexer, token.location), // # is a shortcut for push
            _ => Err(AssemblerError::UnexpectedToken {
                location: token.location,
                token: token.lexeme,
                expected: vec![],
            }),
        }
    }

    fn parse_push(
        &mut self,
        lexer: &mut Lexer,
        location: Location,
    ) -> Result<Option<StatementNode>> {
        let value = self.consume_value(lexer)?;
        Ok(Some(StatementNode::new(Opcode::Push(value), location)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_process_declaration() {
        let mut lexer = Lexer::new("~main:\nnoop");
        let mut parser = Parser::new();
        let mut analyzer = Analyzer::new();

        let process = parser.parse_process(&mut lexer, &mut analyzer).unwrap();
        assert_eq!(
            process,
            Some(ProcessNode {
                instructions: vec![StatementNode::new(
                    Opcode::NoOp,
                    Location { line: 2, column: 1 }
                )],
                process_type: ProcessType(1),
                name: "main".to_string(),
                location: Location { line: 1, column: 1 },
            })
        );
    }

    #[test]
    fn test_parse_opcode_with_value() {
        let mut lexer = Lexer::new("push true");
        let mut parser = Parser::new();

        let statement = parser.parse_single_statement(&mut lexer).unwrap();
        assert_eq!(
            statement,
            Some(StatementNode::new(
                Opcode::Push(Value::Bool(true)),
                Location { line: 1, column: 1 }
            ))
        );
    }

    #[test]
    fn test_parse_hash_as_shortcut_for_push() {
        let mut lexer = Lexer::new("#1.234");
        let mut parser = Parser::new();

        let statement = parser.parse_single_statement(&mut lexer).unwrap();
        assert_eq!(
            statement,
            Some(StatementNode::new(
                Opcode::Push(Value::Numeric(Number::Float(1.234))),
                Location { line: 1, column: 1 }
            ))
        );
    }
}
