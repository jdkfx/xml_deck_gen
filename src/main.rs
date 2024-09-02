use std::env;
use genpdf::Element;
use genpdf::{Document, SimplePageDecorator, Size, Mm, fonts, style::Style};
use genpdf::elements::Paragraph;

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
}
