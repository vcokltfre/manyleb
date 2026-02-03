use std::collections::HashSet;

use crate::{
    Endpoint, Field, Object, Schema, Type,
    parser::{
        lexer::tokenise,
        token::{Token, TokenContext},
    },
};

struct Parser {
    tokens: Vec<TokenContext>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<TokenContext>) -> Self {
        Parser {
            tokens,
            position: 0,
        }
    }

    fn error_message(&self, message: &str) -> String {
        if let Some(token_context) = self.tokens.get(self.position) {
            token_context.error_message(message)
        } else {
            message.to_string()
        }
    }

    fn expect_keyword(&mut self, expected: Token) -> Result<(), String> {
        if let Some(token_context) = self.tokens.get(self.position) {
            if token_context.token == expected {
                self.position += 1;
                return Ok(());
            }
        }

        match expected {
            Token::KWVersion => Err(self.error_message("Expected 'version''")),
            Token::KWTitle => Err(self.error_message("Expected 'title'")),
            Token::KWDescription => Err(self.error_message("Expected 'description'")),
            Token::KWObject => Err(self.error_message("Expected 'object'")),
            Token::KWProperty => Err(self.error_message("Expected 'property'")),
            Token::KWRoute => Err(self.error_message("Expected 'route'")),
            Token::KWBody => Err(self.error_message("Expected 'body'")),
            Token::KWResponse => Err(self.error_message("Expected 'response'")),
            _ => unreachable!(),
        }
    }

    fn expect_identifier(&mut self) -> Result<String, String> {
        if let Some(token_context) = self.tokens.get(self.position) {
            if let Token::Identifier(name) = &token_context.token {
                self.position += 1;
                return Ok(name.clone());
            }
        }

        Err(self.error_message("Expected identifier."))
    }

    fn expect_string(&mut self) -> Result<String, String> {
        if let Some(token_context) = self.tokens.get(self.position) {
            if let Token::String(value) = &token_context.token {
                self.position += 1;
                return Ok(value.clone());
            }
        }

        Err(self.error_message("Expected string."))
    }

    fn expect_block_start(&mut self) -> Result<(), String> {
        if let Some(token_context) = self.tokens.get(self.position) {
            if let Token::BlockStart = &token_context.token {
                self.position += 1;
                return Ok(());
            }
        }

        Err(self.error_message("Expected block start '{'."))
    }

    fn expect_type(&mut self) -> Result<Type, String> {
        self.position += 1;

        if let Some(token_context) = self.tokens.get(self.position - 1) {
            match &token_context.token {
                Token::Identifier(type_name) => match type_name.as_str() {
                    "any" => return Ok(Type::Any),
                    "null" => return Ok(Type::Null),
                    "string" | "str" => return Ok(Type::String),
                    "integer" | "int" => return Ok(Type::Integer),
                    "float" => return Ok(Type::Float),
                    "boolean" | "bool" => return Ok(Type::Boolean),
                    _ => return Ok(Type::Reference(type_name.clone())),
                },
                Token::ArayStart => {
                    self.expect_keyword(Token::ArayEnd)?;
                    let item_type = self.expect_type()?;
                    return Ok(Type::Array(Box::new(item_type)));
                }
                Token::MapStart => {
                    let key_type = self.expect_type()?;
                    self.expect_keyword(Token::Comma)?;
                    let value_type = self.expect_type()?;
                    self.expect_keyword(Token::MapEnd)?;
                    return Ok(Type::Mapping(Box::new(key_type), Box::new(value_type)));
                }
                Token::QuestionMark => {
                    let base_type = self.expect_type()?;
                    return Ok(Type::Optional(Box::new(base_type)));
                }
                _ => {
                    return Err(self.error_message("Expected type."));
                }
            }
        }

        Err(self.error_message("Expected type."))
    }

    fn expect_property(&mut self) -> Result<Field, String> {
        let name = self.expect_identifier()?;
        let field_type = self.expect_type()?;
        let description = self.expect_string()?;

        Ok(Field {
            name,
            description,
            field_type,
        })
    }

    fn parse_object(&mut self) -> Result<Object, String> {
        let id = self.expect_identifier()?;
        let mut fields = Vec::new();

        self.expect_block_start()?;

        while let Some(token) = self.tokens.get(self.position) {
            match &token.token {
                Token::KWProperty => {
                    self.position += 1;
                    let field = self.expect_property()?;
                    fields.push(field);
                }
                Token::BlockEnd => {
                    self.position += 1;
                    break;
                }
                _ => {
                    return Err(self.error_message(
                        format!("Unexpected token in object: {:?}", token.token).as_str(),
                    ));
                }
            }
        }

        Ok(Object { id, fields })
    }

    fn parse_route(&mut self) -> Result<Endpoint, String> {
        let method = self.expect_identifier()?;
        let path = self.expect_string()?;

        self.expect_block_start()?;

        let mut description = None;
        let mut tags = Vec::new();
        let mut params = Vec::new();
        let mut query = Vec::new();
        let mut body = None;
        let mut responses = Vec::new();

        while let Some(token) = self.tokens.get(self.position) {
            match &token.token {
                Token::KWDescription => {
                    self.position += 1;
                    description = Some(self.expect_string()?);
                }
                Token::KWTag => {
                    self.position += 1;
                    let tag = self.expect_string()?;
                    tags.push(tag);
                }
                Token::KWBody => {
                    self.position += 1;
                    let body_type = self.expect_type()?;
                    body = Some(body_type);
                }
                Token::KWParam => {
                    self.position += 1;
                    let param = self.expect_property()?;
                    params.push(param);
                }
                Token::KWQuery => {
                    self.position += 1;
                    let query_param = self.expect_property()?;
                    query.push(query_param);
                }
                Token::KWResponse => {
                    self.position += 1;
                    if let Some(status_token) = self.tokens.get(self.position) {
                        if let Token::Integer(status_code) = status_token.token {
                            self.position += 1;

                            let current_pos = self.position;

                            if let Ok(t) = self.expect_type() {
                                responses.push((status_code as u16, Some(t)));
                                continue;
                            }

                            self.position = current_pos;

                            responses.push((status_code as u16, None));
                        } else {
                            return Err(self
                                .error_message("Expected integer status code after 'response'."));
                        }
                    } else {
                        return Err(self.error_message("Unexpected end of input after 'response'."));
                    }
                }
                Token::BlockEnd => {
                    self.position += 1;
                    break;
                }
                _ => {
                    return Err(self.error_message(
                        format!("Unexpected token in route: {:?}", token.token).as_str(),
                    ));
                }
            }
        }

        Ok(Endpoint {
            id: format!("{}_{}", method, path),
            method,
            path,
            description,
            tags,
            params,
            query,
            request_body: body,
            responses,
        })
    }

    fn parse_resolve(&mut self) -> Result<String, String> {
        if let Some(token_context) = self.tokens.get(self.position) {
            if let Token::String(s) = &token_context.token {
                self.position += 1;
                return Ok(s.clone());
            }
        }

        Err(self.error_message("Expected string after '@' token."))
    }

    pub fn parse(&mut self, resolve: bool, visited: &mut HashSet<String>) -> Result<Schema, String> {
        let mut schema = Schema::new();

        while self.position < self.tokens.len() {
            let token = &self.tokens[self.position];

            match &token.token {
                Token::Resolve => {
                    self.position += 1;
                    let resolve_path = self.parse_resolve()?;
                    if !resolve {
                        continue
                    }

                    if visited.contains(&resolve_path) {
                        return Err(format!("Circular reference detected for '{}'", resolve_path));
                    }

                    visited.insert(resolve_path.clone());
                    let resolved_input = std::fs::read_to_string(&resolve_path)
                        .map_err(|e| format!("Failed to read file '{}': {}", resolve_path, e))?;
                    let resolved_tokens = tokenise(&resolved_input)?;
                    let mut resolved_parser = Parser::new(resolved_tokens);
                    let resolved_schema = resolved_parser.parse(true, visited)?;

                    schema.objects.extend(resolved_schema.objects);
                    schema.endpoints.extend(resolved_schema.endpoints);
                }
                Token::KWVersion => {
                    if schema.version.is_some() {
                        return Err(self.error_message("Multiple version declarations found."));
                    }

                    self.position += 1;
                    if let Some(next_token) = self.tokens.get(self.position) {
                        if let Token::String(version) = &next_token.token {
                            schema.version = Some(version.clone());
                            self.position += 1;
                        } else {
                            return Err(
                                self.error_message("Expected string after 'version' keyword.")
                            );
                        }
                    } else {
                        return Err(
                            self.error_message("Unexpected end of input after 'version' keyword.")
                        );
                    }
                }
                Token::KWTitle => {
                    if schema.title.is_some() {
                        return Err(self.error_message("Multiple title declarations found."));
                    }

                    self.position += 1;
                    if let Some(next_token) = self.tokens.get(self.position) {
                        if let Token::String(title) = &next_token.token {
                            schema.title = Some(title.clone());
                            self.position += 1;
                        } else {
                            return Err(
                                self.error_message("Expected string after 'title' keyword.")
                            );
                        }
                    } else {
                        return Err(
                            self.error_message("Unexpected end of input after 'title' keyword.")
                        );
                    }
                }
                Token::KWDescription => {
                    if schema.description.is_some() {
                        return Err(self.error_message("Multiple description declarations found."));
                    }

                    self.position += 1;
                    if let Some(next_token) = self.tokens.get(self.position) {
                        if let Token::String(description) = &next_token.token {
                            schema.description = Some(description.clone());
                            self.position += 1;
                        } else {
                            return Err(
                                self.error_message("Expected string after 'description' keyword.")
                            );
                        }
                    } else {
                        return Err(self.error_message(
                            "Unexpected end of input after 'description' keyword.",
                        ));
                    }
                }
                Token::KWObject => {
                    self.position += 1;
                    let object = self.parse_object()?;
                    schema.objects.push(object);
                }
                Token::KWRoute => {
                    self.position += 1;
                    let endpoint = self.parse_route()?;
                    schema.endpoints.push(endpoint);
                }
                _ => {
                    return Err(token.error_message("Unexpected token at top level."));
                }
            }
        }

        Ok(schema)
    }
}

pub fn parse(input: &str, resolve: bool) -> Result<Schema, String> {
    Parser::new(tokenise(input)?).parse(resolve, &mut HashSet::<String>::new())
}
