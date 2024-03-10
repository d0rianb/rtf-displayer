use std::fs;
use rtf_parser::{Lexer, Parser, RtfDocument, StyleBlock};
use speedy2d::color::Color;
use speedy2d::dimen::Vector2;
use speedy2d::font::{Font, TextLayout, TextOptions};
use speedy2d::Graphics2D;
use speedy2d::window::UserEventSender;
use lazy_static::lazy_static;
use crate::camera::Camera;

use crate::DisplayerEvent;

lazy_static! {
    static ref DEFAULT_FONT: Font = {
        let file_content = include_bytes!("../resources/Roboto-Regular.ttf");
        Font::new(file_content).expect("Unable to create font")
    };
}

pub struct Displayer {
    pub event_sender: Option<UserEventSender<DisplayerEvent>>,
    pub document: RtfDocument,
    pub camera: Camera
}

impl Displayer {
    pub fn new() -> Self {
        Displayer {
            event_sender: None,
            document: RtfDocument::default(),
            camera: Default::default(),
        }
    }

    /// Send event back to the WindowHandler
    fn send_event(&self, event: DisplayerEvent) {
        if let Some(es) = &self.event_sender {
            es.send_event(event).unwrap();
        } else {
            println!("Try to send event before setting event sender")
        }
    }

    pub fn load_file(&mut self, filepath: &str) {
        let valid_filepath = fs::canonicalize(filepath).expect("Invalid filepath");
        let file_content = fs::read_to_string(&valid_filepath).expect(&format!("Unable to load file to {}", filepath));
        let tokens = Lexer::scan(&file_content).expect("Unable to scan the file");
        let document = Parser::new(tokens).parse().expect("Unable to parse the file");
        self.document = document;
        self.layout_text();
        self.send_event(DisplayerEvent::Render);
    }

    fn layout_text(&mut self) {

    }

    pub fn on_mouse_move(&self, position: Vector2<f32>) {}

    pub fn render(&self, graphics: &mut Graphics2D) {
        // TODO: header inspection
        // TODO: Camera
        // TODO: proper font support
        // TODO: precompute the layout to have the informations on hover
        graphics.clear_screen(Color::WHITE); // TODO: Duplicate with on_draw of WindowHandler
        let font = &DEFAULT_FONT; // Speedy2d font representation
        let font_size = 24.;
        let mut x = 0.;
        let mut line_number = 0;

        for style_block in &self.document.body {
            let StyleBlock { text, painter } = style_block;
            // Handle line return
            let text_without_return = text.replace("\r", "");
            let lines = text_without_return.split("\n");
            for (i, line) in lines.enumerate() {
                let ftb = font.layout_text(&line, font_size, TextOptions::default());
                if i > 0 { // New line
                    line_number += 1;
                    x = 0.;
                }
                let y = (line_number as f32) * font_size;
                graphics.draw_text(Vector2::new(x + self.camera.offset_x, y + self.camera.offset_y), Color::BLACK, &ftb);
                // TODO: handle multi lines
                x += ftb.width();
            }
        }
    }

}