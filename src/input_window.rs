use sdl2::render::{Canvas, TextureCreator};
use sdl2::video::{Window, WindowContext};
use sdl2::pixels::Color;
use sdl2::rect::{Rect, Point};
use sdl2::VideoSubsystem;

use crate::texture_cache::{TextureCache, Image};
use crate::controller_state::ControllerState;
use crate::configuration::Configuration;

pub struct InputWindow {
    canvas: Canvas<Window>,
}

impl InputWindow {
    pub fn new(video: VideoSubsystem, conf: &Configuration) -> Result<InputWindow, String> {
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
            canvas: canvas,
        })
    }

    pub fn texture_creator(&self) -> TextureCreator<WindowContext> {
        self.canvas.texture_creator()
    }

    fn draw_image(&mut self, image: &Image) {
        self.canvas.copy(&image.tex, None, image.dst).unwrap();
    }

    pub fn update(&mut self, textures: &mut TextureCache, state: ControllerState) {
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

        self.canvas.present();
    }
}