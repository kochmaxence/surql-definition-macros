#[cfg(test)]
#[allow(dead_code)]
mod tests {

    use surql_definition_core::SurQLSchemaProducer;
    use surql_definition_macros::SurQLDefinition;

    #[test]
    fn test_simple_auto() {
        #[derive(SurQLDefinition)]
        struct SimplePrimitivesAuto {
            // Signed integers
            i8_val: i8,
            i16_val: i16,
            i32_val: i32,
            i64_val: i64,
            i128_val: i128,
            isize_val: isize,

            // Unsigned integers
            u8_val: u8,
            u16_val: u16,
            u32_val: u32,
            u64_val: u64,
            u128_val: u128,
            usize_val: usize,

            // Floating-point numbers
            f32_val: f32,
            f64_val: f64,

            // Boolean
            bool_val: bool,

            // Character
            char_val: char,
        }

        assert_eq!(SimplePrimitivesAuto::schema_query(), "DEFINE TABLE simple_primitives_auto; DEFINE FIELD i8_val ON simple_primitives_auto TYPE int; DEFINE FIELD i16_val ON simple_primitives_auto TYPE int; DEFINE FIELD i32_val ON simple_primitives_auto TYPE int; DEFINE FIELD i64_val ON simple_primitives_auto TYPE int; DEFINE FIELD i128_val ON simple_primitives_auto TYPE int; DEFINE FIELD isize_val ON simple_primitives_auto TYPE int; DEFINE FIELD u8_val ON simple_primitives_auto TYPE int; DEFINE FIELD u16_val ON simple_primitives_auto TYPE int; DEFINE FIELD u32_val ON simple_primitives_auto TYPE int; DEFINE FIELD u64_val ON simple_primitives_auto TYPE int; DEFINE FIELD u128_val ON simple_primitives_auto TYPE int; DEFINE FIELD usize_val ON simple_primitives_auto TYPE int; DEFINE FIELD f32_val ON simple_primitives_auto TYPE float; DEFINE FIELD f64_val ON simple_primitives_auto TYPE float; DEFINE FIELD bool_val ON simple_primitives_auto TYPE bool; DEFINE FIELD char_val ON simple_primitives_auto TYPE string;");
    }

    #[test]
    fn test_complex_auto() {
        // use std::collections::HashMap;
        // use std::rc::Rc;

        #[derive(SurQLDefinition)]
        struct ComplexAuto {
            // Arrays
            // array_fixed: [i32; 3],
            // array_dynamic: Box<[i32]>,
            // array_slice: &'static [i32],

            // Tuples
            // tuple_simple: (i32, f64),
            // tuple_nested: (i32, (f64, char)),
            // tuple_triple: (i32, f64, char),

            // Vectors
            vector: Vec<i32>,
            // vector_from_slice: Vec<&'static str>,
            // vector_of_tuples: Vec<(i32, f64)>,

            // HashMaps
            // hashmap: HashMap<String, i32>,
            // hashmap_tuple_key: HashMap<(i32, char), f64>,

            // Option
            optional_some: Option<i32>,
            optional_none: Option<i32>,
            // Result
            // result_ok: Result<f64, String>,
            // result_err: Result<f64, String>,

            // Box
            // boxed: Box<i32>,

            // Rc
            // rc: Rc<String>,

            // Arc
            // arc: Arc<String>,
        }

        assert_eq!(ComplexAuto::schema_query(), "DEFINE TABLE complex_auto; DEFINE FIELD vector ON complex_auto TYPE array<int>; DEFINE FIELD optional_some ON complex_auto TYPE option<int>; DEFINE FIELD optional_none ON complex_auto TYPE option<int>;");
    }

    #[test]
    fn test_explicit_name() {
        #[derive(SurQLDefinition)]
        #[surql_table("some_other_name")]
        struct ExplicitName;

        assert_eq!(
            ExplicitName::schema_query(),
            "DEFINE TABLE some_other_name;"
        );
    }

    #[test]
    fn test_explicit_type() {
        #[derive(SurQLDefinition)]
        struct ExplicitType {
            #[surql_field(TYPE = "string")]
            explicit_type: f64,
        }

        assert_eq!(
            ExplicitType::schema_query(),
            "DEFINE TABLE explicit_type; DEFINE FIELD explicit_type ON explicit_type TYPE string;"
        );
    }

    #[test]
    fn test_simple_struct() {
        #[derive(SurQLDefinition)]
        #[surql_table("test_table")]
        struct SimpleStruct {
            // Type inferred from Rust type
            simple_int: i32,

            // Explicitly specified type
            #[surql_field(TYPE = "int")]
            specified_int: i32,

            // Default and assertion
            #[surql_field(DEFAULT = "42", ASSERT = "$value > 0")]
            default_int: i32,
        }
        let expected = "DEFINE TABLE test_table; DEFINE FIELD simple_int ON test_table TYPE int; DEFINE FIELD specified_int ON test_table TYPE int; DEFINE FIELD default_int ON test_table TYPE int DEFAULT 42 ASSERT $value > 0;";
        assert_eq!(SimpleStruct::schema_query(), expected);
    }

    #[test]
    fn test_complex_struct() {
        #[derive(SurQLDefinition)]
        struct ComplexStruct {
            // Type inferred from Rust type
            option_string: Option<String>,

            complex_vec: Vec<Vec<Vec<i32>>>,

            complex_mixed: Option<Vec<Option<String>>>,
        }

        assert_eq!(ComplexStruct::schema_query(), "DEFINE TABLE complex_struct; DEFINE FIELD option_string ON complex_struct TYPE option<string>; DEFINE FIELD complex_vec ON complex_struct TYPE array<array<array<int>>>; DEFINE FIELD complex_mixed ON complex_struct TYPE option<array<option<string>>>;");
    }

    #[test]
    fn test_float_struct() {
        #[derive(SurQLDefinition)]
        #[surql_table("float_table")]
        struct FloatStruct {
            // Type inferred from Rust type
            floating_num: f64,

            // Explicitly specified type
            #[surql_field(TYPE = "float")]
            specified_float: f64,
        }

        let expected = "DEFINE TABLE float_table; DEFINE FIELD floating_num ON float_table TYPE float; DEFINE FIELD specified_float ON float_table TYPE float;";
        assert_eq!(FloatStruct::schema_query(), expected);
    }

    #[test]
    fn test_string_struct() {
        #[derive(SurQLDefinition)]
        #[surql_table("string_table")]
        struct StringStruct {
            // Type inferred from Rust type
            simple_string: String,

            // Explicitly specified type
            #[surql_field(TYPE = "string")]
            specified_string: String,

            // Flexible field
            #[surql_field(TYPE = "string", FLEXIBLE)]
            flexible_string: String,
        }
        let expected = "DEFINE TABLE string_table; DEFINE FIELD simple_string ON string_table TYPE string; DEFINE FIELD specified_string ON string_table TYPE string; DEFINE FIELD flexible_string ON string_table FLEXIBLE TYPE string;";
        assert_eq!(StringStruct::schema_query(), expected);
    }

    #[test]
    fn test_bool_struct() {
        #[derive(SurQLDefinition)]
        #[surql_table("bool_table")]
        struct BoolStruct {
            // Type inferred from Rust type
            simple_bool: bool,

            // Explicitly specified type
            #[surql_field(TYPE = "bool")]
            specified_bool: bool,

            // Value and assertion
            #[surql_field(VALUE = "true", ASSERT = "$value == true")]
            value_bool: bool,
        }
        let expected = "DEFINE TABLE bool_table; DEFINE FIELD simple_bool ON bool_table TYPE bool; DEFINE FIELD specified_bool ON bool_table TYPE bool; DEFINE FIELD value_bool ON bool_table TYPE bool VALUE true ASSERT $value == true;";
        assert_eq!(BoolStruct::schema_query(), expected);
    }

    #[test]
    fn test_readonly_struct() {
        #[derive(SurQLDefinition)]
        #[surql_table("readonly_table")]
        struct ReadonlyStruct {
            // Readonly field
            #[surql_field(TYPE = "int", VALUE = 2, READONLY)]
            readonly_field: i32,
        }
        let expected = "DEFINE TABLE readonly_table; DEFINE FIELD readonly_field ON readonly_table TYPE int VALUE 2 READONLY;";
        assert_eq!(ReadonlyStruct::schema_query(), expected);
    }

    #[cfg(feature = "runtime_query_validation")]
    #[test]
    #[should_panic(expected = "Failed to parse query")]
    fn test_invalid_struct_query() {
        #[derive(SurQLDefinition)]
        #[surql_query("INVALID SYNTAX;")]
        struct InvalidStruct {
            #[surql_field(TYPE = "int")]
            dummy_field: i32,
        }

        InvalidStruct::schema_query();
    }

    #[cfg(feature = "runtime_query_validation")]
    #[test]
    #[should_panic(expected = "Failed to parse query")]
    fn test_invalid_field_assertion() {
        #[derive(SurQLDefinition)]
        #[surql_table("invalid_assert_table")]
        struct InvalidAssertionStruct {
            #[surql_field(TYPE = "int", ASSERT = "INVALID ASSERTION")]
            invalid_field: i32,
        }

        InvalidAssertionStruct::schema_query();
    }
}
