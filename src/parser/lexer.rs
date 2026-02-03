use crate::parser::token::{Token, TokenContext};

struct Lexer {
    input: String,
    line: usize,
    column: usize,
    index: usize,
}

fn strip_escapes(s: &str) -> Result<String, String> {
    let mut result = String::new();
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('n') => result.push('\n'),
                Some('t') => result.push('\t'),
                Some('r') => result.push('\r'),
                Some('"') => result.push('"'),
                Some('\\') => result.push('\\'),
                Some(other) => return Err(format!("Invalid escape sequence: \\{}", other)),
                None => return Err("Invalid escape sequence at end of string".to_string()),
            }
        } else {
            result.push(c);
        }
    }

    Ok(result)
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer {
            input,
            line: 1,
            column: 1,
            index: 0,
        }
    }

    fn get_token(&mut self) -> Result<Option<TokenContext>, String> {
        let char = match self.input.chars().nth(self.index) {
            Some(c) => c,
            None => return Ok(None),
        };

        match char {
            '"' => {
                let start_index = self.index + 1;
                self.index += 1;

                while let Some(c) = self.input.chars().nth(self.index) {
                    if c == '"' {
                        let string_value = &self.input[start_index..self.index];
                        self.index += 1;
                        return Ok(Some(TokenContext {
                            token: Token::String(strip_escapes(string_value)?),
                            line: self.line,
                            column: self.column,
                        }));
                    } else if c == '\\' {
                        self.index += 2;
                    } else {
                        self.index += 1;
                    }
                }

                Err("Unterminated string literal".to_string())
            }
            '1'..='9' => {
                let start_index = self.index;

                while let Some(c) = self.input.chars().nth(self.index) {
                    if c.is_digit(10) {
                        self.index += 1;
                    } else {
                        break;
                    }
                }

                let int_value = &self.input[start_index..self.index];
                let int_parsed = int_value.parse::<i64>().map_err(|e| e.to_string())?;

                Ok(Some(TokenContext {
                    token: Token::Integer(int_parsed),
                    line: self.line,
                    column: self.column,
                }))
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let start_index = self.index;

                while let Some(c) = self.input.chars().nth(self.index) {
                    if c.is_alphanumeric() || c == '_' {
                        self.index += 1;
                    } else {
                        break;
                    }
                }

                let ident_value = &self.input[start_index..self.index];

                let token = match ident_value {
                    "version" => Token::KWVersion,
                    "title" => Token::KWTitle,
                    "description" => Token::KWDescription,
                    "object" => Token::KWObject,
                    "property" => Token::KWProperty,
                    "route" => Token::KWRoute,
                    "body" => Token::KWBody,
                    "response" => Token::KWResponse,
                    "tag" => Token::KWTag,
                    _ => Token::Identifier(ident_value.to_string()),
                };

                Ok(Some(TokenContext {
                    token,
                    line: self.line,
                    column: self.column,
                }))
            }
            '?' => {
                self.index += 1;
                Ok(Some(TokenContext {
                    token: Token::QuestionMark,
                    line: self.line,
                    column: self.column,
                }))
            }
            '{' => {
                self.index += 1;
                Ok(Some(TokenContext {
                    token: Token::BlockStart,
                    line: self.line,
                    column: self.column,
                }))
            }
            '}' => {
                self.index += 1;
                Ok(Some(TokenContext {
                    token: Token::BlockEnd,
                    line: self.line,
                    column: self.column,
                }))
            }
            '[' => {
                self.index += 1;
                Ok(Some(TokenContext {
                    token: Token::ArayStart,
                    line: self.line,
                    column: self.column,
                }))
            }
            ']' => {
                self.index += 1;
                Ok(Some(TokenContext {
                    token: Token::ArayEnd,
                    line: self.line,
                    column: self.column,
                }))
            }
            '<' => {
                self.index += 1;
                Ok(Some(TokenContext {
                    token: Token::MapStart,
                    line: self.line,
                    column: self.column,
                }))
            }
            '>' => {
                self.index += 1;
                Ok(Some(TokenContext {
                    token: Token::MapEnd,
                    line: self.line,
                    column: self.column,
                }))
            }
            ',' => {
                self.index += 1;
                Ok(Some(TokenContext {
                    token: Token::Comma,
                    line: self.line,
                    column: self.column,
                }))
            }
            ' ' | '\t' | '\n' | '\r' => {
                if char == '\n' {
                    self.line += 1;
                    self.column = 1;
                } else {
                    self.column += 1;
                }
                self.index += 1;
                Ok(None)
            }
            _ => Err(format!("Unexpected character: {}", char)),
        }
    }

    pub fn tokenise(&mut self) -> Result<Vec<TokenContext>, String> {
        let mut tokens = Vec::new();

        while self.index < self.input.len() {
            if let Some(token) = self.get_token()? {
                tokens.push(token);
            }
        }

        Ok(tokens)
    }
}

pub fn tokenise(input: &str) -> Result<Vec<TokenContext>, String> {
    Lexer::new(input.into()).tokenise()
}
