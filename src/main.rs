fn main() {
    println!("Exporting PDF files...");

    let font_family = genpdf::fonts::from_files("./fonts/Noto_Sans/static/", "NotoSans", None)
        .expect("Failed to load font family");
    let mut doc = genpdf::Document::new(font_family);
    doc.set_title("Demo document");
    let mut decorator = genpdf::SimplePageDecorator::new();
    decorator.set_margins(10);
    doc.set_page_decorator(decorator);
    doc.push(genpdf::elements::Paragraph::new("This is a demo document."));
    doc.render_to_file("output.pdf").expect("Failed to write PDF file");
}
