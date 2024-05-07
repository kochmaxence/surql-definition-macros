# `surql-definition`

`surql-definition` is a unified Rust library for generating SurrealDB table and field definitions. The library is composed of three crates:

1. `surql-definition-core`: Provides core utilities and traits for generating SurrealDB schema queries.
2. `surql-definition-macros`: A procedural macro crate for generating SurrealDB table and field definitions.
3. `surql-definition`: A high-level crate that re-exports functionality from the other two crates.

## Table of Contents

1. [Features](#features)
2. [Crate Relationships](#crate-relationships)
3. [Installation](#installation)
4. [Usage](#usage)
5. [Examples](#examples)
    1. [Defining a Table with Default Settings](#defining-a-table-with-default-settings)
    2. [Customizing Field Types and Attributes](#customizing-field-types-and-attributes)
    3. [Setting Permissions](#setting-permissions)
    4. [Customizing Table Names and Queries](#customizing-table-names-and-queries)
6. [Validation](#validation)
7. [Feature Flags](#feature-flags)
8. [License](#license)
9. [Links](#links)

## Features

- Automatically generates SurrealDB table and field definitions from Rust structs.
- Supports flexible types, default values, assertions, and permissions.
- Provides runtime and compile-time query validation options.

## Crate Relationships

### `surql-definition-core`

`surql-definition-core` provides the core functionality for SurrealDB schema generation. It includes the `SurQLSchemaProducer` trait, which defines a method for generating schema queries, and a utility function `to_snake_case` for converting strings to snake case.

### `surql-definition-macros`

`surql-definition-macros` is a procedural macro crate that facilitates the creation of SurrealDB schemas through the `SurQLDefinition` derive macro. It builds upon the functionality provided by `surql-definition-core`.

### `surql-definition`

`surql-definition` serves as the high-level interface for the other two crates. It re-exports both the `SurQLDefinition` derive macro from `surql-definition-macros` and the `SurQLSchemaProducer` trait from `surql-definition-core`.

## Installation

Add `surql-definition` to your `Cargo.toml`:

```toml
[dependencies]
surql-definition = "0.2.1"
```

## Usage

To use `surql-definition`, import the relevant items as needed:

```rust
use surql_definition::{SurQLDefinition, SurQLSchemaProducer};
```

## Examples

### Defining a Table with Default Settings

The following example demonstrates how to use the `SurQLDefinition` macro to define a SurrealDB table with default settings:

```rust
use surql_definition::SurQLDefinition;
use surrealdb::sql::Thing;

#[derive(SurQLDefinition)]
struct User {
    #[surql_field(TYPE = "record")]
    id: Thing,
    name: String,
    email: String,
}

assert_eq!(
    User::schema_query(),
    "DEFINE TABLE user; DEFINE FIELD id ON user TYPE record; DEFINE FIELD name ON user TYPE string; DEFINE FIELD email ON user TYPE string;"
);
```

### Customizing Field Types and Attributes

You can customize field types, set default values, and add assertions using the `surql_field` attribute:

```rust
use surql_definition::SurQLDefinition;
use surrealdb::sql::Thing;

#[derive(SurQLDefinition)]
struct Product {
    #[surql_field(TYPE = "record")]
    id: Thing,
    #[surql_field(TYPE = "string")]
    name: String,
    #[surql_field(TYPE = "number", DEFAULT = "10.99")]
    price: f64,
    #[surql_field(TYPE = "bool", ASSERT = "$value == true")]
    available: bool,
    #[surql_field(TYPE = "string", FLEXIBLE)]
    description: String,
    #[surql_field(TYPE = "number", READONLY)]
    rating: f64,
}

assert_eq!(
    Product::schema_query(),
    "DEFINE TABLE product; \
    DEFINE FIELD id ON product TYPE record; \
    DEFINE FIELD name ON product TYPE string; \
    DEFINE FIELD price ON product TYPE number DEFAULT 10.99; \
    DEFINE FIELD available ON product TYPE bool ASSERT $value == true; \
    DEFINE FIELD description ON product FLEXIBLE TYPE string; \
    DEFINE FIELD rating ON product TYPE number READONLY;"
);
```

### Setting Permissions

You can define permissions for fields or the entire table:

```rust
use surql_definition::SurQLDefinition;
use surrealdb::sql::Thing;

#[derive(SurQLDefinition)]
#[surql_table_permissions("FOR select WHERE $auth.role == 'admin'")]
struct Order {
    #[surql_field(TYPE = "record")]
    id: Thing,
    #[surql_field(TYPE = "number", PERMISSIONS = "FOR update WHERE $auth.role == 'admin'")]
    amount: f64,
    #[surql_field(TYPE = "string", PERMISSIONS = "FOR delete WHERE $auth.role == 'admin'")]
    status: String,
}

assert_eq!(
    Order::schema_query(),
    "DEFINE TABLE order PERMISSIONS FOR select WHERE $auth.role == 'admin'; \
    DEFINE FIELD id ON order TYPE record; \
    DEFINE FIELD amount ON order TYPE number PERMISSIONS FOR update WHERE $auth.role == 'admin'; \
    DEFINE FIELD status ON order TYPE string PERMISSIONS FOR delete WHERE $auth.role == 'admin';"
);
```

### Customizing Table Names and Queries

You can also customize table names and define custom queries:

```rust
use surql_definition::SurQLDefinition;
use surrealdb::sql::Thing;

#[derive(SurQLDefinition)]
#[surql_table("custom_table_name")]
#[surql_query("DEFINE TABLE custom_table_name (id INT, name STRING);")]
struct CustomTable {
    #[surql_field(TYPE = "record")]
    id: Thing,
    #[surql_field(TYPE = "string")]
    name: String,
}
```

## Validation

`surql-definition` supports runtime and compile-time validation of generated queries through the features provided by `surql-definition-macros`.

To enable validation, update your `Cargo.toml`:

```toml
[dependencies]
surql-definition = { version = "0.2.1", features = ["runtime_query_validation"] }
```

## Feature Flags

### `runtime_query_validation`

The `runtime_query_validation` feature enables validation of the generated SurrealDB queries at runtime. This feature imports `surrealdb-core` to perform the query validation.

#### Example

```toml
[dependencies]
surql-definition = { version = "0.2.1", features = ["runtime_query_validation"] }
```

### `compile_query_validation`

The `compile_query_validation` feature enables validation of the generated SurrealDB queries at compile time. This feature also imports `surrealdb-core` for query validation.

#### Example

```toml
[dependencies]
surql-definition = { version = "0.2.1", features = ["compile_query_validation"] }
```

### Default

If no feature flags are set, `surrealdb-core` is not imported. This is useful when validation is not required.

#### Example

```toml
[dependencies]
surql-definition = "0.2.1"
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Links

- [surql-definition-core](https://github.com/kochmaxence/surql-definition-macros/tree/main/surql-definition-core)
- [surql-definition-macros](https://github.com/kochmaxence/surql-definition-macros/tree/main/surql-definition-macros)
- [surql-definition](https://github.com/kochmaxence/surql-definition-macros/tree/main/surql-definition)
