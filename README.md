# just a test to make a pdf document  with a horizontally lines

> we foolow the log entry

- create new project
- create example folder and add this to Cargo.toml
- add rust lib genpdf to project and build project
-- Download all dependency

```bash
cargo add genpdf
cargo build
```

- We copy code from FROM_HERE in file first_create_pdf_with_line.rs in folder example

```bash
cat <<EOT > ./examples/first_create_pdf_with_line.rs
use genpdf::{
    elements,
    fonts::{self},
    style, Alignment, Element, Position, RenderResult, Size,
};

const FONT_DIRS: &[&str] = &[
    "/usr/share/fonts/liberation",
    "/usr/share/fonts/truetype/liberation",
];

const DEFAULT_FONT_NAME: &'static str = "LiberationSans";
const MONO_FONT_NAME: &'static str = "LiberationMono";

fn main() {
   /* org from blog
        let mut doc = genpdf::Document::new(
        fonts::from_files("fonts/liberation", "LiberationSans", None).unwrap(),
    );
    */
    let font_dir = FONT_DIRS
        .iter()
        .filter(|path| std::path::Path::new(path).exists())
        .next()
        .expect("Could not find font directory");
    let default_font =
        fonts::from_files(font_dir, DEFAULT_FONT_NAME, Some(fonts::Builtin::Helvetica))
            .expect("Failed to load the default font family");
    let monospace_font = fonts::from_files(font_dir, MONO_FONT_NAME, Some(fonts::Builtin::Courier))
        .expect("Failed to load the monospace font family");

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
    doc.push(elements::Paragraph::new(
        "This document demonstrates how the genpdf crate generates PDF documents. I removed a bunch of code from the demo and now it looks silly",
    ));
    doc.push(Line);

    // #TODO #ATTENTION NOT MULTI OS SAVE
    doc.render_to_file("../output/output.pdf").unwrap();
    
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

EOT
```

- run program with follow command

```bash
cargo build --example first_create_pdf_with_line  
cargo run --example first_create_pdf_with_line
```

- search for font LiberationSans-Regular.ttf
-- found on ubuntu here => /usr/share/fonts/truetype/liberation

- found genpdf add fonts here [src example](https://git.sr.ht/~ireas/genpdf-rs/tree/master/examples/demo.rs)
