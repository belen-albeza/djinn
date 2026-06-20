use crate::asm::Location;
use crate::asm::token::TokenKind;

fn unexpected_token_expecting(expected: &[TokenKind]) -> String {
    match expected {
        [] => " Expecting any opcode.".to_string(),
        expected => format!(
            " Expecting: {}.",
            expected
                .iter()
                .map(|e| format!("`{e}`"))
                .collect::<Vec<_>>()
                .join(", ")
        ),
    }
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum AssemblerError {
    #[error("Unexpected character: {1}.")]
    UnexpectedCharacter(Location, char),
    #[error("Unexpected token: `{token}`.{}", unexpected_token_expecting(.expected))]
    UnexpectedToken {
        location: Location,
        token: String,
        expected: Vec<TokenKind>,
    },
    #[error("Main process not found.")]
    MainProcessNotFound(Location),
    #[error("Process already defined: {1}.")]
    ProcessAlreadyDefined(Location, String),
    #[error("Unknown alias: {1}.")]
    UnknownAlias(Location, String),
}

impl AssemblerError {
    pub fn location(&self) -> Location {
        match self {
            AssemblerError::UnexpectedCharacter(loc, _) => *loc,
            AssemblerError::UnexpectedToken { location, .. } => *location,
            AssemblerError::MainProcessNotFound(loc) => *loc,
            AssemblerError::ProcessAlreadyDefined(loc, _) => *loc,
            AssemblerError::UnknownAlias(loc, _) => *loc,
        }
    }

    pub fn message(&self) -> String {
        format!("{}", self)
    }

    pub fn with_location(self, location: Location) -> Self {
        match self {
            AssemblerError::UnexpectedCharacter(_, c) => {
                AssemblerError::UnexpectedCharacter(location, c)
            }
            AssemblerError::UnexpectedToken {
                token, expected, ..
            } => AssemblerError::UnexpectedToken {
                location,
                token,
                expected,
            },
            AssemblerError::MainProcessNotFound(_) => AssemblerError::MainProcessNotFound(location),
            AssemblerError::ProcessAlreadyDefined(_, name) => {
                AssemblerError::ProcessAlreadyDefined(location, name)
            }
            AssemblerError::UnknownAlias(_, alias) => AssemblerError::UnknownAlias(location, alias),
        }
    }
}

pub type Result<T> = std::result::Result<T, AssemblerError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unexpected_token_message_without_expected() {
        let err = AssemblerError::UnexpectedToken {
            location: Location { line: 1, column: 1 },
            token: "foo".into(),
            expected: vec![],
        };
        assert_eq!(
            err.message(),
            "Unexpected token: `foo`. Expecting any opcode."
        );
    }

    #[test]
    fn unexpected_token_message_with_expected() {
        let err = AssemblerError::UnexpectedToken {
            location: Location { line: 2, column: 3 },
            token: "bar".into(),
            expected: vec![TokenKind::NoOp, TokenKind::Yield],
        };
        assert_eq!(
            err.message(),
            "Unexpected token: `bar`. Expecting: `NOOP`, `YLD`."
        );
    }
}
