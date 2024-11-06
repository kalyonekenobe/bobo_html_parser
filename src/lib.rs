pub mod core {
    use pest_derive::Parser;

    #[derive(Parser)]
    #[grammar = "./grammar/grammar.pest"]
    pub struct Grammar;
}
