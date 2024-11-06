use html_parser::core::{Grammar, Rule};
use pest::Parser;

fn main() -> anyhow::Result<()> {
    let grammar = Grammar::parse(Rule::tag, "</div>")?;
    println!("{}", grammar);

    Ok(())
}
