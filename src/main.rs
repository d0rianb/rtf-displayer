// Project linked with the rtf-parser library, to debug the parsing

mod displayer;
mod camera;

use std::thread;
use std::time::Duration;
use speedy2d::color::Color;
use speedy2d::dimen::Vector2;
use speedy2d::{Graphics2D, Window};
use speedy2d::window::{KeyScancode, MouseButton, MouseScrollDistance, VirtualKeyCode, WindowCreationOptions, WindowHandler, WindowHelper, WindowPosition, WindowSize, WindowStartupInfo};
use speedy2d::window::MouseScrollDistance::Pixels;
use crate::displayer::Displayer;

const FPS: u64 = 60;
const FRAME_DURATION: u64 = 1000 / FPS; // ms

pub enum DisplayerEvent {
    Update,
    Render
}

struct DisplayerWindowHandler {
    pub displayer: Displayer,
}

impl WindowHandler<DisplayerEvent> for DisplayerWindowHandler {
    fn on_start(&mut self, helper: &mut WindowHelper<DisplayerEvent>, _info: WindowStartupInfo) {
        let event_sender = helper.create_user_event_sender();
        self.displayer.event_sender = Some(event_sender.clone());
        helper.request_redraw();
        thread::spawn(move || {
            loop {
                event_sender.send_event(DisplayerEvent::Update).unwrap();
                thread::sleep(Duration::from_millis(FRAME_DURATION));
            }
        });
    }

    #[warn(unreachable_patterns)]
    fn on_user_event(&mut self, helper: &mut WindowHelper<DisplayerEvent>, user_event: DisplayerEvent) {
        use DisplayerEvent::*;
        match user_event {
            Render => helper.request_redraw(),
            Update => {},
        }
    }

    fn on_resize(&mut self, _helper: &mut WindowHelper<DisplayerEvent>, _size_pixels: Vector2<u32>) {}

    fn on_draw(&mut self, _helper: &mut WindowHelper<DisplayerEvent>, graphics: &mut Graphics2D) {
        graphics.clear_screen(Color::WHITE);
        self.displayer.render(graphics);
    }

    fn on_key_up(&mut self, helper: &mut WindowHelper<DisplayerEvent>, virtual_key_code: Option<VirtualKeyCode>, scancode: KeyScancode) {
        use VirtualKeyCode::*;
        if let Some (vkc) = virtual_key_code {
            match vkc {
                R => self.displayer.camera.reset(),
                _ => {}
            };
            helper.request_redraw();
        }
    }

    fn on_mouse_move(&mut self, helper: &mut WindowHelper<DisplayerEvent>, position: Vector2<f32>) {
        self.displayer.on_mouse_move(position);
        helper.request_redraw();
    }

    fn on_mouse_button_down(&mut self, _helper: &mut WindowHelper<DisplayerEvent>, _button: MouseButton) {}

    fn on_mouse_button_up(&mut self, _helper: &mut WindowHelper<DisplayerEvent>, _button: MouseButton) {}

    fn on_mouse_wheel_scroll(&mut self, helper: &mut WindowHelper<DisplayerEvent>, distance: MouseScrollDistance) {
        if let Pixels { x, y, z} = distance {
            self.displayer.camera.scroll(x, y);
        }
    }
}


fn main() {
    const WINDOW_WIDTH: f32 = 600.;
    const WINDOW_HEIGHT: f32 = 400.;

    let args: Vec<String> = std::env::args().collect();
    let window = Window::new_with_user_events(
        "RTF displayer",
        WindowCreationOptions::new_windowed(
            WindowSize::ScaledPixels((WINDOW_WIDTH, WINDOW_HEIGHT).into()),
            Some(WindowPosition::Center)
        )
    ).unwrap();
    let mut displayer = Displayer::new();
    if args.len() > 1 {
        let filename = &args[1];
        displayer.load_file(filename);
    } else {
        displayer.load_file("./resources/sample.rtf");
    }

    let window_handler = DisplayerWindowHandler {
        displayer
    };

    window.run_loop(window_handler);
}
