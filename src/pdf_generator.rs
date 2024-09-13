use genpdf::elements::Paragraph;
use genpdf::Element;
use genpdf::{fonts, style::Style, Document, Mm, SimplePageDecorator, Size};
use xml_parser::Node;

use crate::xml_parser;

pub fn generate(output_deck_size: Option<&str>, output_file_name: &String, node: Node) {
    let font_family = fonts::from_files("./fonts/Noto_Sans/static/", "NotoSans", None)
        .expect("Failed to load font family");

    let mut doc = Document::new(font_family);

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

    if node.tag_name == "deck" {
        for child in node.children {
            if child.tag_name == "page" {
                for grandchild in child.children {
                    match grandchild.tag_name.as_str() {
                        "title" => {
                            if let Some(text) = grandchild.text {
                                doc.set_title(text.clone());
                                let title_paragraph = Paragraph::new(text.clone())
                                    .styled(Style::new().with_font_size(24));
                                doc.push(title_paragraph);
                            }
                        }
                        "text" => {
                            if let Some(text) = grandchild.text {
                                let text_paragraph = Paragraph::new(text).styled(style);
                                doc.push(text_paragraph);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    println!("Exporting PDF files...");
    doc.render_to_file(output_file_name)
        .expect("Failed to write PDF file");
}
