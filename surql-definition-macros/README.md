# `surql-definition-macros`

`surql-definition-macros` is a Rust procedural macro crate that simplifies the process of generating SurrealDB table and field definitions. The crate provides the `SurQLDefinition` derive macro, which automatically creates SurrealDB schema queries based on annotated Rust structs.

## Table of Contents

1. [Features](#features)
2. [Installation](#installation)
3. [Usage](#usage)
4. [Examples](#examples)
    1. [Simple Usage](#simple-usage)
    2. [Advanced Usage](#advanced-usage)
5. [Validation](#validation)
6. [License](#license)
7. [Links](#links)

## Features

- Automatically generates SurrealDB table and field definitions from Rust structs.
- Supports flexible types, default values, assertions, and permissions.
- Provides runtime and compile-time query validation options.

## Installation

Add `surql-definition-macros` to your `Cargo.toml`:

```toml
[dependencies]
surql-definition-macros = "0.1.0"
```

## Usage

To use `surql-definition-macros`, simply derive `SurQLDefinition` on your struct and optionally use the provided attributes for customization:

- `surql_table`: Specifies the table name.
- `surql_field`: Configures field properties like type, default value, and assertions.
- `surql_field_permissions`: Sets field-level permissions.
- `surql_table_permissions`: Sets table-level permissions.
- `surql_query`: Defines a custom SurrealDB query.

## Examples

### Simple Usage

Here’s a basic example showcasing the use of `SurQLDefinition` to generate SurrealDB schema queries for a struct with various primitive types:

```rust
use surql_definition_macros::SurQLDefinition;

#[derive(SurQLDefinition)]
struct SimpleStruct {
    i32_val: i32,
    bool_val: bool,
    string_val: String,
}

assert_eq!(
    SimpleStruct::schema_query(),
    "DEFINE TABLE simple_struct; DEFINE FIELD i32_val ON simple_struct TYPE int; DEFINE FIELD bool_val ON simple_struct TYPE bool; DEFINE FIELD string_val ON simple_struct TYPE string;"
);
```

### Advanced Usage

In this example, the `ComplexStruct` demonstrates more advanced features like flexible types, custom field types, and default values:

```rust
use surql_definition_macros::SurQLDefinition;

#[derive(SurQLDefinition)]
struct ComplexStruct {
    #[surql_field(TYPE = "float", DEFAULT = "3.14")]
    float_val: f64,

    #[surql_field(TYPE = "string", FLEXIBLE)]
    flexible_string: String,
}

assert_eq!(
    ComplexStruct::schema_query(),
    "DEFINE TABLE complex_struct; DEFINE FIELD float_val ON complex_struct TYPE float DEFAULT 3.14; DEFINE FIELD flexible_string ON complex_struct FLEXIBLE TYPE string;"
);
```

## Validation

`surql-definition-macros` supports runtime and compile-time validation of generated queries using the `runtime_query_validation` and `compile_query_validation` features, respectively. These features rely on the `surrealdb-core` crate for query parsing.

To enable validation, update your `Cargo.toml`:

```toml
[dependencies]
surql-definition-macros = { version = "0.1.0", features = ["runtime_query_validation"] }
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Links

- [Crate on crates.io](https://crates.io/crates/surql-definition-macros)
- [Documentation](https://docs.rs/surql-definition-macros)
- [Repository](https://github.com/kochmaxence/surql-definition-macros)
- [SurrealDB](https://surrealdb.com/)
