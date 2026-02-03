#[derive(Debug, Clone)]
pub enum Type {
    Any,
    Null,
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
    pub fields: Vec<Field>,
}

#[derive(Debug, Clone)]
pub struct Endpoint {
    pub id: String,
    pub method: String,
    pub path: String,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub request_body: Option<Type>,
    pub responses: Vec<(u16, Option<Type>)>,
}

#[derive(Debug, Clone)]
pub struct Schema {
    pub version: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub objects: Vec<Object>,
    pub endpoints: Vec<Endpoint>,
}

impl Schema {
    pub fn new() -> Self {
        Schema {
            version: None,
            title: None,
            description: None,
            objects: Vec::new(),
            endpoints: Vec::new(),
        }
    }

    pub fn verify(&self) -> Result<(), String> {
        let objects: std::collections::HashSet<_> = self.objects.iter().map(|o| &o.id).collect();

        for object in &self.objects {
            for field in &object.fields {
                match &field.field_type {
                    Type::Reference(ref_name) => {
                        if !objects.contains(ref_name) {
                            return Err(format!(
                                "Undefined object reference '{}' in field '{}' of object '{}'",
                                ref_name, field.name, object.id
                            ));
                        }
                    }
                    Type::Array(item_type) => {
                        if let Type::Reference(ref_name) = item_type.as_ref() {
                            if !objects.contains(ref_name) {
                                return Err(format!(
                                    "Undefined object reference '{}' in array field '{}' of object '{}'",
                                    ref_name, field.name, object.id
                                ));
                            }
                        }
                    }
                    Type::Mapping(key_type, value_type) => {
                        if let Type::Reference(ref_name) = key_type.as_ref() {
                            if !objects.contains(ref_name) {
                                return Err(format!(
                                    "Undefined object reference '{}' in mapping key of field '{}' of object '{}'",
                                    ref_name, field.name, object.id
                                ));
                            }
                        }
                        if let Type::Reference(ref_name) = value_type.as_ref() {
                            if !objects.contains(ref_name) {
                                return Err(format!(
                                    "Undefined object reference '{}' in mapping value of field '{}' of object '{}'",
                                    ref_name, field.name, object.id
                                ));
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        for endpoint in &self.endpoints {
            if let Some(body_type) = &endpoint.request_body {
                if let Type::Reference(ref_name) = body_type {
                    if !objects.contains(ref_name) {
                        return Err(format!(
                            "Undefined object reference '{}' in request body of endpoint '{}'",
                            ref_name, endpoint.id
                        ));
                    }
                }
            }

            for (_, response_type) in &endpoint.responses {
                if let Some(Type::Reference(ref_name)) = response_type {
                    if !objects.contains(ref_name) {
                        return Err(format!(
                            "Undefined object reference '{}' in response of endpoint '{}'",
                            ref_name, endpoint.id
                        ));
                    }
                }
            }
        }

        Ok(())
    }
}
