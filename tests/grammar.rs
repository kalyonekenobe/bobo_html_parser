#[cfg(test)]
pub mod tests {
    use html_parser::{
        core::{parse_html, parse_input_by_rule},
        grammar::Rule,
    };

    #[test]
    pub fn is_valid_html() -> anyhow::Result<()> {
        let html = "
        <!DOCTYPE html PUBLIC \"-//W3C//DTD XHTML 1.1//EN\" \"http://www.w3.org/TR/xhtml11/DTD/xhtml11.dtd\">
        <html>
        
            <body>
            
                <h1 class=\"heading\">My First Heading</h1>

                <p>My first paragraph.</p>
            
            </body>

        </html>

        ";
        let result = parse_html(&html);

        assert!(result.is_ok());

        Ok(())
    }

    #[test]
    pub fn is_invalid_html() -> anyhow::Result<()> {
        let html = "
        <!DOCTYPE html>
        <html>
            </body>
            <body>
            
                <h1 class=\"heading\">My First Heading</h1>

                <p>My first paragraph.</p>

            </body>

        </html>

        ";
        let result = parse_html(&html);

        assert!(result.is_err());

        Ok(())
    }

    #[test]
    pub fn is_valid_tag_name() -> anyhow::Result<()> {
        let result = parse_input_by_rule(Rule::tag_name, "img");

        assert!(result.is_ok());
        let tag = result.unwrap();

        assert_eq!("img", tag.as_str());

        Ok(())
    }

    #[test]
    pub fn is_invalid_tag_name() -> anyhow::Result<()> {
        let result = parse_input_by_rule(Rule::tag_name, "158TAG");

        assert!(result.is_err());

        Ok(())
    }

    #[test]
    pub fn is_opening_tag() -> anyhow::Result<()> {
        let result = parse_input_by_rule(Rule::opening_tag, "<html>");

        assert!(result.is_ok());
        let tag = result.unwrap();
        assert_eq!(tag.as_str(), "<html>");

        Ok(())
    }

    #[test]
    pub fn is_valid_closing_tag() -> anyhow::Result<()> {
        let result: Result<pest::iterators::Pairs<'_, Rule>, html_parser::error::HtmlParserError> =
            parse_input_by_rule(Rule::common_element, "<div></div>");

        assert!(result.is_ok());
        let tag = result.unwrap();
        assert_eq!(tag.as_str(), "<div></div>");

        Ok(())
    }

    #[test]
    pub fn is_invalid_closing_tag() -> anyhow::Result<()> {
        let result: Result<pest::iterators::Pairs<'_, Rule>, html_parser::error::HtmlParserError> =
            parse_input_by_rule(Rule::common_element, "<div></di>");

        assert!(result.is_err());

        Ok(())
    }

    #[test]
    pub fn is_self_closing_tag() -> anyhow::Result<()> {
        let result = parse_input_by_rule(Rule::self_closing_tag, "<input />");

        assert!(result.is_ok());
        let tag = result.unwrap();
        assert_eq!(tag.as_str(), "<input />");

        Ok(())
    }

    #[test]
    pub fn is_not_closing_tag() -> anyhow::Result<()> {
        let result = parse_input_by_rule(Rule::closing_tag, "<div>");

        assert!(result.is_err());

        Ok(())
    }

    #[test]
    pub fn is_hyphen() -> anyhow::Result<()> {
        let result = parse_input_by_rule(Rule::hyphen, "-");

        assert!(result.is_ok());

        Ok(())
    }

    #[test]
    pub fn is_not_exclamation_mark() -> anyhow::Result<()> {
        let result = parse_input_by_rule(Rule::exclamation_mark, "?");

        assert!(result.is_err());

        Ok(())
    }

    #[test]
    pub fn is_question_mark() -> anyhow::Result<()> {
        let result = parse_input_by_rule(Rule::question_mark, "?");

        assert!(result.is_ok());

        Ok(())
    }

    #[test]
    pub fn is_underscore() -> anyhow::Result<()> {
        let result = parse_input_by_rule(Rule::underscore, "_");

        assert!(result.is_ok());

        Ok(())
    }

    #[test]
    pub fn is_valid_attribute_name() -> anyhow::Result<()> {
        let result = parse_input_by_rule(Rule::attribute_name, "class");

        assert!(result.is_ok());
        let tag = result.unwrap();

        assert_eq!("class", tag.as_str());

        Ok(())
    }

    #[test]
    pub fn is_invalid_attribute_name() -> anyhow::Result<()> {
        let result = parse_input_by_rule(Rule::attribute_name, "041class");

        assert!(result.is_err());

        Ok(())
    }

    #[test]
    pub fn is_not_equation_mark() -> anyhow::Result<()> {
        let result = parse_input_by_rule(Rule::equation_mark, "!=");

        assert!(result.is_err());

        Ok(())
    }

    #[test]
    pub fn is_single_quote() -> anyhow::Result<()> {
        let result = parse_input_by_rule(Rule::single_quote, "'");

        assert!(result.is_ok());

        Ok(())
    }

    #[test]
    pub fn is_valid_common_element() -> anyhow::Result<()> {
        let result = parse_input_by_rule(Rule::common_element, "<div><span>text</span></div>");

        assert!(result.is_ok());
        let tag = result.unwrap();
        assert_eq!("<div><span>text</span></div>", tag.as_str());

        Ok(())
    }

    #[test]
    pub fn is_invalid_common_element() -> anyhow::Result<()> {
        let result = parse_input_by_rule(Rule::common_element, "<3div><span>text</div></span>");

        assert!(result.is_err());

        Ok(())
    }

    #[test]
    pub fn is_invalid_self_closing_element() -> anyhow::Result<()> {
        let result = parse_input_by_rule(Rule::self_closing_element, "</ tete>");

        assert!(result.is_err());

        Ok(())
    }

    #[test]
    pub fn is_double_quote() -> anyhow::Result<()> {
        let result = parse_input_by_rule(Rule::double_quote, "\"");

        assert!(result.is_ok());

        Ok(())
    }

    #[test]
    pub fn is_left_chevron() -> anyhow::Result<()> {
        let result = parse_input_by_rule(Rule::left_chevron, "<");

        assert!(result.is_ok());

        Ok(())
    }

    #[test]
    pub fn is_right_chevron() -> anyhow::Result<()> {
        let result = parse_input_by_rule(Rule::right_chevron, ">");

        assert!(result.is_ok());

        Ok(())
    }

    #[test]
    pub fn is_not_slash() -> anyhow::Result<()> {
        let result = parse_input_by_rule(Rule::slash, "|");

        assert!(result.is_err());

        Ok(())
    }

    #[test]
    pub fn is_valid_html_text() -> anyhow::Result<()> {
        let result = parse_input_by_rule(Rule::html_text, "Some text");

        assert!(result.is_ok());
        let tag = result.unwrap();
        assert_eq!("Some text", tag.as_str());

        Ok(())
    }

    #[test]
    pub fn is_invalid_html_text() -> anyhow::Result<()> {
        let result = parse_input_by_rule(Rule::html_text, "<Some text");

        assert!(result.is_err());

        Ok(())
    }

    #[test]
    pub fn is_invalid_html_element() -> anyhow::Result<()> {
        let result = parse_input_by_rule(Rule::html_element, "<<Some text");

        assert!(result.is_err());

        Ok(())
    }

    #[test]
    pub fn is_invalid_html_node() -> anyhow::Result<()> {
        let result = parse_input_by_rule(Rule::html_node, "<div>SOme content</div>");

        assert!(result.is_ok());
        let tag = result.unwrap();
        assert_eq!("<div>SOme content</div>", tag.as_str());

        Ok(())
    }

    #[test]
    pub fn is_whitespace() -> anyhow::Result<()> {
        let result = parse_input_by_rule(Rule::whitespace, "\n \t\t\r");

        assert!(result.is_ok());

        Ok(())
    }

    #[test]
    pub fn is_digit() -> anyhow::Result<()> {
        let result = parse_input_by_rule(Rule::digit, "8");

        assert!(result.is_ok());

        Ok(())
    }

    #[test]
    pub fn is_not_alpha() -> anyhow::Result<()> {
        let result = parse_input_by_rule(Rule::alpha, "#");

        assert!(result.is_err());

        Ok(())
    }

    #[test]
    pub fn is_left_square_bracket() -> anyhow::Result<()> {
        let result = parse_input_by_rule(Rule::left_square_bracket, "[");

        assert!(result.is_ok());

        Ok(())
    }

    #[test]
    pub fn is_right_square_bracket() -> anyhow::Result<()> {
        let result = parse_input_by_rule(Rule::right_square_bracket, "]");

        assert!(result.is_ok());

        Ok(())
    }

    #[test]
    pub fn is_doctype_keyword() -> anyhow::Result<()> {
        let result = parse_input_by_rule(Rule::doctype_keyword, "DOCTYPE");

        assert!(result.is_ok());

        Ok(())
    }

    #[test]
    pub fn is_not_doctype_keyword() -> anyhow::Result<()> {
        let result = parse_input_by_rule(Rule::doctype_keyword, "D0CtYPE");

        assert!(result.is_err());

        Ok(())
    }

    #[test]
    pub fn is_doctype_top_level_element() -> anyhow::Result<()> {
        let result = parse_input_by_rule(Rule::doctype_top_level_element, "html");

        assert!(result.is_ok());

        Ok(())
    }

    #[test]
    pub fn is_not_doctype_top_level_element() -> anyhow::Result<()> {
        let result = parse_input_by_rule(Rule::doctype_keyword, "js");

        assert!(result.is_err());

        Ok(())
    }

    #[test]
    pub fn is_doctype_privacy_level() -> anyhow::Result<()> {
        let result = parse_input_by_rule(Rule::doctype_privacy_level, "system");

        assert!(result.is_ok());

        Ok(())
    }

    #[test]
    pub fn is_not_doctype_privacy_level() -> anyhow::Result<()> {
        let result = parse_input_by_rule(Rule::doctype_privacy_level, "PRIVATE");

        assert!(result.is_err());

        Ok(())
    }

    #[test]
    pub fn is_doctype_params_allowed_symbol() -> anyhow::Result<()> {
        let result = parse_input_by_rule(Rule::doctype_params_allowed_symbol, "-");

        assert!(result.is_ok());

        Ok(())
    }

    #[test]
    pub fn is_doctype_params_disallowed_symbol() -> anyhow::Result<()> {
        let result = parse_input_by_rule(Rule::doctype_params_allowed_symbol, "`");

        assert!(result.is_err());

        Ok(())
    }

    #[test]
    pub fn is_doctype_url_allowed_symbol() -> anyhow::Result<()> {
        let result = parse_input_by_rule(Rule::doctype_url_allowed_symbol, "&");

        assert!(result.is_ok());

        Ok(())
    }

    #[test]
    pub fn is_doctype_url_disallowed_symbol() -> anyhow::Result<()> {
        let result = parse_input_by_rule(Rule::doctype_url_allowed_symbol, "+");

        assert!(result.is_err());

        Ok(())
    }

    #[test]
    pub fn is_dot() -> anyhow::Result<()> {
        let result = parse_input_by_rule(Rule::dot, ".");

        assert!(result.is_ok());

        Ok(())
    }

    #[test]
    pub fn is_comma() -> anyhow::Result<()> {
        let result = parse_input_by_rule(Rule::comma, ",");

        assert!(result.is_ok());

        Ok(())
    }

    #[test]
    pub fn is_valid_doctype_params() -> anyhow::Result<()> {
        let result = parse_input_by_rule(
            Rule::doctype_params,
            "\"-//W3C//DTD HTML 4.01 Frameset//EN\"",
        );

        assert!(result.is_ok());

        Ok(())
    }

    #[test]
    pub fn is_invalid_doctype_params() -> anyhow::Result<()> {
        let result =
            parse_input_by_rule(Rule::doctype_params, "\"-W3C//DTD HTML 4.01 Frameset//EN\"");

        assert!(result.is_err());

        Ok(())
    }

    #[test]
    pub fn is_valid_doctype_url() -> anyhow::Result<()> {
        let result = parse_input_by_rule(
            Rule::doctype_url,
            "\"http://www.w3.org/TR/xhtml1/DTD/xhtml1-frameset.dtd\"",
        );

        assert!(result.is_ok());

        Ok(())
    }

    #[test]
    pub fn is_invalid_doctype_url() -> anyhow::Result<()> {
        let result = parse_input_by_rule(
            Rule::doctype_url,
            "\"http://www.w3.org/TR/xht```ml1/DTD/xhtml1-frameset.dtd\"",
        );

        assert!(result.is_err());

        Ok(())
    }

    #[test]
    pub fn is_valid_doctype_declaration() -> anyhow::Result<()> {
        let result =
            parse_input_by_rule(Rule::doctype_declaration, "<!DOCTYPE html PUBLIC \"-//W3C//DTD XHTML 1.1//EN\" \"http://www.w3.org/TR/xhtml11/DTD/xhtml11.dtd\">");

        assert!(result.is_ok());

        Ok(())
    }

    #[test]
    pub fn is_invalid_doctype_declaration() -> anyhow::Result<()> {
        let result =
            parse_input_by_rule(Rule::doctype_declaration, "<!DOCTYPE html Private -//W3C//DTD XHTML 1.1//EN\" \"http://www.w3.org/TR/xhtml11/DTD/xhtml11.dtd\">");

        assert!(result.is_err());

        Ok(())
    }

    #[test]
    pub fn is_not_colon() -> anyhow::Result<()> {
        let result = parse_input_by_rule(Rule::colon, ";");

        assert!(result.is_err());

        Ok(())
    }

    #[test]
    pub fn is_not_semicolon() -> anyhow::Result<()> {
        let result = parse_input_by_rule(Rule::semicolon, ":");

        assert!(result.is_err());

        Ok(())
    }

    #[test]
    pub fn is_valid_attribute_value_opening_quote() -> anyhow::Result<()> {
        let result = parse_input_by_rule(Rule::attribute, "class=\"test\"");

        assert!(result.is_ok());

        Ok(())
    }

    #[test]
    pub fn is_invalid_attribute_value_opening_quote() -> anyhow::Result<()> {
        let result = parse_input_by_rule(Rule::attribute, "class=\'test\"");

        assert!(result.is_err());

        Ok(())
    }

    #[test]
    pub fn is_valid_attribute_value() -> anyhow::Result<()> {
        let result = parse_input_by_rule(Rule::attribute, "class=\"some-value\"");

        assert!(result.is_ok());
        let tag = result.unwrap();
        assert_eq!("class=\"some-value\"", tag.as_str());

        Ok(())
    }

    #[test]
    pub fn is_invalid_attribute_value() -> anyhow::Result<()> {
        let result = parse_input_by_rule(Rule::opening_tag, "<div class=\'some-\'value\'>");

        assert!(result.is_err());

        Ok(())
    }

    #[test]
    pub fn is_valid_attribute() -> anyhow::Result<()> {
        let result = parse_input_by_rule(Rule::attribute, "id='123'");

        assert!(result.is_ok());
        let tag = result.unwrap();
        assert_eq!("id='123'", tag.as_str());

        Ok(())
    }

    #[test]
    pub fn is_space() -> anyhow::Result<()> {
        let result = parse_input_by_rule(Rule::space, " ");

        assert!(result.is_ok());

        Ok(())
    }

    #[test]
    pub fn is_dollar() -> anyhow::Result<()> {
        let result = parse_input_by_rule(Rule::dollar, "$");

        assert!(result.is_ok());

        Ok(())
    }

    #[test]
    pub fn is_ampersand() -> anyhow::Result<()> {
        let result = parse_input_by_rule(Rule::ampersand, "&");

        assert!(result.is_ok());

        Ok(())
    }

    #[test]
    pub fn is_html_comment_opening_tag() -> anyhow::Result<()> {
        let result = parse_input_by_rule(Rule::html_comment_opening_tag, "<!--");

        assert!(result.is_ok());

        Ok(())
    }

    #[test]
    pub fn is_invalid_html_comment_opening_tag() -> anyhow::Result<()> {
        let result = parse_input_by_rule(Rule::html_comment_opening_tag, "<--");

        assert!(result.is_err());

        Ok(())
    }

    #[test]
    pub fn is_html_comment_closing_tag() -> anyhow::Result<()> {
        let result = parse_input_by_rule(Rule::html_comment_closing_tag, "-->");

        assert!(result.is_ok());

        Ok(())
    }

    #[test]
    pub fn is_invalid_html_comment_closing_tag() -> anyhow::Result<()> {
        let result = parse_input_by_rule(Rule::html_comment_closing_tag, ">");

        assert!(result.is_err());

        Ok(())
    }

    #[test]
    pub fn is_html_comment_content() -> anyhow::Result<()> {
        let result = parse_input_by_rule(Rule::html_comment_content, "ahgh fajsbf 1u41y2afs _");

        assert!(result.is_ok());

        Ok(())
    }

    #[test]
    pub fn is_invalid_html_comment() -> anyhow::Result<()> {
        let result = parse_input_by_rule(Rule::html_comment, "<!-- <!-- ahgh fajsbf 1u41y2afs _");

        assert!(result.is_err());

        Ok(())
    }

    #[test]
    pub fn is_valid_html_comment() -> anyhow::Result<()> {
        let result = parse_input_by_rule(Rule::html_comment, "<!-- ahgh fajsbf 1u41y2afs _ -->");

        assert!(result.is_ok());

        Ok(())
    }
}
