use std::collections::HashMap;
use std::fmt;

pub fn parse_template_from_json(
    template_text: &str,
    json_variables: &str,
) -> Result<String, TemplateError> {
    if template_text.is_empty() {
        return Err(TemplateError::EmptyFile);
    }
    let dictionary = build_dictionary(json_variables)?;
    parse_template(template_text, dictionary)
}

pub fn parse_template(
    template_text: &str,
    variables: HashMap<String, String>,
) -> Result<String, TemplateError> {
    if template_text.is_empty() {
        return Err(TemplateError::EmptyFile);
    }
    let mut parsed_template = template_text.to_string();
    for (variable, value) in variables {
        parsed_template = replace(&parsed_template, (&variable, &value));
    }
    Ok(parsed_template)
}

fn replace(template_text: &str, word_and_value: (&str, &str)) -> String {
    template_text.replace(&format!("${{{}}}", word_and_value.0), word_and_value.1)
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
    fn replace_one_word() {
        let words_replaced = replace("${greet}", ("greet", "Hello"));
        assert_eq!(words_replaced, "Hello");
    }

    #[test]
    fn replace_with_wrong_format_does_not_replace() {
        let words_replaced = replace("${greet", ("greet", "Hello"));
        assert_eq!(words_replaced, "${greet");
    }

    #[test]
    fn replace_same_word_two_times() {
        let words_replaced = replace("${greet}${greet}", ("greet", "Hello"));
        assert_eq!(words_replaced, "HelloHello");
    }

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
