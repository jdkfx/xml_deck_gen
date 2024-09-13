use genpdf::elements::Paragraph;
use genpdf::Element;
use genpdf::{fonts, style::Style, Document, Mm, SimplePageDecorator, Size};
use regex::Regex;
use std::env;
use std::fs::File;
use std::io::Read;

#[derive(Debug, PartialEq)]
enum State {
    Data,
    TagOpen,
    EndTagOpen,
    TagName,
    SelfClosingStartTag,
}

#[derive(Debug, Clone)]
enum TokenType {
    StartTag,
    EndTag,
    Char,
    Eof,
}

#[derive(Debug, Clone)]
struct Token {
    token_type: TokenType,
    tag_name: Option<String>,
    self_closing: bool,
    data: Option<String>,
}

impl Token {
    fn new(
        token_type: TokenType,
        tag_name: Option<String>,
        self_closing: bool,
        data: Option<String>,
    ) -> Self {
        Token {
            token_type,
            tag_name,
            self_closing,
            data,
        }
    }
}

#[derive(Debug, Clone)]
struct Node {
    tag_name: String,
    children: Vec<Node>,
    text: Option<String>,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Error: Not enough arguments!");
        std::process::exit(1);
    }

    if !args[1].ends_with(".xml") {
        eprintln!("Error: Input file name is required!");
        std::process::exit(1);
    }

    if !args[2].ends_with(".pdf") {
        eprintln!("Error: Output file name is required!");
        std::process::exit(1);
    }

    let input_file_name = &args[1];
    let output_file_name = &args[2];
    let output_deck_size = args.get(3).map(|s| s.as_str());

    let mut input_file = File::open(input_file_name).expect("Error: Input File not found!");

    let mut xml_contents = String::new();
    input_file
        .read_to_string(&mut xml_contents)
        .expect("Error: Something went wrong reading the file!");

    let re = Regex::new(r"[\t\n]").unwrap();
    xml_contents = re.replace_all(&xml_contents, "").to_string();

    let font_family = fonts::from_files("./fonts/Noto_Sans/static/", "NotoSans", None)
        .expect("Failed to load font family");

    let mut doc = Document::new(font_family);
    doc.set_title("Demo document");

    let (page_width, page_height, font_size): (Mm, Mm, u8) = match output_deck_size {
        Some("wide") => {
            println!("Size is set to wide. Applying wide settings...");
            (Mm::from(254.0), Mm::from(142.9), 14u8)
        }
        _ => {
            println!("Size is not set. Applying default settings...");
            (Mm::from(275.1), Mm::from(190.5), 12u8)
        }
    };
    let page_size = Size::new(page_width, page_height);
    doc.set_paper_size(page_size);

    let mut decorator = SimplePageDecorator::new();
    decorator.set_margins(10);
    doc.set_page_decorator(decorator);

    let style = Style::new().with_font_size(font_size);

    let paragraph = Paragraph::new("This product is a Rust application that quickly, easily, and simply generates PDF files of slides. The slide size is set to 16:9 when 'wide' is specified as a command argument at runtime. Currently, the application only has the functionality to read text written in the program and generate PDF files. However, in the future, we plan to implement a feature that will allow the creation of slides by reading XML files.")
        .styled(style);

    for _ in 0..10 {
        doc.push(paragraph.clone());
    }

    let tokens = tokenize(&xml_contents);

    match parse(&tokens) {
        Ok(node) => {
            println!("Parsed node: {:?}", node);
        }
        Err(err) => {
            eprintln!("Parsing error: {}", err);
        }
    }

    println!("Exporting PDF files...");
    doc.render_to_file(output_file_name)
        .expect("Failed to write PDF file");
}

fn tokenize(xml: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut state = State::Data;
    let mut current_token: Option<Token> = None;

    let mut chars = xml.chars().peekable();
    while let Some(c) = chars.next() {
        match state {
            State::Data => {
                if c == '<' {
                    state = State::TagOpen;
                } else {
                    tokens.push(Token::new(
                        TokenType::Char,
                        None,
                        false,
                        Some(c.to_string()),
                    ));
                }
            }
            State::TagOpen => {
                if c == '/' {
                    state = State::EndTagOpen;
                } else if c.is_alphabetic() {
                    current_token = Some(Token::new(
                        TokenType::StartTag,
                        Some(c.to_string()),
                        false,
                        None,
                    ));
                    state = State::TagName;
                } else {
                    tokens.push(Token::new(
                        TokenType::Char,
                        None,
                        false,
                        Some('<'.to_string()),
                    ));
                    state = State::Data;
                }
            }
            State::EndTagOpen => {
                if c.is_alphabetic() {
                    current_token = Some(Token::new(
                        TokenType::EndTag,
                        Some(c.to_string()),
                        false,
                        None,
                    ));
                    state = State::TagName;
                }
            }
            State::TagName => {
                if c == '/' {
                    state = State::SelfClosingStartTag;
                } else if c == '>' {
                    if let Some(token) = current_token.take() {
                        tokens.push(token);
                    }
                    state = State::Data;
                } else if c.is_uppercase() {
                    if let Some(ref mut token) = current_token {
                        token.tag_name =
                            Some(token.tag_name.clone().unwrap() + &c.to_lowercase().to_string());
                    }
                } else if let Some(ref mut token) = current_token {
                    token.tag_name = Some(token.tag_name.clone().unwrap() + &c.to_string());
                }
            }
            State::SelfClosingStartTag => {
                if c == '>' {
                    if let Some(ref mut token) = current_token {
                        token.self_closing = true;
                        tokens.push(token.clone());
                    }
                    state = State::Data;
                }
            }
        }
    }

    tokens.push(Token::new(TokenType::Eof, None, false, None));
    tokens
}

fn parse(tokens: &[Token]) -> Result<Node, String> {
    let mut stack: Vec<Node> = Vec::new();
    let mut current_node = Node {
        tag_name: String::new(),
        children: Vec::new(),
        text: None,
    };

    for token in tokens {
        match token.token_type {
            TokenType::StartTag => {
                if let Some(tag_name) = &token.tag_name {
                    if !current_node.tag_name.is_empty() {
                        stack.push(current_node.clone());
                    }
                    current_node = Node {
                        tag_name: tag_name.clone(),
                        children: Vec::new(),
                        text: None,
                    };
                }
            }
            TokenType::EndTag => {
                if let Some(tag_name) = &token.tag_name {
                    if tag_name == &current_node.tag_name {
                        let completed_node = Node {
                            tag_name: current_node.tag_name.clone(),
                            children: current_node.children.clone(),
                            text: current_node.text.clone(),
                        };

                        current_node = if let Some(mut parent) = stack.pop() {
                            parent.children.push(completed_node);
                            parent
                        } else {
                            completed_node
                        };
                    } else {
                        return Err(format!(
                            "Mismatched end tag: expected </{}> but found </{}>",
                            current_node.tag_name, tag_name
                        ));
                    }
                }
            }
            TokenType::Char => {
                if let Some(data) = &token.data {
                    current_node.text = Some(current_node.text.take().unwrap_or_default() + data);
                }
            }
            TokenType::Eof => {
                break;
            }
        }
    }

    if !stack.is_empty() {
        return Err("Unclosed tags remaining.".to_string());
    }

    Ok(current_node)
}
