use regex::Regex;
use std::collections::HashMap;
use std::fmt;

pub fn parse_template_from_json(
    template_text: &str,
    json_variables: &str,
) -> Result<ParsedTemplate, TemplateError> {
    check_text_not_empty(template_text)?;
    let dictionary = build_dictionary(json_variables)?;
    parse_template(template_text, dictionary)
}

fn check_dictionary_not_empty(dictionary: &HashMap<String, String>) -> Result<(), TemplateError> {
    if dictionary.is_empty() {
        return Err(TemplateError::EmptyDictionary);
    }
    Ok(())
}

fn check_text_not_empty(text: &str) -> Result<(), TemplateError> {
    if text.is_empty() {
        return Err(TemplateError::EmptyFile);
    }
    Ok(())
}

#[derive(Debug)]
pub struct ParsedTemplate {
    pub text: String,
    pub warnings: Vec<String>,
}

pub fn parse_template(
    template_text: &str,
    variables: HashMap<String, String>,
) -> Result<ParsedTemplate, TemplateError> {
    check_text_not_empty(template_text)?;
    check_dictionary_not_empty(&variables)?;
    let mut warnings = Vec::new();
    let mut parsed_template_text = template_text.to_string();
    for (variable, value) in variables {
        parsed_template_text = parsed_template_text.replace(&format!("${{{}}}", variable), &value);
        if !parsed_template_text.contains(&value) {
            warnings.push(format!("Variable {} not found", variable));
        }
    }
    let not_replaced_variables_warnings =
        get_warnings_for_not_replaced_variables(parsed_template_text.clone());
    warnings.extend(not_replaced_variables_warnings);
    Ok(ParsedTemplate {
        text: parsed_template_text,
        warnings,
    })
}

fn get_warnings_for_not_replaced_variables(parsed_template_text: String) -> Vec<String> {
    let mut warnings = Vec::new();
    let re = Regex::new(r"\$\{(\w+)\}").unwrap();
    for captures in re.captures_iter(&parsed_template_text) {
        let word = &captures[1];
        warnings.push(format!("Variable {} not replaced", word));
    }
    warnings
}

type Dictionary = HashMap<String, String>;

fn build_dictionary(json: &str) -> Result<Dictionary, TemplateError> {
    let dicttionary = serde_json::from_str::<Dictionary>(json).map_err(TemplateError::JsonError)?;
    check_dictionary_not_empty(&dicttionary)?;
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
