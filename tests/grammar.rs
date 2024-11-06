#[cfg(test)]
pub mod tests {
    use html_parser::core::{Grammar, Rule};
    use pest::Parser;

    #[test]
    pub fn is_tag() {
        let result = Grammar::parse(Rule::tag, "<div>");

        assert!(result.is_ok());
        let tag = result.unwrap();
        assert_eq!(tag.as_str(), "<div>");
    }

    #[test]
    pub fn is_opening_tag() {
        let result = Grammar::parse(Rule::opening_tag, "<html>");

        assert!(result.is_ok());
        let tag = result.unwrap();
        assert_eq!(tag.as_str(), "<html>");
    }

    #[test]
    pub fn is_closing_tag() {
        let result = Grammar::parse(Rule::closing_tag, "</div>");

        assert!(result.is_ok());
        let tag = result.unwrap();
        assert_eq!(tag.as_str(), "</div>");
    }

    #[test]
    pub fn is_self_closing_tag() {
        let result = Grammar::parse(Rule::self_closing_tag, "<input />");

        assert!(result.is_ok());
        let tag = result.unwrap();
        assert_eq!(tag.as_str(), "<input />");
    }

    #[test]
    pub fn is_not_closing_tag() {
        let result = Grammar::parse(Rule::closing_tag, "<div>");

        assert!(result.is_err());
    }
}
