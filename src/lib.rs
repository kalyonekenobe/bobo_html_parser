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

    pub fn print_help() {
        println!(
            "BOBO HTML PARSER CLI\n\n\
            USAGE:\n\
            \tcargo run -- --file <path-to-the-file> parse\n\
            \tcargo run -- help\n\n\
            OPTIONS:\n\
            \t-f, --file <FILE>     Specifies the path to the HTML file to parse\n\n\
            SUBCOMMANDS:\n\
            \tparse                 Parses the specified HTML file and outputs relevant data\n\
            \thelp                  Prints help information\n\n\
            DESCRIPTION:\n\
            \tThis HTML parser CLI reads an HTML file from the specified path and processes\n\
            \tthe content. It can analyze, validate, or extract data based on the file's structure.\n\n\
            EXAMPLES:\n\
            \tParse an HTML file located at './default.html':\n\
            \t    cargo run -- --file ./default.html parse\n\n\
            NOTE:\n\
            \tEnsure that the file path is valid and points to an HTML file."
        );
    }
}
