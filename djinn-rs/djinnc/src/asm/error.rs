use super::Location;
use super::lexer::LexerError;

#[derive(Debug, thiserror::Error)]
pub enum AssemblerError {
    #[error(transparent)]
    Lexer(LexerError),
    #[error("Parser error at {0}: {1}")]
    Parser(Location, String),
}

impl AssemblerError {
    pub fn location(&self) -> Location {
        match self {
            AssemblerError::Lexer(err) => err.location(),
            AssemblerError::Parser(position, _) => *position,
        }
    }
    pub fn message(&self) -> String {
        match self {
            AssemblerError::Lexer(err) => err.message(),
            AssemblerError::Parser(_, message) => message.to_string(),
        }
    }
}

impl From<LexerError> for AssemblerError {
    fn from(err: LexerError) -> Self {
        Self::Lexer(err)
    }
}
