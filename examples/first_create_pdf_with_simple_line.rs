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
// const MONO_FONT_NAME: &'static str = "LiberationMono";

fn main() {
    /* org from blog
        let mut doc = genpdf::Document::new(
        fonts::from_files("fonts/liberation", "LiberationSans", None).unwrap(),
    );
    */
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
    // let monospace_font = fonts::from_files(font_dir, MONO_FONT_NAME, Some(fonts::Builtin::Courier))
    //   .expect("Failed to load the monospace font family");

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

    /*doc.push(elements::Paragraph::new(
        "This document demonstrates how the genpdf crate generates PDF documents. I removed a bunch of code from the demo and now it looks silly",
    ));
    */

    doc.push(elements::Paragraph::new("First sentences"));

    doc.push(Line);

    doc.push(elements::Paragraph::new("Second sentences"));

    doc.push(Line);

    doc.push(elements::Paragraph::new("Third sentences"));

    doc.push(elements::Break::new(1.0));

    doc.push(elements::Paragraph::new(
        "______________underline__________",
    ));

    doc.push(elements::Break::new(1.0));

    doc.push(elements::Paragraph::new("Hallo Text Line one"));

    // FROM HERE
    // https://docs.rs/genpdf/latest/genpdf/elements/struct.Paragraph.html

    let p = elements::Paragraph::default()
        .string("This is a ")
        .styled_string("important", style::Color::Rgb(255, 0, 0))
        .string(" message!")
        .aligned(genpdf::Alignment::Center);

    doc.push(p);

    doc.push(elements::Break::new(1.0));

    // https://www.loremipsum.de/
    let sample_text = "Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet.";
    doc.push(elements::Paragraph::new(sample_text));

    
    /*
    //page break
    doc.push(genpdf::elements::PageBreak::new());

    doc.push(Line);
    doc.push(Line);
    doc.push(Line);
    doc.push(Line);
    doc.push(Line);
    doc.push(Line);
    */
    /*
    //page break
    doc.push(genpdf::elements::PageBreak::new());

    doc.push(elements::Paragraph::new("______________underline__________",));
    doc.push(elements::Paragraph::new("______________underline__________",));
    doc.push(elements::Paragraph::new("______________underline__________",));
    doc.push(elements::Paragraph::new("______________underline__________",));
    doc.push(elements::Paragraph::new("______________underline__________",));
    doc.push(elements::Paragraph::new("______________underline__________",));
    */

    /*
    doc.push(genpdf::elements::PageBreak::new());

    doc.push(elements::Paragraph::new("Watch the film again. What's the right information?",));
    doc.push(elements::Paragraph::new("______________underline__________",));
    doc.push(elements::Paragraph::new("Watch the film again. What's the right information?",));
    doc.push(elements::Paragraph::new("______________underline__________",));
    doc.push(elements::Paragraph::new("Watch the film again. What's the right information?",));
    doc.push(elements::Paragraph::new("______________underline__________",));
    doc.push(elements::Paragraph::new("Watch the film again. What's the right information?",));
    doc.push(elements::Paragraph::new("______________underline__________",));
    */

    doc.push(elements::Paragraph::new("empty"));

    doc.push(elements::Paragraph::new(
        "______________underline__________",
    ));

    let output_file_path = "pdf_with_simple_line.pdf";
    /*
    match fs::metadata(file_path) {
        Ok(_) => {println!("File exists!");
        doc.render_to_file("output.pdf").unwrap();
                 },
        Err(_) => println!("File does not exist!"),
    }
    */
    // #TODO #ATTENTION NOT MULTI OS SAVE
    doc.render_to_file(output_file_path).unwrap();
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
