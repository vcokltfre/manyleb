use crate::{
    Schema,
    Object,
    Endpoint,
    Field,
    Type,
};

fn escape_string(s: &str) -> String {
    let mut result = String::new();

    for c in s.chars() {
        if c == '\n' {
            result.push_str("\\n");
        } else if c == '\t' {
            result.push_str("\\t");
        } else if c == '\r' {
            result.push_str("\\r");
        } else if c == '"' {
            result.push_str("\\\"");
        } else if c == '\\' {
            result.push_str("\\\\");
        } else {
            result.push(c);
        }
    }

    result
}

trait Formatable {
    fn format(&self) -> String;
}

trait FormatableField {
    fn format(&self) -> (String, String, String);
}

impl Formatable for Type {
    fn format(&self) -> String {
        match self {
            Type::Any => "any".to_string(),
            Type::Null => "null".to_string(),
            Type::String => "string".to_string(),
            Type::Integer => "integer".to_string(),
            Type::Float => "float".to_string(),
            Type::Boolean => "boolean".to_string(),
            Type::Reference(name) => escape_string(name),
            Type::Array(item_type) => format!("[]{}", item_type.format()),
            Type::Mapping(key_type, value_type) => {
                format!("<{}, {}>", key_type.format(), value_type.format())
            }
            Type::Optional(inner_type) => format!("?{}", inner_type.format()),
        }
    }
}

impl FormatableField for Field {
    fn format(&self) -> (String, String, String) {
        (
            self.name.clone(),
            self.field_type.format(),
            self.description.clone(),
        )
    }
}

impl Formatable for Object {
    fn format(&self) -> String {
        let mut longest_name = 0;
        let mut longest_type = 0;

        for field in &self.fields {
            let (name, field_type, _) = field.format();
            if name.len() > longest_name {
                longest_name = name.len();
            }
            if field_type.len() > longest_type {
                longest_type = field_type.len();
            }
        }

        let mut result = format!("object {} {{\n", self.id);

        for field in &self.fields {
            let (name, field_type, description) = field.format();
            result.push_str(&format!(
                "    prop {:width_name$} {:width_type$} \"{}\"\n",
                name,
                field_type,
                description,
                width_name = longest_name,
                width_type = longest_type
            ));
        }

        result.push_str("}\n");
        result
    }
}

impl Formatable for Endpoint {
    fn format(&self) -> String {
        let mut result = format!("route {} \"{}\" {{\n", self.method, escape_string(&self.path));

        if let Some(description) = &self.description {
            result.push_str(&format!("    description \"{}\"\n", escape_string(description)));
            result.push('\n');
        }

        for tag in &self.tags {
            result.push_str(&format!("    tag \"{}\"\n", escape_string(tag)));
        }
        if self.tags.len() > 0 {
            result.push('\n');
        }

        for param in &self.params {
            let (name, field_type, description) = param.format();
            result.push_str(&format!(
                "    param {} {} \"{}\"\n",
                name,
                field_type,
                escape_string(&description)
            ));
        }
        if self.params.len() > 0 {
            result.push('\n');
        }

        for query in &self.query {
            let (name, field_type, description) = query.format();
            result.push_str(&format!(
                "    query {} {} \"{}\"\n",
                name,
                field_type,
                escape_string(&description)
            ));
        }
        if self.query.len() > 0 {
            result.push('\n');
        }

        if let Some(body_type) = &self.request_body {
            result.push_str(&format!(
                "    body {}\n",
                body_type.format()
            ));
            result.push('\n');
        }

        for (status_code, response_type) in &self.responses {
            if let Some(resp_type) = response_type {
                result.push_str(&format!(
                    "    response {} {}\n",
                    status_code,
                    resp_type.format()
                ));
            } else {
                result.push_str(&format!("    response {}\n", status_code));
            }
        }

        result.push_str("}\n");
        result
    }
}

impl Formatable for Schema {
    fn format(&self) -> String {
        let mut result = String::new();

        if let Some(version) = &self.version {
            result.push_str(&format!("version \"{}\"\n", escape_string(version)));
        }

        if let Some(title) = &self.title {
            result.push_str(&format!("title \"{}\"\n", escape_string(title)));
        }

        if let Some(description) = &self.description {
            result.push_str(&format!("description \"{}\"\n", escape_string(description)));
        }

        result.push('\n');

        for object in &self.objects {
            result.push_str(&object.format());
            result.push('\n');
        }

        for endpoint in &self.endpoints {
            result.push_str(&endpoint.format());
            result.push('\n');
        }

        if result.ends_with("\n\n") {
            result.pop();
        }
        result
    }
}

pub fn format(schema: &Schema) -> String {
    schema.format()
}