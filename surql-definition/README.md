# `surql-definition`

`surql-definition` is a Rust crate that provides a unified interface for generating SurrealDB table and field definitions. It re-exports functionality from `surql-definition-macros` and `surql-definition-core`.

## Table of Contents

1. [Features](#features)
2. [Installation](#installation)
3. [Usage](#usage)
4. [Examples](#examples)
5. [Validation](#validation)
6. [License](#license)
7. [Links](#links)

## Features

- Re-exports the `SurQLDefinition` derive macro from `surql-definition-macros`.
- Re-exports the `SurQLSchemaProducer` trait from `surql-definition-core`.
- Simplifies the process of defining and validating SurrealDB schemas.

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

### Defining a SurrealDB Table

The following example demonstrates how to use the `SurQLDefinition` macro to define a SurrealDB table:

```rust
use surql_definition::SurQLDefinition;

#[derive(SurQLDefinition)]
struct User {
    id: u64,
    name: String,
    email: String,
}

assert_eq!(
    User::schema_query(),
    "DEFINE TABLE user; DEFINE FIELD id ON user TYPE int; DEFINE FIELD name ON user TYPE string; DEFINE FIELD email ON user TYPE string;"
);
```

### Using `SurQLSchemaProducer`

The `SurQLSchemaProducer` trait allows you to manually implement a schema query for a struct:

```rust
use surql_definition::SurQLSchemaProducer;

struct Product;

impl SurQLSchemaProducer for Product {
    fn schema_query() -> &'static str {
        "DEFINE TABLE product;"
    }
}

assert_eq!(Product::schema_query(), "DEFINE TABLE product;");
```

## Validation

`surql-definition` supports runtime and compile-time validation of generated queries through the features provided by `surql-definition-macros`.

To enable validation, update your `Cargo.toml`:

```toml
[dependencies]
surql-definition = { version = "0.2.1", features = ["runtime_query_validation"] }
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Links

- [Documentation](https://docs.rs/surql-definition)
- [Repository](https://github.com/kochmaxence/surql-definition)
- [surql-definition-macros](https://github.com/kochmaxence/surql-definition-macros)
- [surql-definition-core](https://github.com/kochmaxence/surql-definition-core)
