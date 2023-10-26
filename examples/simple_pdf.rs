// Load a font from the file system
let font_family = genpdf::fonts::from_files("./fonts", "LiberationSans", None)
    .expect("Failed to load font family");
// Create a document and set the default font family
let mut doc = genpdf::Document::new(font_family);
doc.push(genpdf::elements::Paragraph::new("Document content"));
doc.render_to_file("output.pdf").expect("Failed to render document");