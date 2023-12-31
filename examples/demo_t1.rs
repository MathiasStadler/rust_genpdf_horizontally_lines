// rm ouput_t1.pdf && cargo run --example  demo_t1 ouput_t1.pdf
// OUTPUT_FILE=ouput_t1.pdf && if [ -f $OUTPUT_FILE ]; then   rm $OUTPUT_FILE; fi && cargo run --example  demo_t1 $OUTPUT_FILE

// RUST_BACKTRACE=1 cargo run --example  demo_t1 $OUTPUT_FILE

// FROM HERE
// https://git.sr.ht/~ireas/genpdf-rs/tree/v0.2.0/item/examples/demo.rs

// SPDX-FileCopyrightText: 2020 Robin Krahl <robin.krahl@ireas.org>
// SPDX-License-Identifier: CC0-1.0

//! This example generates a demo PDF document and writes it to the path that was passed as the
//! first command-line argument.  You may have to adapt the `FONT_DIRS`, `DEFAULT_FONT_NAME` and
//! `MONO_FONT_NAME` constants for your system so that these files exist:
//! - `{FONT_DIR}/{name}-Regular.ttf`
//! - `{FONT_DIR}/{name}-Bold.ttf`
//! - `{FONT_DIR}/{name}-Italic.ttf`
//! - `{FONT_DIR}/{name}-BoldItalic.ttf`
//! for `name` in {`DEFAULT_FONT_NAME`, `MONO_FONT_NAME`}.
//!
//! The generated document using the latest `genpdf-rs` release is available
//! [here](https://genpdf-rs.ireas.org/examples/demo.pdf).

use std::env;

use genpdf::Alignment;
use genpdf::Element as _;
use genpdf::{elements, fonts, style};

const FONT_DIRS: &[&str] = &[
    "/usr/share/fonts/liberation",
    "/usr/share/fonts/truetype/liberation",
];
const DEFAULT_FONT_NAME: &'static str = "LiberationSans";
const MONO_FONT_NAME: &'static str = "LiberationMono";
const LOREM_IPSUM: &'static str =
    "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut \
    labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco \
    laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in \
    voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat \
    non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

fn main() {
    //read args
    let args: Vec<_> = env::args().skip(1).collect();
    if args.len() != 1 {
        panic!("Missing argument: output file");
    }

    let output_file = &args[0];

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
    doc.set_title("genpdf Demo Document");
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

    let monospace = doc.add_font_family(monospace_font);
    let _code = style::Style::from(monospace).bold();
    let _red = style::Color::Rgb(255, 0, 0);
    let _blue = style::Color::Rgb(0, 0, 255);

    doc.push(elements::Paragraph::new(
        "Now let’s print a long table to demonstrate how page wrapping works:",
    ));

    let mut table = elements::TableLayout::new(vec![1, 5]);

    table.set_cell_decorator(elements::FrameCellDecorator::new(true, true, false));

    table
        .row()
        .element(
            elements::Paragraph::new("Index Spalte 1")
                .styled(style::Effect::Bold)
                .padded(1),
        )
        .element(
            elements::Paragraph::new("Index Spalte 1")
                .styled(style::Effect::Bold)
                .padded(1),
        )
        .element(
            elements::Paragraph::new("Text Spalte 2")
                .styled(style::Effect::Bold)
                .padded(1),
        )
        .push()
        .expect("Invalid table row");

    // for i in 0..10 {
    // add rows
    let rows = 5 + 1; // +1 for last rows
    for i in 1..rows {
        table
            .row()
            .element(elements::Paragraph::new(format!("#{}", i)).padded(1))
            .element(elements::Paragraph::new(LOREM_IPSUM).padded(1))
            .push()
            .expect("Invalid table row");
    }

    doc.push(table);

    doc.render_to_file(output_file)
        .expect("Failed to write output file");
}

// Only import the images if the feature is enabled. This helps verify our handling of feature toggles.
#[cfg(feature = "images")]
mod images {
    use super::*;

    const IMAGE_PATH_JPG: &'static str = "examples/images/test_image.jpg";

    pub fn do_image_test(doc: &mut genpdf::Document) {
        doc.push(elements::Paragraph::new(
            "Here is an example image with default position/scale:",
        ));
        doc.push(elements::Image::from_path(IMAGE_PATH_JPG).expect("Unable to load image"));
        doc.push(elements::Paragraph::new(
            "and here is one that is centered, rotated, and scaled some.",
        ));
        doc.push(
            elements::Image::from_path(IMAGE_PATH_JPG)
                .expect("Unable to load image")
                .with_alignment(Alignment::Center)
                .with_scale(genpdf::Scale::new(0.5, 2))
                .with_clockwise_rotation(45.0),
        );
        doc.push(elements::Paragraph::new(
            "For a full example of image functionality, please see images.pdf.",
        ));
        doc.push(elements::Break::new(1.5));
    }
}
