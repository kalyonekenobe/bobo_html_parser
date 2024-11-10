pub mod cli;
pub mod error;
pub mod grammar;

pub mod core {
    use std::io::Read;
    use std::{fs::OpenOptions, path::PathBuf};

    use crate::error::HtmlParserError;
    use crate::grammar::{Grammar, Rule};
    use pest::{iterators::Pairs, Parser};
    use pest_ascii_tree::into_ascii_tree;

    pub fn parse_input_by_rule(rule: Rule, input: &str) -> Result<Pairs<Rule>, HtmlParserError> {
        Grammar::parse(rule, input).map_err(|e| HtmlParserError::ParseError(Box::new(e)))
    }

    pub fn parse_html(input: &str) -> Result<Pairs<Rule>, HtmlParserError> {
        parse_input_by_rule(Rule::html, input)
    }

    pub fn read_html_from_file(file_path: PathBuf) -> Result<String, HtmlParserError> {
        let mut file = OpenOptions::new().read(true).write(true).open(file_path)?;

        let mut file_content = String::new();
        file.read_to_string(&mut file_content)?;

        Ok(file_content.to_string())
    }

    pub fn print_parsed_tree(pairs: Pairs<Rule>) {
        eprintln!("{}", into_ascii_tree(pairs).unwrap());
    }
}
