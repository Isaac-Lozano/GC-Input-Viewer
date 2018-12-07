use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};

use sdl2::render::Canvas;
use sdl2::event::Event;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::Sdl;

use crate::texture_cache::{TextureCreatorExt, TextureCache, Image, Analog};
use crate::controller_state::ControllerState;
use crate::configuration::Configuration;

pub struct InputWindow {
    sdl: Sdl,
    canvas: Canvas<Window>,
    state: Arc<Mutex<ControllerState>>,
}

impl InputWindow {
    pub fn new(conf: &Configuration, state: Arc<Mutex<ControllerState>>) -> Result<InputWindow, String> {
        let sdl = sdl2::init().unwrap();
        let video = sdl.video().unwrap();

        let window = video.window("GC Input Viewer", conf.size.0, conf.size.1)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas()
            .accelerated()
            .target_texture()
            .build()
            .unwrap();

        Ok(InputWindow{
            sdl: sdl,
            canvas: canvas,
            state: state,
        })
    }

    fn draw_image(&mut self, image: &Image) {
        self.canvas.copy(&image.tex, None, image.dst).unwrap();
    }

    fn draw_analog(&mut self, analog: &Analog, position: (u8, u8)) {
        let xoffset = ((position.0 as f32 / 256.0) * 2.0 * analog.range.0 as f32) as i32;
        let yoffset = ((position.1 as f32 / 256.0) * 2.0 * analog.range.1 as f32) as i32;

        let mut dst = analog.image.dst.clone();
        dst.offset(xoffset - analog.range.0, analog.range.1 - yoffset);

        self.canvas.copy(&analog.image.tex, None, dst).unwrap();
    }

    fn draw_trigger(&mut self, image: &Image, value: u8) {
        let tex_info = image.tex.query();
        let src_h = ((tex_info.height as f32 * value as f32) / 256.0) as u32;
        let dst_h = ((image.dst.height() as f32 * value as f32) / 256.0) as u32;

        let src = Rect::new(0, (tex_info.height - src_h) as i32, tex_info.width, src_h);
        let mut dst = image.dst;
        dst.set_height(dst_h);
        dst.offset((image.dst.height() - dst_h) as i32, 0);

        self.canvas.copy(&image.tex, src, dst).unwrap();
    }

    fn update(&mut self, textures: &mut TextureCache, state: ControllerState) {
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        self.canvas.clear();

        self.draw_image(&textures.background);
        if state.a {
            self.draw_image(&textures.a);
        }
        if state.b {
            self.draw_image(&textures.b);
        }
        if state.x {
            self.draw_image(&textures.x);
        }
        if state.y {
            self.draw_image(&textures.y);
        }
        if state.start {
            self.draw_image(&textures.start);
        }
        if state.l_digital {
            self.draw_image(&textures.l_digital);
        }
        if state.r_digital {
            self.draw_image(&textures.r_digital);
        }

        self.draw_analog(&textures.analog, state.analog);
        self.draw_analog(&textures.c, state.c);

        self.draw_trigger(&textures.l_analog, state.l_analog);
        self.draw_trigger(&textures.r_analog, state.r_analog);

        self.canvas.present();
    }

    pub fn run(&mut self, conf: Configuration) {
        let tex_creator = self.canvas.texture_creator();
        let mut tex = tex_creator.texture_cache(&conf);

        let mut event_pump = self.sdl.event_pump().unwrap();
        'running: loop {

            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} =>
                        break 'running,
                    _ => {}
                }
            }

            let state = *self.state.lock().unwrap();
            self.update(&mut tex, state);

            thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}