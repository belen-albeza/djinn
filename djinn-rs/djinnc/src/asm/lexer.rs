use peekmore::{PeekMore, PeekMoreIterator};

use std::str::Chars;

use crate::asm::Location;
use crate::asm::token::{Token, TokenKind};

use crate::asm::{AssemblerError, Result};

#[derive(Debug, Clone)]
pub struct Lexer<'a> {
    source: PeekMoreIterator<Chars<'a>>,
    current_location: Location,
    start_location: Location,
    buffer: String,
    peeked_token: Option<Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source: source.chars().peekmore(),
            current_location: Location { line: 1, column: 1 },
            start_location: Location { line: 1, column: 1 },
            buffer: String::new(),
            peeked_token: None,
        }
    }

    pub fn current_location(&self) -> Location {
        self.current_location
    }

    // NOTE: do not peek twice without calling scan_token in between
    pub fn peek_token(&mut self) -> Result<Token> {
        if let Some(token) = &self.peeked_token {
            Ok(token.clone())
        } else {
            let token = self.scan_token()?;
            self.peeked_token = Some(token.clone());
            Ok(token)
        }
    }

    pub fn scan_token(&mut self) -> Result<Token> {
        if let Some(token) = self.peeked_token.take() {
            return Ok(token);
        }

        // clear the buffer and skip whitespace
        self.buffer.clear();
        self.skip_whitespace();
        self.buffer.clear();

        self.start_location = self.current_location;

        if let Some(x) = self.advance() {
            let kind = match x {
                // one-char tokens
                ':' => TokenKind::Colon,
                '~' => TokenKind::Tilde,
                '#' => TokenKind::Hash,
                '$' => TokenKind::Dollar,
                '@' => TokenKind::At,
                // multi-char tokens
                _ if x.is_ascii_digit() || x == '-' => self.scan_number_literal()?,
                _ if x.is_alphabetic() => self.scan_identifier_or_opcode()?,
                _ => {
                    return Err(AssemblerError::UnexpectedCharacter(self.start_location, x));
                }
            };
            Ok(build_token(kind, &self.buffer, self.start_location))
        } else {
            Ok(build_token(TokenKind::Eof, "", self.start_location))
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(x) = self.source.peek() {
            match *x {
                ' ' | '\t' => {
                    self.advance();
                }
                '\n' => {
                    self.current_location.line += 1;
                    self.current_location.column = 0;
                    self.advance();
                }
                ';' => self.skip_comment(),
                _ => break,
            }
        }
    }

    fn advance(&mut self) -> Option<char> {
        let res = self.source.next();
        if let Some(x) = res {
            self.buffer.push(x);
            self.current_location.column += 1;
        }
        res
    }

    fn skip_comment(&mut self) {
        while let Some(x) = self.source.peek() {
            if *x == '\n' {
                break;
            } else {
                self.advance();
            }
        }
    }

    fn scan_identifier_or_opcode(&mut self) -> Result<TokenKind> {
        while let Some(x) = self.source.peek() {
            if x.is_alphanumeric() || *x == '-' {
                self.advance();
            } else {
                break;
            }
        }
        let kind = opcode_for_lexeme(&self.buffer).unwrap_or(TokenKind::Id);
        Ok(kind)
    }

    fn scan_number_literal(&mut self) -> Result<TokenKind> {
        let mut has_decimal = false;

        while let Some(x) = self.source.peek() {
            match x {
                '.' if !has_decimal => {
                    has_decimal = true;
                    self.advance();
                }
                _ if x.is_ascii_digit() => {
                    self.advance();
                }
                _ => break,
            }
        }

        if has_decimal {
            Ok(TokenKind::Float(self.buffer.parse::<f64>().unwrap()))
        } else {
            Ok(TokenKind::Int(self.buffer.parse::<i32>().unwrap()))
        }
    }
}

fn opcode_for_lexeme(lexeme: &str) -> Option<TokenKind> {
    match lexeme {
        // process control
        "noop" => Some(TokenKind::NoOp),
        "yld" => Some(TokenKind::Yield),
        "spwn" => Some(TokenKind::Spawn),
        "kill" => Some(TokenKind::Kill),
        // device
        "dev" => Some(TokenKind::Dev),
        // stack
        "push" => Some(TokenKind::Push),
        "pop" => Some(TokenKind::Pop),
        "dup" => Some(TokenKind::Dup),
        // vars
        "stl" => Some(TokenKind::Stl),
        "ldl" => Some(TokenKind::Ldl),
        "stg" => Some(TokenKind::Stg),
        "ldg" => Some(TokenKind::Ldg),
        // flow control
        "jmp" => Some(TokenKind::Jmp),
        // values
        "true" => Some(TokenKind::Bool(true)),
        "false" => Some(TokenKind::Bool(false)),
        // alu
        "not" => Some(TokenKind::Not),
        "and" => Some(TokenKind::And),
        "or" => Some(TokenKind::Or),
        "xor" => Some(TokenKind::Xor),
        "add" => Some(TokenKind::Add),
        "sub" => Some(TokenKind::Sub),
        "mul" => Some(TokenKind::Mul),
        "div" => Some(TokenKind::Div),
        "mod" => Some(TokenKind::Mod),
        "eq" => Some(TokenKind::Eq),
        "neq" => Some(TokenKind::Neq),
        "lt" => Some(TokenKind::Lt),
        "leq" => Some(TokenKind::Leq),
        "gt" => Some(TokenKind::Gt),
        "geq" => Some(TokenKind::Geq),
        "inc" => Some(TokenKind::Inc),
        "dec" => Some(TokenKind::Dec),
        _ => None,
    }
}

fn build_token(kind: TokenKind, lexeme: &str, location: Location) -> Token {
    Token {
        kind,
        lexeme: lexeme.to_string(),
        location,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_eof() {
        let mut lexer = Lexer::new("");
        let token = lexer.scan_token().unwrap();
        assert_eq!(token.kind, TokenKind::Eof);
    }

    #[test]
    fn test_scan_returns_right_location() {
        let mut lexer = Lexer::new("noop\nyld noop");
        assert_eq!(
            lexer.scan_token().unwrap().location,
            Location { line: 1, column: 1 }
        );
        assert_eq!(
            lexer.scan_token().unwrap().location,
            Location { line: 2, column: 1 }
        );
        assert_eq!(
            lexer.scan_token().unwrap().location,
            Location { line: 2, column: 5 }
        );
        assert_eq!(lexer.scan_token().unwrap().kind, TokenKind::Eof);
    }

    #[test]
    fn test_skip_comment() {
        let input = r"; some
;comment with a fake opcode yld
yld ;actual opcode
; comment before EOF";
        let mut lexer = Lexer::new(input);

        assert_eq!(
            lexer.scan_token().unwrap(),
            Token {
                kind: TokenKind::Yield,
                lexeme: "yld".to_string(),
                location: Location { line: 3, column: 1 }
            }
        );
        assert_eq!(lexer.scan_token().unwrap().kind, TokenKind::Eof);
    }

    #[test]
    fn test_scan_opcodes() {
        let opcodes = vec![
            ("noop", TokenKind::NoOp),
            ("yld", TokenKind::Yield),
            ("spwn", TokenKind::Spawn),
            ("kill", TokenKind::Kill),
            ("push", TokenKind::Push),
            ("pop", TokenKind::Pop),
            ("dup", TokenKind::Dup),
            ("stl", TokenKind::Stl),
            ("ldl", TokenKind::Ldl),
            ("stg", TokenKind::Stg),
            ("ldg", TokenKind::Ldg),
            ("jmp", TokenKind::Jmp),
            ("not", TokenKind::Not),
            ("and", TokenKind::And),
            ("or", TokenKind::Or),
            ("xor", TokenKind::Xor),
            ("add", TokenKind::Add),
            ("sub", TokenKind::Sub),
            ("mul", TokenKind::Mul),
            ("div", TokenKind::Div),
            ("mod", TokenKind::Mod),
            ("inc", TokenKind::Inc),
            ("dec", TokenKind::Dec),
            ("eq", TokenKind::Eq),
            ("neq", TokenKind::Neq),
            ("lt", TokenKind::Lt),
            ("leq", TokenKind::Leq),
            ("gt", TokenKind::Gt),
            ("geq", TokenKind::Geq),
            ("dev", TokenKind::Dev),
        ];

        for (lexeme, kind) in opcodes {
            let mut lexer = Lexer::new(lexeme);
            assert_eq!(lexer.scan_token().unwrap().kind, kind);
        }
    }

    #[test]
    fn test_scan_process_declaration() {
        let mut lexer = Lexer::new("~main:");
        assert_eq!(lexer.scan_token().unwrap().kind, TokenKind::Tilde);
        assert_eq!(lexer.scan_token().unwrap().kind, TokenKind::Id);
        assert_eq!(lexer.scan_token().unwrap().kind, TokenKind::Colon);
        assert_eq!(lexer.scan_token().unwrap().kind, TokenKind::Eof);
    }

    #[test]
    fn test_scan_number_literal() {
        let mut lexer = Lexer::new("123");
        assert_eq!(lexer.scan_token().unwrap().kind, TokenKind::Int(123));
        let mut lexer = Lexer::new("123.456");
        assert_eq!(lexer.scan_token().unwrap().kind, TokenKind::Float(123.456));
        let mut lexer = Lexer::new("-123");
        assert_eq!(lexer.scan_token().unwrap().kind, TokenKind::Int(-123));
        let mut lexer = Lexer::new("-123.456");
        assert_eq!(lexer.scan_token().unwrap().kind, TokenKind::Float(-123.456));
    }

    #[test]
    fn test_scan_boolean_literal() {
        let mut lexer = Lexer::new("true");
        assert_eq!(lexer.scan_token().unwrap().kind, TokenKind::Bool(true));
        let mut lexer = Lexer::new("false");
        assert_eq!(lexer.scan_token().unwrap().kind, TokenKind::Bool(false));
    }

    #[test]
    fn test_scan_hash() {
        let mut lexer = Lexer::new("#");
        assert_eq!(lexer.scan_token().unwrap().kind, TokenKind::Hash);
    }

    #[test]
    fn test_scan_at() {
        let mut lexer = Lexer::new("@");
        assert_eq!(lexer.scan_token().unwrap().kind, TokenKind::At);
    }

    #[test]
    fn test_return_unexpected_character_error() {
        let mut lexer = Lexer::new("lorem*");
        assert_eq!(lexer.scan_token().unwrap().kind, TokenKind::Id); // lorem
        assert_eq!(
            lexer.scan_token().unwrap_err(),
            AssemblerError::UnexpectedCharacter(Location { line: 1, column: 6 }, '*')
        );
    }
}
