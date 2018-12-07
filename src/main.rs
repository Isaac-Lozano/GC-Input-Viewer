extern crate sdl2;

mod input_window;
mod configuration;
mod texture_cache;
mod controller_state;

use std::thread;
use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Scancode;

use crate::input_window::InputWindow;
use crate::texture_cache::TextureCreatorExt;
use crate::configuration::{Configuration, ImageConf};
use crate::controller_state::ControllerState;

fn main() {
    println!("Hello, world!");

    let conf = Configuration {
        size: (512, 256),
        background: ImageConf {
            path: "resources/background.png".into(),
            dst: (0, 0),
            size: None,
        },
        a: ImageConf {
            path: "resources/a.png".into(),
            dst: (302, 59),
            size: None,
        },
        b: ImageConf {
            path: "resources/b.png".into(),
            dst: (245, 110),
            size: None,
        },
        x: ImageConf {
            path: "resources/x.png".into(),
            dst: (384, 50),
            size: None,
        },
        y: ImageConf {
            path: "resources/y.png".into(),
            dst: (288, 12),
            size: None,
        },
        start: ImageConf {
            path: "resources/start.png".into(),
            dst: (298, 188),
            size: None,
        },
    };

    let sdl = sdl2::init().unwrap();

    let mut iw = InputWindow::new(sdl.video().unwrap(), &conf).unwrap();
    let tex_creator = iw.texture_creator();
    let mut tex = tex_creator.texture_cache(&conf);

    let mut event_pump = sdl.event_pump().unwrap();
    'running: loop {
        let mut state = ControllerState::default();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} =>
                    break 'running,
                _ => {}
            }
        }

        let kb = event_pump.keyboard_state();

        state.a = kb.is_scancode_pressed(Scancode::A);
        state.b = kb.is_scancode_pressed(Scancode::B);
        state.x = kb.is_scancode_pressed(Scancode::X);
        state.y = kb.is_scancode_pressed(Scancode::Y);
        state.start = kb.is_scancode_pressed(Scancode::Space);

        iw.update(&mut tex, state);

        thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    println!("Hello, world!");
}
