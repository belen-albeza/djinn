use crate::asm::analyzer::Analyzer;
use crate::asm::lexer::Lexer;
use crate::asm::token::{Token, TokenKind};
use crate::asm::{AssemblerError, Location, Number, Opcode, ProcessType, Result, Value};
use djinn_core::devices::DeviceType;

mod alias;
use alias::Alias;

#[derive(Debug, Clone, PartialEq)]
pub struct StatementNode {
    pub raw_opcode: Opcode,
    pub location: Location,
    pub raw_args: Vec<String>,
}

impl StatementNode {
    pub fn new(raw_opcode: Opcode, location: Location) -> Self {
        Self {
            raw_opcode,
            location,
            raw_args: vec![],
        }
    }

    pub fn with_args(self, args: Vec<String>) -> Self {
        Self {
            raw_args: args,
            ..self
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ProcessNode {
    pub instructions: Vec<StatementNode>,
    pub process_type: ProcessType,
    pub name: String,
    pub location: Location,
    pub args: Vec<usize>,
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

        let (location, name, process_type, args) =
            self.parse_process_declaration(lexer, analyzer)?;

        let instructions = self.parse_statements(lexer, analyzer)?;

        Ok(Some(ProcessNode {
            instructions,
            process_type,
            name,
            location,
            args,
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

    fn consume_process_alias(&mut self, lexer: &mut Lexer) -> Result<String> {
        self.consume(lexer, &[TokenKind::Tilde])?;
        let id = self.consume(lexer, &[TokenKind::Id])?;
        Ok(id.lexeme)
    }

    fn consume_value_or_alias(&mut self, lexer: &mut Lexer) -> Result<Value> {
        let peeked = lexer.peek_token()?;
        if peeked.kind == TokenKind::Colon {
            let alias = self.consume_alias(lexer)?;
            return Ok(Value::Numeric(Number::Int(alias as i32)));
        }

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

    fn consume_alias(&mut self, lexer: &mut Lexer) -> Result<u8> {
        self.consume(lexer, &[TokenKind::Colon])?;
        let id = self.consume(lexer, &[TokenKind::Id])?;
        let alias = Alias::try_from(id.lexeme).map_err(|e| e.with_location(id.location))?;

        Ok(alias.0)
    }

    fn consume_local(&mut self, lexer: &mut Lexer, analyzer: &mut Analyzer) -> Result<usize> {
        self.consume(lexer, &[TokenKind::Dollar])?;
        let id = self.consume(lexer, &[TokenKind::Id])?;
        let index = analyzer.add_local(&self.current_process, id.lexeme.to_owned())?;

        Ok(index)
    }

    fn parse_process_declaration(
        &mut self,
        lexer: &mut Lexer,
        analyzer: &mut Analyzer,
    ) -> Result<(Location, String, ProcessType, Vec<usize>)> {
        let tilde = self.consume(lexer, &[TokenKind::Tilde])?;

        let identifier = self.consume(lexer, &[TokenKind::Id])?;
        self.current_process = identifier.lexeme.clone();
        let process_type = analyzer.add_process(&self.current_process, tilde.location)?;

        // parse args
        let mut args = Vec::new();
        while lexer.peek_token()?.kind == TokenKind::Dollar {
            let arg = self.consume_local(lexer, analyzer)?;
            args.push(arg);
        }

        self.consume(lexer, &[TokenKind::Colon])?;

        Ok((tilde.location, identifier.lexeme, process_type, args))
    }

    fn parse_statements(
        &mut self,
        lexer: &mut Lexer,
        analyzer: &mut Analyzer,
    ) -> Result<Vec<StatementNode>> {
        let mut res = Vec::new();
        while let Some(statement) = self.parse_single_statement(lexer, analyzer)? {
            res.push(statement);
            // TODO: increment pc for labels
        }

        Ok(res)
    }

    fn parse_single_statement(
        &mut self,
        lexer: &mut Lexer,
        analyzer: &mut Analyzer,
    ) -> Result<Option<StatementNode>> {
        // stop at process declaration or EOF
        let peeked = lexer.peek_token()?;
        if peeked.kind == TokenKind::Tilde || peeked.kind == TokenKind::Eof {
            return Ok(None);
        }

        let token = lexer.scan_token()?;
        match token.kind {
            TokenKind::NoOp => Ok(Some(StatementNode::new(Opcode::NoOp, token.location))),
            TokenKind::Yield => Ok(Some(StatementNode::new(Opcode::Yield, token.location))),
            TokenKind::Spawn => self.parse_spawn(lexer, token.location),
            TokenKind::Kill => Ok(Some(StatementNode::new(Opcode::Kill, token.location))),
            TokenKind::Dev => self.parse_dev_call(lexer, token.location),
            TokenKind::Pop => Ok(Some(StatementNode::new(Opcode::Pop, token.location))),
            TokenKind::Dup => Ok(Some(StatementNode::new(Opcode::Dup, token.location))),
            TokenKind::Stl => self.parse_store_local(lexer, analyzer, token.location),
            TokenKind::Ldl => self.parse_load_local(lexer, analyzer, token.location),
            TokenKind::Push => self.parse_push(lexer, token.location),
            TokenKind::Hash => self.parse_push(lexer, token.location), // # is a shortcut for push
            TokenKind::Not => Ok(Some(StatementNode::new(Opcode::Not, token.location))),
            TokenKind::And => Ok(Some(StatementNode::new(Opcode::And, token.location))),
            TokenKind::Or => Ok(Some(StatementNode::new(Opcode::Or, token.location))),
            TokenKind::Xor => Ok(Some(StatementNode::new(Opcode::Xor, token.location))),
            TokenKind::Add => Ok(Some(StatementNode::new(Opcode::Add, token.location))),
            TokenKind::Sub => Ok(Some(StatementNode::new(Opcode::Sub, token.location))),
            TokenKind::Mul => Ok(Some(StatementNode::new(Opcode::Mul, token.location))),
            TokenKind::Div => Ok(Some(StatementNode::new(Opcode::Div, token.location))),
            TokenKind::Mod => Ok(Some(StatementNode::new(Opcode::Mod, token.location))),
            TokenKind::Eq => Ok(Some(StatementNode::new(Opcode::Eq, token.location))),
            TokenKind::Neq => Ok(Some(StatementNode::new(Opcode::Neq, token.location))),
            TokenKind::Lt => Ok(Some(StatementNode::new(Opcode::Lt, token.location))),
            TokenKind::Leq => Ok(Some(StatementNode::new(Opcode::Leq, token.location))),
            TokenKind::Gt => Ok(Some(StatementNode::new(Opcode::Gt, token.location))),
            TokenKind::Geq => Ok(Some(StatementNode::new(Opcode::Geq, token.location))),
            TokenKind::Inc => Ok(Some(StatementNode::new(Opcode::Inc, token.location))),
            TokenKind::Dec => Ok(Some(StatementNode::new(Opcode::Dec, token.location))),
            _ => Err(AssemblerError::UnexpectedToken {
                location: token.location,
                token: token.lexeme,
                expected: vec![],
            }),
        }
    }

    fn parse_spawn(
        &mut self,
        lexer: &mut Lexer,
        location: Location,
    ) -> Result<Option<StatementNode>> {
        let alias = self.consume_process_alias(lexer)?;
        Ok(Some(
            StatementNode::new(Opcode::Spawn(ProcessType(0)), location).with_args(vec![alias]),
        ))
    }

    fn parse_push(
        &mut self,
        lexer: &mut Lexer,
        location: Location,
    ) -> Result<Option<StatementNode>> {
        let value = self.consume_value_or_alias(lexer)?;
        Ok(Some(StatementNode::new(Opcode::Push(value), location)))
    }

    fn parse_store_local(
        &mut self,
        lexer: &mut Lexer,
        analyzer: &mut Analyzer,
        location: Location,
    ) -> Result<Option<StatementNode>> {
        let index = self.consume_local(lexer, analyzer)?;
        Ok(Some(StatementNode::new(Opcode::Stl(index), location)))
    }

    fn parse_load_local(
        &mut self,
        lexer: &mut Lexer,
        analyzer: &mut Analyzer,
        location: Location,
    ) -> Result<Option<StatementNode>> {
        let index = self.consume_local(lexer, analyzer)?;
        Ok(Some(StatementNode::new(Opcode::Ldl(index), location)))
    }

    fn parse_dev_call(
        &mut self,
        lexer: &mut Lexer,
        location: Location,
    ) -> Result<Option<StatementNode>> {
        let device_type = self.consume_alias(lexer)?;
        let api_op = self.consume_alias(lexer)?;

        let opcode = Opcode::Device(DeviceType::from(device_type), api_op);
        Ok(Some(StatementNode::new(opcode, location)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use djinn_core::asm::BUILTIN_LOCALS;

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
                args: vec![],
            })
        );
    }

    #[test]
    fn test_parse_process_declaration_with_args() {
        let mut lexer = Lexer::new("~ship $x $y $foo: noop");
        let mut parser = Parser::new();
        let mut analyzer = Analyzer::new();

        let process = parser.parse_process(&mut lexer, &mut analyzer).unwrap();
        let custom_locals_offset = BUILTIN_LOCALS.len();
        assert_eq!(
            process,
            Some(ProcessNode {
                instructions: vec![StatementNode::new(
                    Opcode::NoOp,
                    Location {
                        line: 1,
                        column: 19
                    }
                )],
                process_type: ProcessType(2),
                name: "ship".to_string(),
                location: Location { line: 1, column: 1 },
                // TODO: fix slotted indexes once we have built-in local s
                args: vec![0, 1, custom_locals_offset],
            })
        );
    }

    #[test]
    fn test_parse_opcode_with_value() {
        let mut lexer = Lexer::new("push true");
        let mut parser = Parser::new();
        let mut analyzer = Analyzer::new();

        let statement = parser
            .parse_single_statement(&mut lexer, &mut analyzer)
            .unwrap();
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
        let mut analyzer = Analyzer::new();

        let statement = parser
            .parse_single_statement(&mut lexer, &mut analyzer)
            .unwrap();
        assert_eq!(
            statement,
            Some(StatementNode::new(
                Opcode::Push(Value::Numeric(Number::Float(1.234))),
                Location { line: 1, column: 1 }
            ))
        );
    }

    #[test]
    fn test_parse_alias() {
        let mut lexer = Lexer::new(":console");
        let mut parser = Parser::new();

        let alias = parser.consume_alias(&mut lexer).unwrap();
        assert_eq!(alias, 0x00);
    }
}
