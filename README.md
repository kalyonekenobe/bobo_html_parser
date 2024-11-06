# HTML Parser

This project is a simple HTML parser built using Rust and the [Pest](https://crates.io/crates/pest)
parsing library. It parses HTML documents, tags, attributes, nested structures, and text content.
This parser is a foundation for more complex parsing logic, which can be extended to handle
self-closing tags, custom error handling, and more.

## Table of Contents

- [Features](#features)
- [Technical Description](#technical-description)
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

## Installation

1. Install [Rust](https://www.rust-lang.org/tools/install) if you haven't already
2. Clone the repository

```bash
git clone https://github.com/kalyonekenobe/html-parser.git
```

or using SSH

```bash
git clone git@github.com:kalyonekenobe/html-parser.git
```

3. Go to project's directory

```bash
cd html-parser
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

    let result = Grammar::parse(Rule::tag, input);

    match result {
        Ok(tag) => println!("Parsed tag: {}", tag.as_str()),
        Err(e) => eprintln!("Error parsing tag: {}", e),
    }
}
```

```bash
cargo run
```

Then, run the following command in your terminal to execute the program:

## Project Structure

```
html-parser/ 
└── src/ 
    └── grammar/
        └──grammar.pest         # Grammar file for parsing HTML
    ├── lib.rs                  # Lib file with core module 
    └── main.rs                 # Main Rust code for the parser
└── tests/
    └── grammar.rs              # Tests for html parser grammar rules 
└── target/                     # (does not exist until the project is built)
    └── ...                     # Build directories and files
├── .gitignore 
├── Cargo.toml 
├── Cargo.lock 
└── README.md
```

## Example Output

Here is the example output of the program with input html `<div>`

```bash
[tag(0, 6)]
```
