use std::collections::HashMap;
use std::fmt;

fn apply_template(text: &str, dictionary: &str) -> Result<String, TemplateError> {
    if text.is_empty() {
        return Err(TemplateError::EmptyFile);
    }
    let dictionary = build_dictionary(dictionary)?;
    let mut result = text.to_string();
    for (key, value) in dictionary {
        result = replace(&result, (&key, &value));
    }
    Ok(result)
}

fn replace(text: &str, word_and_value: (&str, &str)) -> String {
    text.replace(&format!("${{{}}}", word_and_value.0), word_and_value.1)
}

type Dictionary = HashMap<String, String>;

fn build_dictionary(json: &str) -> Result<Dictionary, TemplateError> {
    let dicttionary = serde_json::from_str::<Dictionary>(json).map_err(TemplateError::JsonError)?;
    if dicttionary.is_empty() {
        return Err(TemplateError::EmptyDictionary);
    }
    Ok(dicttionary)
}

#[derive(Debug)]
enum TemplateError {
    JsonError(serde_json::Error),
    EmptyDictionary,
    EmptyFile,
}

impl fmt::Display for TemplateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TemplateError::JsonError(ref err) => write!(f, "JSON error: {}", err),
            TemplateError::EmptyDictionary => write!(f, "Empty dictionary is not allowed"),
            TemplateError::EmptyFile => write!(f, "Empty file is not allowed"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let result = apply_template(text, dictionary).unwrap();
        assert_eq!(result, "Hello, John");
    }

    #[test]
    fn given_a_text_apply_template_when_not_all_variables_exist() {
        let dictionary = r#"{"greet": "Hello"}"#;
        let text = "${greet}, ${name}";
        let result = apply_template(text, dictionary).unwrap();
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
        assert_eq!(
            result.unwrap_err().to_string(),
            "JSON error: expected `:` at line 1 column 9"
        );
    }

    #[test]
    fn given_an_empty_json_it_raises_an_error() {
        let json = r#"{}"#;
        let result = apply_template("a", json);
        assert_eq!(
            result.unwrap_err().to_string(),
            "Empty dictionary is not allowed"
        );
    }

    #[test]
    fn given_an_empty_file_it_raises_an_error() {
        let json = r#"{"name": "John", "greet": "Hello"}"#;
        let result = apply_template("", json);
        assert_eq!(result.unwrap_err().to_string(), "Empty file is not allowed");
    }
}
