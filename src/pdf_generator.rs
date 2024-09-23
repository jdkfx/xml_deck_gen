use genpdf::elements::{Break, OrderedList, PageBreak, Paragraph, UnorderedList};
use genpdf::Alignment;
use genpdf::Element;
use genpdf::Margins;
use genpdf::{fonts, style::Style, Document, Mm, SimplePageDecorator, Size};
use xml_parser::Node;

use crate::xml_parser;

pub fn generate(output_deck_size: Option<&str>, output_file_name: &String, node: Node) {
    let font_family = fonts::from_files("./fonts/Noto_Sans/static/", "NotoSans", None)
        .expect("Failed to load font family");

    let mut doc = Document::new(font_family);

    let (page_width, page_height): (Mm, Mm) = match output_deck_size {
        Some("wide") => {
            println!("Size is set to wide. Applying wide settings...");
            (Mm::from(254.0), Mm::from(142.9))
        }
        _ => {
            println!("Size is not set. Applying default settings...");
            (Mm::from(275.1), Mm::from(190.5))
        }
    };
    let page_size = Size::new(page_width, page_height);
    doc.set_paper_size(page_size);
    let mut decorator = SimplePageDecorator::new();
    decorator.set_margins(Margins::trbl(
        Mm::from(20.0),
        Mm::from(15.0),
        Mm::from(20.0),
        Mm::from(15.0),
    ));
    doc.set_page_decorator(decorator);

    if node.tag_name == "deck" {
        for (i, child) in node.children.iter().enumerate() {
            if child.tag_name == "page" {
                for grandchild in child.children.clone() {
                    match grandchild.tag_name.as_str() {
                        "title" => {
                            if let Some(text) = grandchild.text {
                                doc.set_title(text.clone());
                                let title_paragraph = Paragraph::new(text.clone())
                                    .aligned(Alignment::Center)
                                    .styled(Style::new().with_font_size(48));
                                doc.push(title_paragraph);
                            }
                        }
                        "head" => {
                            if let Some(text) = grandchild.text {
                                let text_paragraph = Paragraph::new(text)
                                    .styled(Style::new().bold().with_font_size(30));
                                doc.push(text_paragraph);
                            }
                        }
                        "text" => {
                            if let Some(text) = grandchild.text {
                                let text_paragraph =
                                    Paragraph::new(text).styled(Style::new().with_font_size(24));
                                doc.push(text_paragraph);
                            }
                        }
                        "ul" => {
                            let mut list = UnorderedList::new();
                            for item in grandchild.children {
                                if item.tag_name == "li" {
                                    if let Some(text) = item.text {
                                        list.push(Paragraph::new(text));
                                    }
                                }
                            }
                            let styled_list = list.styled(Style::new().with_font_size(24));
                            doc.push(styled_list);
                        }
                        "ol" => {
                            let mut list = OrderedList::new();
                            for item in grandchild.children {
                                if item.tag_name == "li" {
                                    if let Some(text) = item.text {
                                        list.push(Paragraph::new(text));
                                    }
                                }
                            }
                            let styled_list = list.styled(Style::new().with_font_size(24));
                            doc.push(styled_list);
                        }
                        "br" => {
                            doc.push(Break::new(1));
                        }
                        _ => {}
                    }
                }

                if i + 1 < node.children.len() && node.children[i + 1].tag_name == "page" {
                    doc.push(PageBreak::new());
                }
            }
        }
    }

    println!("Exporting PDF files...");
    doc.render_to_file(output_file_name)
        .expect("Failed to write PDF file");
}
