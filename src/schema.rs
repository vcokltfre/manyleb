#[derive(Debug, Clone)]
pub enum Type {
    Any,
    String,
    Integer,
    Float,
    Boolean,
    Reference(String),
    Array(Box<Type>),
    Mapping(Box<Type>, Box<Type>),
    Optional(Box<Type>),
}

#[derive(Debug, Clone)]
pub struct Field {
    pub name: String,
    pub description: String,
    pub field_type: Type,
}

#[derive(Debug, Clone)]
pub struct Object {
    pub id: String,
    pub name: String,
    pub description: String,
    pub fields: Vec<Field>,
}

#[derive(Debug, Clone)]
pub struct Endpoint {
    pub id: String,
    pub method: String,
    pub path: String,
    pub description: String,
    pub request_body: Option<Type>,
    pub responses: Vec<(u16, Option<Type>)>,
}

#[derive(Debug, Clone)]
pub struct Schema {
    pub version: String,
    pub title: String,
    pub description: String,
    pub objects: Vec<Object>,
    pub endpoints: Vec<Endpoint>,
}
