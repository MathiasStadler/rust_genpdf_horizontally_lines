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


    doc.set_title("Test");
    doc.set_minimal_conformance();
    doc.set_line_spacing(1.25);

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
    doc.set_page_decorator(decorator);

    doc.push(
        elements::Paragraph::new("genpdf Demo Document")
            .aligned(Alignment::Center)
            .styled(style::Style::new().bold().with_font_size(20)),
    );
    doc.push(elements::Break::new(1.5));
    /*doc.push(elements::Paragraph::new(
        "This document demonstrates how the genpdf crate generates PDF documents. I removed a bunch of code from the demo and now it looks silly",
    ));
    */
    doc.push(elements::Paragraph::new("First sentences",));

    doc.push(Line);

    doc.push(elements::Paragraph::new("Second sentences",));

    doc.push(Line);

    doc.push(elements::Paragraph::new("First sentences",));

    doc.push(elements::Break::new(1.5));

    doc.push(elements::Paragraph::new("______________underline__________",));

    //page break
    doc.push(genpdf::elements::PageBreak::new());

    doc.push(elements::Paragraph::new("lehr",));

    doc.push(elements::Paragraph::new("______________underline__________",));



    let output_file_path = "output.pdf";
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

