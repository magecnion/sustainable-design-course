use std::collections::HashMap;
use std::fmt;

pub fn apply_template(text: &str, variables: &str) -> Result<String, TemplateError> {
    if text.is_empty() {
        return Err(TemplateError::EmptyFile);
    }
    let dictionary = build_dictionary(variables)?;
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
pub enum TemplateError {
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
    fn given_a_valid_json_string_i_can_create_a_dictionary() {
        let variables = r#"{"name": "John", "greet": "Hello"}"#;
        let dictionary = build_dictionary(variables).unwrap();
        assert_eq!(dictionary.get("name"), Some(&String::from("John")));
        assert_eq!(dictionary.get("greet"), Some(&String::from("Hello")));
    }

    #[test]
    fn given_an_invalid_json_string_it_raises_an_error() {
        let variables = r#"{"name" "John", "greet": "Hello"}"#;
        let dictionary = build_dictionary(variables);
        assert_eq!(
            dictionary.unwrap_err().to_string(),
            "JSON error: expected `:` at line 1 column 9"
        );
    }
}
