use std::collections::HashMap;
use std::error::Error;

fn apply_template(text: &str, dictionary: &str) -> String {
    let dictionary = build_dictionary(dictionary).unwrap();
    let mut result = text.to_string();
    for (key, value) in dictionary {
        result = replace(&result, (&key, &value));
    }
    result
}

fn replace(text: &str, word_and_value: (&str, &str)) -> String {
    text.replace(&format!("${{{}}}", word_and_value.0), word_and_value.1)
}

type Dictionary = HashMap<String, String>;

fn build_dictionary(json: &str) -> Result<Dictionary, TemplateError> {
    serde_json::from_str::<Dictionary>(json).map_err(TemplateError::JsonError)
}

#[derive(Debug)]
enum TemplateError {
    JsonError(serde_json::Error),
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Error as SerdeJsonError;

    #[test]
    fn replace_one_word() {
        let result = replace("${greet}", ("greet", "Hello"));
        assert_eq!(result, "Hello");
    }

    #[test]
    fn replace_with_wrong_format_does_not_replace() {
        let result = replace("${greet", ("greet", "Hello"));
        assert_eq!(result, "${greet");
    }

    #[test]
    fn replace_same_word_two_times() {
        let result = replace("${greet}${greet}", ("greet", "Hello"));
        assert_eq!(result, "HelloHello");
    }

    #[test]
    fn given_a_text_apply_template() {
        let dictionary = r#"{"name": "John", "greet": "Hello"}"#;
        let text = "${greet}, ${name}";
        let result = apply_template(text, dictionary);
        assert_eq!(result, "Hello, John");
    }

    #[test]
    fn given_a_text_apply_template_when_not_all_variables_exist() {
        let dictionary = r#"{"greet": "Hello"}"#;
        let text = "${greet}, ${name}";
        let result = apply_template(text, dictionary);
        assert_eq!(result, "Hello, ${name}");
    }

    #[test]
    fn given_a_valid_json_string_i_can_create_a_dictionary() {
        let json = r#"{"name": "John", "greet": "Hello"}"#;
        let dictionary = build_dictionary(json).unwrap();
        assert_eq!(dictionary.get("name"), Some(&String::from("John")));
        assert_eq!(dictionary.get("greet"), Some(&String::from("Hello")));
    }

    #[test]
    fn given_an_invalid_json_string_it_raises_an_error() {
        let json = r#"{"name" "John", "greet": "Hello"}"#;
        let result = build_dictionary(json);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "");
    }
}
