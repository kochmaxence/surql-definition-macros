# surql-definition-macros

A Rust procedural macro for generating SurrealDB table and field definitions.

## Overview

`surql-definition-macros` provides a convenient way to define [SurrealDB](https://surrealdb.com/) tables and fields directly from Rust structs. The macro allows you to specify custom attributes, making it easy to set up table schemas and field properties using SurrealQL. It supports both auto-generated and explicitly defined table and field attributes.

## Goals

- Provide a user-friendly way to define SurrealDB tables and fields using Rust.
- Offer customization of table and field properties via attributes.
- Generate SurrealQL queries for defining table schemas.
- Allow specifying permissions for tables and fields using SurrealQL syntax.

## Non-Goals

- Execute the generated SurrealQL queries. `surql-definition-macros` does not depend on SurrealDB or interact with the database directly.

## Features

- **Auto-generate or explicitly define table definitions** from Rust structs.
- **Customize field properties** using attributes.
- **Specify table and field permissions** using SurrealQL syntax.
- **Easily integrate with SurrealDB** to manage your data models.

## Usage

Add `surql-definition-macros` as a dependency in your `Cargo.toml` file:

```toml
[dependencies]
surql-definition-macros = "0.1"
```

Then, derive `SurQLDefinition` for your struct:

```rust
use surql_definition_macros::SurQLDefinition;

#[derive(SurQLDefinition)]
#[surql_table("user")]
struct User {
    #[surql_field(TYPE = "string")]
    name: String,
    #[surql_field(TYPE = "int", DEFAULT = 30)]
    age: i32,
}
```

This will generate a SurrealQL query for the `User` struct:

```rust
impl User {
    pub fn schema_query() -> String {
        "DEFINE TABLE user; DEFINE FIELD name ON user TYPE string; DEFINE FIELD age ON user TYPE int DEFAULT 30;".to_string()
    }
}
```

### Attributes

- **`surql_table`**:
  - Specifies the table name.

- **`surql_query`**:
  - Specifies a custom SurrealQL query.

- **`surql_field`**:
  - **`TYPE`**: Specifies the field type.
  - **`DEFAULT`**: Specifies the default value for the field.
  - **`READONLY`**: Sets the field as read-only.
  - **`VALUE`**: Specifies the value for the field.
  - **`ASSERT`**: Specifies a condition for the field.
  - **`FLEXIBLE`**: Allows flexible typing for the field.

- **`surql_table_permissions`** / **`surql_field_permissions`**:
  - **`SELECT`**: Specifies the permissions for the select operation.
  - **`CREATE`**: Specifies the permissions for the create operation.
  - **`UPDATE`**: Specifies the permissions for the update operation.
  - **`DELETE`**: Specifies the permissions for the delete operation.

## Examples

### Basic Example

```rust
use surql_definition_macros::SurQLDefinition;

#[derive(SurQLDefinition)]
#[surql_table("user")]
struct User {
    #[surql_field(TYPE = "string")]
    name: String,
    #[surql_field(TYPE = "int", DEFAULT = 30)]
    age: i32,
}

assert_eq!(
    User::schema_query(),
    "DEFINE TABLE user; DEFINE FIELD name ON user TYPE string; DEFINE FIELD age ON user TYPE int DEFAULT 30;".to_string()
);
```

### Table Permissions

```rust
use surql_definition_macros::SurQLDefinition;

#[derive(SurQLDefinition)]
#[surql_table_permissions(SELECT = "user = true")]
struct User {
    #[surql_field(TYPE = "string")]
    name: String,
    #[surql_field(TYPE = "int")]
    age: i32,
}

assert_eq!(
    User::schema_query(),
    "DEFINE TABLE user PERMISSIONS FOR select user = true; DEFINE FIELD name ON user TYPE string; DEFINE FIELD age ON user TYPE int;".to_string()
);
```

### Flexible Field

```rust
use surql_definition_macros::SurQLDefinition;

#[surql_table("string_table")]
struct StringStruct {
    #[surql_field(TYPE = "string", FLEXIBLE)]
    flexible_string: String,
}

assert_eq!(
    StringStruct::schema_query(),
    "DEFINE TABLE string_table; DEFINE FIELD flexible_string ON string_table FLEXIBLE TYPE string;".to_string()
);
```

### Readonly Field

```rust
use surql_definition_macros::SurQLDefinition;

#[surql_table("readonly_table")]
struct ReadonlyStruct {
    #[surql_field(TYPE = "int", READONLY)]
    readonly_field: i32,
}

assert_eq!(
    ReadonlyStruct::schema_query(),
    "DEFINE TABLE readonly_table; DEFINE FIELD readonly_field ON readonly_table TYPE int READONLY;".to_string()
);
```

### Custom Query

```rust
use surql_definition_macros::SurQLDefinition;

#[surql_query("DEFINE TABLE person SCHEMAFULL;")]
struct Person {
    name: String,
}

assert_eq!(
    Person::schema_query(),
    "DEFINE TABLE person SCHEMAFULL; DEFINE FIELD name ON person TYPE string;".to_string()
);
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for details.

## Acknowledgments

Special thanks to the SurrealDB team for their excellent database technology.
