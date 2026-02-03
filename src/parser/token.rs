#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    KWVersion,
    KWTitle,
    KWDescription,
    KWObject,
    KWProperty,
    KWRoute,
    KWBody,
    KWResponse,
    KWTag,
    KWParam,
    KWQuery,

    Identifier(String),
    String(String),
    Integer(i64),

    BlockStart,
    BlockEnd,
    ArayStart,
    ArayEnd,
    MapStart,
    MapEnd,
    Comma,
    QuestionMark,
}

#[derive(Debug, Clone)]
pub struct TokenContext {
    pub token: Token,
    pub line: usize,
    pub column: usize,
}

impl TokenContext {
    pub fn error_message(&self, message: &str) -> String {
        format!("Error at line {}, column {}: {}", self.line, self.column, message)
    }
}
