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
                // multi-char tokens
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
}

fn opcode_for_lexeme(lexeme: &str) -> Option<TokenKind> {
    match lexeme {
        "noop" => Some(TokenKind::NoOp),
        "yld" => Some(TokenKind::Yield),
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
        let mut lexer = Lexer::new("noop yld");
        assert_eq!(lexer.scan_token().unwrap().kind, TokenKind::NoOp);
        assert_eq!(lexer.scan_token().unwrap().kind, TokenKind::Yield);
        assert_eq!(lexer.scan_token().unwrap().kind, TokenKind::Eof);
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
    fn test_return_unexpected_character_error() {
        let mut lexer = Lexer::new("lorem*");
        assert_eq!(lexer.scan_token().unwrap().kind, TokenKind::Id); // lorem
        assert_eq!(
            lexer.scan_token().unwrap_err(),
            AssemblerError::UnexpectedCharacter(Location { line: 1, column: 6 }, '*')
        );
    }
}
