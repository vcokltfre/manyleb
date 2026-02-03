use crate::{Schema, Type};

fn format_type(t: &Type) -> String {
    match t {
        crate::Type::Any => "any".to_string(),
        crate::Type::Null => "null".to_string(),
        crate::Type::String => "string".to_string(),
        crate::Type::Integer => "integer".to_string(),
        crate::Type::Float => "float".to_string(),
        crate::Type::Boolean => "boolean".to_string(),
        crate::Type::Reference(name) => format!("[{}](#{})", name, name.to_lowercase()),
        crate::Type::Array(item_type) => format!("[]{}", format_type(item_type)),
        crate::Type::Mapping(key_type, value_type) => {
            format!("<{}, {}>", format_type(key_type), format_type(value_type))
        }
        crate::Type::Optional(inner_type) => format!("?{}", format_type(inner_type)),
    }
}

pub fn generate_docs(schema: &Schema) -> String {
    let mut docs = String::new();

    if let Some(title) = &schema.title {
        docs.push_str(&format!("# {}\n\n", title));
    }

    if let Some(version) = &schema.version {
        docs.push_str(&format!("**Version:** {}\n\n", version));
    }

    if let Some(description) = &schema.description {
        docs.push_str(&format!("{}\n\n", description));
    }

    docs.push_str("## Objects\n\n");
    for object in &schema.objects {
        docs.push_str(&format!("### {}\n\n", object.id));

        docs.push_str("| Property | Type | Description |\n");
        docs.push_str("|----------|------|-------------|\n");
        for field in &object.fields {
            docs.push_str(&format!(
                "| {} | {} | {} |\n",
                field.name,
                format_type(&field.field_type),
                field.description
            ));
        }
        docs.push_str("\n");
    }

    docs.push_str("## Endpoints\n\n");
    for endpoint in &schema.endpoints {
        docs.push_str(&format!(
            "### {} {}\n\n",
            endpoint.method.to_uppercase(),
            endpoint.path
        ));
        if let Some(desc) = &endpoint.description {
            docs.push_str(&format!("{}\n\n", desc));
        }

        if !endpoint.params.is_empty() {
            docs.push_str("#### Parameters\n\n");
            docs.push_str("| Name | Type | Description |\n");
            docs.push_str("|------|------|-------------|\n");
            for param in &endpoint.params {
                docs.push_str(&format!(
                    "| {} | {} | {} |\n",
                    param.name,
                    format_type(&param.field_type),
                    param.description
                ));
            }
            docs.push_str("\n");
        }

        if !endpoint.query.is_empty() {
            docs.push_str("#### Query Parameters\n\n");
            docs.push_str("| Name | Type | Description |\n");
            docs.push_str("|------|------|-------------|\n");
            for query in &endpoint.query {
                docs.push_str(&format!(
                    "| {} | {} | {} |\n",
                    query.name,
                    format_type(&query.field_type),
                    query.description
                ));
            }
            docs.push_str("\n");
        }
        docs.push_str("#### Responses\n\n");
        docs.push_str("| Status Code | Type |\n");
        docs.push_str("|-------------|------|\n");
        for (status, resp_type) in &endpoint.responses {
            let type_str = match resp_type {
                Some(t) => format_type(t),
                None => "No Content".to_string(),
            };
            docs.push_str(&format!("| {} | {} |\n", status, type_str));
        }
        docs.push_str("\n");
    }
    docs
}

pub fn generate_summary(schema: &Schema) -> String {
    let mut summary = String::new();

    summary.push_str("# Summary\n\n## Objects\n\n");

    for object in &schema.objects {
        summary.push_str(&format!("- `{}`\n", object.id));
    }

    summary.push_str("\n## Endpoints\n\n");

    for endpoint in &schema.endpoints {
        summary.push_str(&format!(
            "- `{} {}`\n",
            endpoint.method.to_uppercase(),
            endpoint.path,
        ));
    }

    summary
}
