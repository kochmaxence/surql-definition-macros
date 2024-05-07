pub trait SurQLSchemaProducer {
    fn schema_query() -> &'static str;
}

pub fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    let mut last_char_was_upper = false;
    let mut last_char_was_letter = false;

    for c in s.chars() {
        if c.is_ascii_uppercase() {
            if last_char_was_letter && !last_char_was_upper {
                result.push('_');
            }
            result.push(c.to_ascii_lowercase());
            last_char_was_upper = true;
        } else if c.is_ascii_lowercase() || c.is_ascii_digit() {
            result.push(c);
            last_char_was_upper = false;
        } else {
            result.push('_');
            last_char_was_upper = false;
        }
        last_char_was_letter = c.is_ascii_alphabetic();
    }

    result
}
