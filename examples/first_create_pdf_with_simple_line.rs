use genpdf::{
    elements,
    fonts::{self},
    style, Alignment, Element, Position, RenderResult, Size,
};
//from here
//https://linuxhint.com/rust-check-file-exists/
//use std::fs;

const FONT_DIRS: &[&str] = &[
    "/usr/share/fonts/liberation",
    "/usr/share/fonts/truetype/liberation",
];

const DEFAULT_FONT_NAME: &'static str = "LiberationSans";

fn main() {
    //from here
    // https://git.sr.ht/~ireas/genpdf-rs/tree/master/examples/demo.rs
    let font_dir = FONT_DIRS
        .iter()
        .filter(|path| std::path::Path::new(path).exists())
        .next()
        .expect("Could not find font directory");

    let default_font =
        fonts::from_files(font_dir, DEFAULT_FONT_NAME, Some(fonts::Builtin::Helvetica))
            .expect("Failed to load the default font family");

    let mut doc = genpdf::Document::new(default_font);

    doc.set_title("Simple Line");
    doc.set_minimal_conformance();
    doc.set_line_spacing(1.0);

    let mut decorator = genpdf::SimplePageDecorator::new();
    decorator.set_margins(10);
    decorator.set_header(|page| {
        let mut layout = elements::LinearLayout::vertical();
        if page > 1 {
            layout.push(
                elements::Paragraph::new(format!("Page {}", page)).aligned(Alignment::Center),
            );
            layout.push(elements::Break::new(1));
        }
        layout.styled(style::Style::new().with_font_size(10))
    });

    //set decorator
    doc.set_page_decorator(decorator);

    //push to page
    doc.push(
        elements::Paragraph::new("genpdf Demo Document")
            .aligned(Alignment::Center)
            .styled(style::Style::new().bold().with_font_size(20)),
    );

    // push line
    doc.push(elements::Break::new(1.0));

    doc.push(elements::Paragraph::new("First sentences"));

    doc.push(Line);

    doc.push(elements::Paragraph::new("Second sentences"));

    doc.push(Line);

    doc.push(elements::Paragraph::new("Third sentences"));

    doc.push(Line);

    /* doc.push(elements::Paragraph::new(
        "______________underline__________",
    ));
    */

    //doc.push(elements::Break::new(1.0));

    //doc.push(elements::Paragraph::new("Hallo Text Line one"));

    // FROM HERE
    // https://docs.rs/genpdf/latest/genpdf/elements/struct.Paragraph.html

    let p = elements::Paragraph::default()
        .string("This is a ")
        .styled_string("important", style::Color::Rgb(255, 0, 0))
        .string(" message!")
        .aligned(genpdf::Alignment::Center);

    doc.push(p);

    // add blank line empty line
    doc.push(elements::Break::new(1.0));
    doc.push(elements::Break::new(1.0));
    doc.push(elements::Break::new(1.0));

    // https://www.loremipsum.de/
    let sample_text = " Test Text : Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet.";
    doc.push(elements::Paragraph::new(sample_text));

    //page break
    doc.push(genpdf::elements::PageBreak::new());

    //empty page
    doc.push(elements::Paragraph::new("empty page"));

    // output file path
    let output_file_path = "pdf_with_simple_line.pdf";

    // render to file amd save
    // old doc.render_to_file(output_file_path).unwrap();
    doc.render_to_file(output_file_path).expect("Failed to render document");
}

struct Line;

impl Element for Line {
    fn render(
        &mut self,
        _: &genpdf::Context,
        area: genpdf::render::Area<'_>,
        style: style::Style,
    ) -> Result<genpdf::RenderResult, genpdf::error::Error> {
        area.draw_line(
            vec![
                Position {
                    x: 0.into(),
                    y: 0.into(),
                },
                Position {
                    x: area.size().width,
                    y: 0.into(),
                },
            ],
            style.with_color(style::Color::Rgb(0, 0, 255)),
        );

        Ok(RenderResult {
            size: Size {
                width: area.size().width,
                height: 1.into(),
            },
            has_more: false,
        })
    }
}
