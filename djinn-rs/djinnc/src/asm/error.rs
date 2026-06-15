use super::Location;

#[derive(Debug, thiserror::Error)]
pub enum AssemblerError {
    #[error("Lexer error at {position}: {message}")]
    LexerError { position: Location, message: String },
}

impl AssemblerError {
    pub fn location(&self) -> Location {
        match self {
            AssemblerError::LexerError { position, .. } => *position,
        }
    }
    pub fn message(&self) -> String {
        match self {
            AssemblerError::LexerError { message, .. } => message.to_string(),
        }
    }
}
