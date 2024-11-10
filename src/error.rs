use std::io;
use thiserror::Error;

use crate::grammar::Rule;

#[derive(Error, Debug)]
pub enum HtmlParserError {
    #[error("Parse error: {0}")]
    ParseError(#[from] Box<pest::error::Error<Rule>>),

    #[error("File I/O error: {0}")]
    IoError(#[from] io::Error),
}
