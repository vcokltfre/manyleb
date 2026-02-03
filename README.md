# Manyleb

Manyleb (Welsh for "specification") is a DSL designed for specifying HTTP APIs in a concise but precise manner.

## Writing Specifications

### Basic Structure

A Manyleb specification consists of some metadata, followed by object definitions, followed by route definitions.

```manyleb
version "1.0.0"
title "Sample API"
description "This is a sample API specification."

object User {
    property id   string   "Unique identifier for the user"
    property name string   "Name of the user"
    property age  ?integer "Age of the user"
}

route get "/users" {
    description "Retrieve a list of users"

    tag "Users"

    response 200 []User
}

route get "/users/:id" {
    description "Retrieve a specific user by ID"

    tag "Users"

    response 200 User
    response 404
}
```

### Metadata

Metadata fields include `version`, `title`, and `description`.

### Types

Types can be primitive, composite, or user-defined objects.

Primitive types are:

- `string` / `str`
- `integer` / `int`
- `boolean` / `bool`
- `float`
- `null`

Composite types are:

- Arrays: `[]typename`
- Maps: `<tkey, tvalue>`

User defined objects are declared using the `object` keyword, and referenced in types by their name, e.g., `User`.

Types can be marked as optional by prefixing them with `?`, e.g., `?string`.

### Objects

Objects are defined using the `object` keyword, followed by properties.

Properties follow the format `property name type "description"`.

### Routes

Routes are defined using the `route` keyword, followed by the HTTP method and path.

Routes can have descriptions, tags, params, query params, and multiple responses.

Routes follow the format:

```manyleb
route method "/path" {
    description "Route description"

    tag "TagName"

    param name type "description"

    query name type "description"

    response status_code type
}
```

## CLI Usage

### Format Command

To format a Manyleb specification file, use the following command:

```bash
manyleb format path/to/spec.manyleb
```

### Verify Command

To verify a Manyleb specification file for correctness, use the following command:

```bash
manyleb verify path/to/spec.manyleb
```

## License

Manyleb is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
