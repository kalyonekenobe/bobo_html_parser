# bobo_html_parser

This project is a simple HTML parser built using Rust and the [Pest](https://crates.io/crates/pest)
parsing library. It parses HTML documents, tags, attributes, nested structures, and text content.
This parser is a foundation for more complex parsing logic, which can be extended to handle
self-closing tags, custom error handling, and more.

## Table of Contents

- [Features](#features)
- [Technical Description](#technical-description)
- [Grammar](#grammar)
- [Installation](#installation)
- [Usage](#usage)
- [Project Structure](#project-structure)
- [Example Output](#example-output)

## Features

- Parses HTML elements and nested structures.
- Captures tag names, attributes, and text content.
- Prints a structured view of the HTML tree.
- Handles nested HTML elements.

## Technical Description

The HTML parser is built using the `pest` crate and is designed to process basic HTML elements such as tags and their content. The parser adheres to a simplified version of HTML syntax, focusing on parsing the structure of tags and identifying key components within them.

### Parsing Tags

The primary goal of the parser is to identify and process HTML tags. Tags are parsed as the fundamental building blocks of HTML structure. The parser can recognize tags enclosed in angle brackets (e.g., `<div>`, `<p>`, `<span>`) and correctly identify whether they are valid HTML tags.

1. **Tag Parsing**: Tags are parsed by matching the opening `<` and closing `>`. The parser captures the tag name (e.g., `div`, `p`, `span`) and checks that it adheres to the rules of valid HTML tag names.

2. **Whitespace Handling**: The parser also considers spaces, newlines (\n), and tabs (\t) as valid separators for tags and their content. These spaces are ignored during the parsing process unless they are inside tag names, attributes, or content.

3. **Attributes Parsing**: Although this parser focuses mainly on the basic structure of tags, it has the capability to identify attributes like `class`, `id`, `src`, etc., within tags. Each attribute is parsed separately as part of the tag structure.

### Handling Nested and Self-Closing Tags
The parser can also handle:

**Nested Tags**: Tags that are inside other tags (e.g., `<div><span>Text</span></div>`) are correctly recognized and nested properly in the output tree structure.
**Self-Closing Tags**: Tags like `<img />`, `<br />`, and `<hr />` are parsed as self-closing tags without content.

## Grammar

Here is the grammar file which is used to parse html markdown:

```pest
/// Matches a hyphen character.
hyphen = _{"-"}

/// Matches an exclamation mark character.
exclamation_mark = _{"!"}

/// Matches a question mark character.
question_mark = _{"?"}

/// Matches a underscore character.
underscore = _{"_"}

/// Matches a equation mark character.
equation_mark = _{"="}

/// Matches a single quote character.
single_quote = _{"'"}

/// Matches a double quote character.
double_quote = _{"\""}

/// Matches a left chevron character.
left_chevron = _{"<"}

/// Matches a right chevron character.
right_chevron = _{">"}

/// Matches a slash character.
slash = _{"/"}

/// Matches a whitespace characters.
whitespace = _{" " | "\n" | "\r" | "\t"}

/// Matches a digit.
digit = _{'0'..'9'}

/// Matches a small and big latin letters.
alpha = _{'a'..'z' | 'A'..'Z'}

/// Matches a left square bracket character.
left_square_bracket = _{"["}

/// Matches a right square bracket character.
right_square_bracket = _{"]"}

/// Matches a dot character.
dot = _{"."}

/// Matches a comma character.
comma = _{","}

/// Matches a colon character.
colon = _{":"}

/// Matches a semicolon character.
semicolon = _{";"}

/// Matches a space character.
space = _{" "}

/// Matches a dollar character.
dollar = _{"$"}

/// Matches a ampersand character.
ampersand = _{"&"}

/// Matches the opening tag of an HTML comment (`<!--`).
html_comment_opening_tag = _{left_chevron ~ exclamation_mark ~ hyphen ~ hyphen}

/// Matches the closing tag of an HTML comment (`-->`).
html_comment_closing_tag = _{hyphen ~ hyphen ~ right_chevron}

/// Matches the content within an HTML comment, which can be any character except the opening or closing tag.
html_comment_content = {(!(html_comment_opening_tag | html_comment_closing_tag) ~ ANY)*}

/// Matches a complete HTML comment, including the opening tag, optional whitespace, content, and closing tag.
html_comment = {html_comment_opening_tag ~ whitespace* ~ html_comment_content ~ whitespace* ~ html_comment_closing_tag}

/// Matches the name of an HTML attribute, starting with an alphabetic character and followed by alphanumeric characters or hyphens.
attribute_name = !{alpha ~ ((alpha | digit) ~ hyphen?)*}

/// Matches the opening quote of an attribute value (`'` or `"`).
attribute_value_opening_quote = @{PUSH("'" | "\"")}

/// Matches the closing quote of an attribute value.
attribute_value_closing_quote = @{POP}

/// Matches the content of an attribute value, which can be any character except for the left chevron `<`, right chevron `>`, or the quote character.
attribute_value = !{(!(left_chevron | right_chevron | PEEK) ~ ANY)*}

/// Matches a complete attribute as a key-value pair (name="value").
attribute = @{attribute_name ~ equation_mark ~ attribute_value_opening_quote ~ attribute_value ~ attribute_value_closing_quote}

/// Matches the name of an HTML tag, starting with an alphabetic character and followed by alphanumeric characters or hyphens.
tag_name = {alpha ~ ((alpha | digit) ~ hyphen?)*}

/// Matches an opening HTML tag, which includes the tag name and optional attributes.
opening_tag = {left_chevron ~ PUSH(tag_name) ~ (whitespace+ ~ attribute? ~ whitespace*)* ~ right_chevron}

/// Matches a closing HTML tag, which includes a slash and the tag name.
closing_tag = {left_chevron ~ slash ~ POP ~ whitespace* ~ right_chevron}

/// Matches a self-closing HTML tag, which includes a tag name and a slash before the closing chevron.
self_closing_tag = {left_chevron ~ tag_name ~ (whitespace+ ~ attribute? ~ whitespace*)* ~ slash ~ right_chevron}

/// Matches plain text content within HTML that is not part of any HTML tags.
html_text = {(!(html_element | left_chevron | right_chevron) ~ whitespace* ~ ANY)+}

/// Matches an HTML element, which can be a self-closing element or a common element with opening and closing tags.
html_element = {whitespace* ~ self_closing_element | common_element ~ whitespace*}

/// Matches any valid HTML node, which can be a comment, an HTML element, or plain text.
html_node = _{html_comment | html_element | html_text}

/// Matches a standard HTML element with opening and closing tags and optional nested nodes.
common_element = _{(opening_tag ~ whitespace* ~ (!closing_tag ~ html_node)* ~ whitespace* ~ closing_tag)}

/// Matches a self-closing HTML element.
self_closing_element = _{self_closing_tag}

/// Matches the `DOCTYPE` keyword, which can appear at the beginning of an HTML document.
doctype_keyword = _{^"DOCTYPE" | ^"doctype"}

/// Matches the top-level element name in a `DOCTYPE` declaration, which is usually `HTML` or `html`.
doctype_top_level_element = _{^"HTML" | ^"html"}

/// Matches the privacy level in a `DOCTYPE` declaration, such as `PUBLIC` or `SYSTEM`.
doctype_privacy_level = _{^"PUBLIC" | ^"SYSTEM" | ^"public" | ^"system"}

/// Matches characters allowed in `DOCTYPE` parameters, including alphanumeric characters, hyphens, dots, colons, semicolons, and spaces.
doctype_params_allowed_symbol = _{alpha | digit | hyphen | dot | colon | semicolon | comma | space}

/// Matches characters allowed in a `DOCTYPE` URL, including various punctuation marks.
doctype_url_allowed_symbol = _{doctype_params_allowed_symbol | exclamation_mark | question_mark | underscore | equation_mark | single_quote | slash | left_square_bracket | right_square_bracket | dollar | ampersand}

/// Matches the parameters of a `DOCTYPE` declaration, typically in a quoted string.
doctype_params = _{double_quote ~ doctype_params_allowed_symbol+ ~ slash ~ slash ~ doctype_params_allowed_symbol+ ~ slash ~ slash ~ doctype_params_allowed_symbol+ ~ slash ~ slash ~ doctype_params_allowed_symbol+ ~ double_quote}

/// Matches a URL in a `DOCTYPE` declaration, typically in a quoted string.
doctype_url = _{double_quote ~ doctype_url_allowed_symbol+ ~ double_quote}

/// Matches a complete `DOCTYPE` declaration, which includes the `DOCTYPE` keyword, an optional top-level element, privacy level, parameters, and URL.
doctype_declaration = {left_chevron ~ exclamation_mark ~ doctype_keyword ~ (whitespace+ ~ doctype_top_level_element ~ (whitespace+ ~ doctype_privacy_level ~ whitespace+ ~ doctype_params ~ whitespace+ ~ doctype_url)*)* ~ right_chevron}

/// Matches a complete HTML document structure, including optional comments, an optional `DOCTYPE` declaration, and at least one HTML element.
html = {SOI ~ whitespace* ~ html_comment* ~ whitespace* ~ doctype_declaration? ~ whitespace* ~ html_element ~ whitespace* ~ EOI}
```

## Installation

1. Install [Rust](https://www.rust-lang.org/tools/install) if you haven't already
2. Clone the repository

```bash
git clone https://github.com/kalyonekenobe/bobo_html_parser.git
```

or using SSH

```bash
git clone git@github.com:kalyonekenobe/bobo_html_parser.git
```

3. Go to project's directory

```bash
cd bobo_html_parser
```

4. Build the project

```bash
cargo build
```

## Usage

To use the HTML parser, simply add your HTML code to the input string in the provided `main.rs` file
and run the program using `cargo run`. The parser will read the HTML code, process it, and output
the parsed result. For example, if you want to parse a `<div>` tag, update the `input` string like
this:

```rust
fn main() {
    let input = "<div>";

    let result = Grammar::parse(Rule::opening_tag, input);

    match result {
        Ok(tag) => println!("Parsed tag: {}", tag.as_str()),
        Err(e) => eprintln!("Error parsing tag: {}", e),
    }
}
```

```bash
cargo run
```

Then, run the following command in your terminal to execute the program.

You are also able to use CLI to parse necessary HTML file. To do this you can simpy use this command:

```bash
cargo run -- --file <path-to-the-file> parse
```

There are the `default.html` file in the root of the project, so you can easily test this.

If you need some help with CLI commands just use this:

```bash
cargo run -- --help
```

You are also able to use `makefile` available in the root of the project.

## Project Structure

```
bobo_html_parser/ 
└── src/ 
    └── grammar/
        ├── grammar.pest        # Grammar file for parsing HTML
        └── mod.rs              # Grammar module
    ├── cli.rs                  # CLI config file
    ├── error.rs                # Errors config file
    ├── lib.rs                  # Lib file with core module 
    └── main.rs                 # Main Rust code for the parser
└── tests/
    └── grammar.rs              # Tests for html parser grammar rules 
└── target/                     # (does not exist until the project is built)
    └── ...                     # Build directories and files
├── .gitignore                  # File to ignore local .git changes
├── Cargo.toml                  # Cargo.toml
├── Cargo.lock                  # Cargo.lock
├── default.html                # Default HTML file is used for tests 
├── makefile                    # File with the list of terminal commands
└── README.md                   # README.md
```

## Example Output

Here is the example of Rust code:

```rust
use bobo_html_parser::core::{Grammar, Rule};
use pest::Parser;
use pest_ascii_tree::print_ascii_tree;

fn main() -> anyhow::Result<()> {
    let html = "
    <!DOCTYPE html>
    <html>
        <body>

            <h1 class='heading'>My First Heading</h1>

            <p>My first paragraph.</p>

        </body>
    </html>
    ";

    let grammar = Grammar::parse(Rule::html, &html);
    print_ascii_tree(grammar);

    Ok(())
}
```

The output for this Rust program is:

```bash
html
├─ doctype_declaration "<!DOCTYPE\ html>"
└─ html_element
    ├─ opening_tag
    │  └─ tag_name "html"
    ├─ html_element
    │  ├─ opening_tag
    │  │  └─ tag_name "body"
    │  ├─ html_element
    │  │  ├─ opening_tag
    │  │  │  ├─ tag_name "h1"
    │  │  │  └─ attribute
    │  │  │     ├─ attribute_name "class"
    │  │  │     └─ attribute_value "heading"
    │  │  ├─ html_text "My\ First\ Heading"
    │  │  └─ closing_tag "</h1>"
    │  ├─ html_element
    │  │  ├─ opening_tag
    │  │  │  └─ tag_name "p"
    │  │  ├─ html_text "My\ first\ paragraph."
    │  │  └─ closing_tag "</p>"
    │  └─ closing_tag "</body>"
    └─ closing_tag "</html>"
```
