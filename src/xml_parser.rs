#[derive(Debug, PartialEq)]
pub enum State {
    Data,
    TagOpen,
    EndTagOpen,
    TagName,
    SelfClosingStartTag,
}

#[derive(Debug, Clone)]
pub enum TokenType {
    StartTag,
    EndTag,
    Char,
    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    token_type: TokenType,
    tag_name: Option<String>,
    self_closing: bool,
    data: Option<String>,
}

impl Token {
    pub fn new(
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
pub struct Node {
    pub tag_name: String,
    pub children: Vec<Node>,
    pub text: Option<String>,
}

pub fn tokenize(xml: &str) -> Vec<Token> {
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

pub fn parse(tokens: &[Token]) -> Result<Node, String> {
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
