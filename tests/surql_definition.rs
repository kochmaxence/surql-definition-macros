#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use surql_definition_macros::SurQLDefinition;

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

    #[derive(SurQLDefinition)]
    #[surql_table("float_table")]
    struct FloatStruct {
        // Type inferred from Rust type
        floating_num: f64,

        // Explicitly specified type
        #[surql_field(TYPE = "float")]
        specified_float: f64,
    }

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

    #[derive(SurQLDefinition)]
    #[surql_table("readonly_table")]
    struct ReadonlyStruct {
        // Readonly field
        #[surql_field(TYPE = "int", READONLY)]
        readonly_field: i32,
    }

    #[derive(SurQLDefinition)]
    #[surql_query("LIVE SELECT DIFF FROM person;")]
    struct LiveDiffStruct {
        #[surql_field(TYPE = "string")]
        name: String,
    }

    #[derive(SurQLDefinition)]
    #[surql_query("PATCH person WITH {\"op\": \"replace\", \"path\": \"/last_updated\", \"value\": \"2023-06-16T08:34:25Z\"};")]
    struct PatchDiffStruct {
        #[surql_field(TYPE = "timestamp")]
        last_updated: String,
    }

    #[test]
    fn test_simple_struct() {
        let expected = "DEFINE TABLE test_table; DEFINE FIELD simple_int ON test_table TYPE int; DEFINE FIELD specified_int ON test_table TYPE int; DEFINE FIELD default_int ON test_table TYPE int DEFAULT 42 ASSERT $value > 0;";
        assert_eq!(SimpleStruct::schema_query(), expected);
    }

    #[test]
    fn test_float_struct() {
        let expected = "DEFINE TABLE float_table; DEFINE FIELD floating_num ON float_table TYPE float; DEFINE FIELD specified_float ON float_table TYPE float;";
        assert_eq!(FloatStruct::schema_query(), expected);
    }

    #[test]
    fn test_string_struct() {
        let expected = "DEFINE TABLE string_table; DEFINE FIELD simple_string ON string_table TYPE string; DEFINE FIELD specified_string ON string_table TYPE string; DEFINE FIELD flexible_string ON string_table FLEXIBLE TYPE string;";
        assert_eq!(StringStruct::schema_query(), expected);
    }

    #[test]
    fn test_bool_struct() {
        let expected = "DEFINE TABLE bool_table; DEFINE FIELD simple_bool ON bool_table TYPE bool; DEFINE FIELD specified_bool ON bool_table TYPE bool; DEFINE FIELD value_bool ON bool_table TYPE bool VALUE true ASSERT $value == true;";
        assert_eq!(BoolStruct::schema_query(), expected);
    }

    #[test]
    fn test_readonly_struct() {
        let expected = "DEFINE TABLE readonly_table; DEFINE FIELD readonly_field ON readonly_table TYPE int READONLY;";
        assert_eq!(ReadonlyStruct::schema_query(), expected);
    }

    #[test]
    fn test_live_diff_struct() {
        let expected = "LIVE SELECT DIFF FROM person;";
        assert_eq!(LiveDiffStruct::schema_query(), expected);
    }

    #[test]
    fn test_patch_diff_struct() {
        let expected = "PATCH person WITH {\"op\": \"replace\", \"path\": \"/last_updated\", \"value\": \"2023-06-16T08:34:25Z\"};";
        assert_eq!(PatchDiffStruct::schema_query(), expected);
    }
}
