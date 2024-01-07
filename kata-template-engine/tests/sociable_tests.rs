use std::collections::HashMap;

use kata_template_engine::{parse_template, parse_template_from_json};

#[test]
fn given_a_text_apply_template_from_json() {
    let variables = r#"{"name": "John", "greet": "Hello"}"#;
    let text = "${greet}, ${name}";
    let parsed_template = parse_template_from_json(text, variables).unwrap();
    assert_eq!(parsed_template.text, "Hello, John");
}

#[test]
fn given_a_text_apply_template_from_json_when_not_all_variables_exist() {
    let variables = r#"{"greet": "Hello"}"#;
    let text = "${greet}, ${name}";
    let parsed_template = parse_template_from_json(text, variables).unwrap();
    assert_eq!(parsed_template.text, "Hello, ${name}");
}

#[test]
fn given_an_empty_json_it_raises_an_error() {
    let variables = r#"{}"#;
    let parsed_template = parse_template_from_json("a", variables);
    assert_eq!(
        parsed_template.unwrap_err().to_string(),
        "Empty dictionary is not allowed"
    );
}

#[test]
fn given_an_empty_dictionary_it_raises_an_error() {
    let parsed_template = parse_template("a", HashMap::new());
    assert_eq!(
        parsed_template.unwrap_err().to_string(),
        "Empty dictionary is not allowed"
    );
}

#[test]
fn given_an_empty_file_it_raises_an_error() {
    let variables = r#"{"name": "John", "greet": "Hello"}"#;
    let parsed_template = parse_template_from_json("", variables);
    assert_eq!(
        parsed_template.unwrap_err().to_string(),
        "Empty file is not allowed"
    );
}

#[test]
fn given_a_file_where_there_are_no_variables_that_matches_it_returns_the_same_text_and_warnings() {
    let variables = r#"{"name": "John", "greet": "Hello"}"#;
    let parsed_template = parse_template_from_json("Hello, ${surname}", variables).unwrap();
    assert_eq!(parsed_template.text, "Hello, ${surname}");
    assert_eq!(parsed_template.warnings.len(), 2);
    assert_eq!(parsed_template.warnings[0], "Variable name not found");
    assert_eq!(parsed_template.warnings[1], "Variable surname not replaced");
}
