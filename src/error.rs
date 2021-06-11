use std::fmt::Formatter;

#[derive(Debug)]
pub enum ErrorType {
    ExecutionLevelError,
    ArgumentError,
    UnknownEntityError,
}

pub struct UndeadError {
    error_type: ErrorType,
    line: usize,
    message: String,
}

impl UndeadError {
    pub fn new(error_type: ErrorType, line: usize, message: String) -> Self { Self { error_type, line, message } }

    pub fn execution_level_error(line: usize, message: String) -> Self {
        Self {
            error_type: ErrorType::ExecutionLevelError,
            line,
            message,
        }
    }
    pub fn argument_error(line: usize, message: String) -> Self {
        Self {
            error_type: ErrorType::ArgumentError,
            line,
            message,
        }
    }
    pub fn unknown_entity_error(line: usize, message: String) -> Self {
        Self {
            error_type: ErrorType::UnknownEntityError,
            line,
            message,
        }
    }
}

impl std::fmt::Debug for UndeadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} on line {}: {}", self.error_type, self.line, self.message)
    }
}
