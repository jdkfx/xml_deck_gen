use std::env;
use genpdf::Element;
use genpdf::{Document, SimplePageDecorator, Size, Mm, fonts, style::Style};
use genpdf::elements::Paragraph;

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
    fn new(token_type: TokenType, tag_name: Option<String>, self_closing: bool, data: Option<String>) -> Self {
        Token {
            token_type,
            tag_name,
            self_closing,
            data,
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || !args[1].contains(".pdf") {
        eprintln!("Error: Output file name is required.");
        std::process::exit(1);
    }

    let output_file_name = &args[1];
    let size = args.get(2).map(|s| s.as_str());

    let font_family = fonts::from_files("./fonts/Noto_Sans/static/", "NotoSans", None)
        .expect("Failed to load font family");

    let mut doc = Document::new(font_family);
    doc.set_title("Demo document");

    let (page_width, page_height, font_size): (Mm, Mm, u8) = match size {
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

    println!("Exporting PDF files...");
    doc.render_to_file(output_file_name).expect("Failed to write PDF file");

    show_xml_tokens();
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
                    tokens.push(Token::new(TokenType::Char, None, false, Some(c.to_string())));
                }
            }
            State::TagOpen => {
                if c == '/' {
                    state = State::EndTagOpen;
                } else if c.is_alphabetic() {
                    current_token = Some(Token::new(TokenType::StartTag, Some(c.to_string()), false, None));
                    state = State::TagName;
                } else {
                    tokens.push(Token::new(TokenType::Char, None, false, Some('<'.to_string())));
                    state = State::Data;
                }
            }
            State::EndTagOpen => {
                if c.is_alphabetic() {
                    current_token = Some(Token::new(TokenType::EndTag, Some(c.to_string()), false, None));
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
                        token.tag_name = Some(token.tag_name.clone().unwrap() + &c.to_lowercase().to_string());
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

fn show_xml_tokens() {
    let xml = "<deck><title>Sample Deck</title></deck>";
    let tokens = tokenize(xml);

    for token in tokens {
        println!("{:?}", token);
    }
}
