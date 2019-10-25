use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use std::path::PathBuf;

use sdl2::render::Canvas;
use sdl2::event::Event;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::rect::{Rect, Point};
use sdl2::Sdl;

use crate::texture_cache::{CanvasExt, TextureCache, Image, Analog, Trigger};
use crate::controller_state::ControllerState;
use crate::configuration::{ThemeConfiguration, TriggerDirection};
use crate::error::{Error, Result};

pub struct InputWindow {
    sdl: Sdl,
    canvas: Canvas<Window>,
    state: Arc<Mutex<ControllerState>>,
}

impl InputWindow {
    pub fn new(conf: &ThemeConfiguration, state: Arc<Mutex<ControllerState>>) -> Result<InputWindow> {
        let sdl = sdl2::init()?;
        let video = sdl.video()?;

        let window = video.window("GC Input Viewer", conf.size.0, conf.size.1)
            .position_centered()
            .build()
            .map_err(|e| Error::Sdl2Error(e.into()))?;

        let canvas = window.into_canvas()
            .accelerated()
            .target_texture()
            .build()
            .map_err(|e| Error::Sdl2Error(e.into()))?;

        Ok(InputWindow{
            sdl: sdl,
            canvas: canvas,
            state: state,
        })
    }

    fn draw_image(&mut self, image: &Image) -> Result<()> {
        self.canvas.copy(&image.tex, None, image.dst)?;
        Ok(())
    }

    fn draw_analog(&mut self, analog: &Analog, position: (u8, u8)) -> Result<()> {
        let xoffset = ((position.0 as f32 / 256.0) * 2.0 * analog.range.0 as f32) as i32 - analog.range.0;
        let yoffset = analog.range.1 - ((position.1 as f32 / 256.0) * 2.0 * analog.range.1 as f32) as i32;

        let mut dst = analog.image.dst.clone();
        dst.offset(xoffset, yoffset);

        if let Some((x, y)) = analog.line_from {
            self.canvas.set_draw_color(Color::RGB(0, 0, 0));
            self.canvas.draw_line(Point::new(x, y), Point::new(x + xoffset, y + yoffset))?;
        }
        self.canvas.copy(&analog.image.tex, None, dst)?;
        Ok(())
    }

    fn draw_trigger(&mut self, trigger: &Trigger, value: u8) -> Result<()> {
        let tex_info = trigger.image.tex.query();

        // Hacky fix to value 0 still showing up
        if value == 0 {
            return Ok(());
        }

        let (src, dst) = match trigger.direction {
            TriggerDirection::Up => {
                let src_h = ((tex_info.height as f32 * value as f32) / 256.0) as u32;
                let dst_h = ((trigger.image.dst.height() as f32 * value as f32) / 256.0) as u32;

                let src = Rect::new(0, (tex_info.height - src_h) as i32, tex_info.width, src_h);
                let mut dst = trigger.image.dst;
                dst.set_height(dst_h);
                dst.offset(0, (trigger.image.dst.height() - dst_h) as i32);
                (src, dst)
            }
            TriggerDirection::Down => {
                let src_h = ((tex_info.height as f32 * value as f32) / 256.0) as u32;
                let dst_h = ((trigger.image.dst.height() as f32 * value as f32) / 256.0) as u32;

                let src = Rect::new(0, 0, tex_info.width, src_h);
                let mut dst = trigger.image.dst;
                dst.set_height(dst_h);
                (src, dst)
            }
            TriggerDirection::Left => {
                let src_w = ((tex_info.width as f32 * value as f32) / 256.0) as u32;
                let dst_w = ((trigger.image.dst.width() as f32 * value as f32) / 256.0) as u32;

                let src = Rect::new((tex_info.width - src_w) as i32, 0, src_w, tex_info.height);
                let mut dst = trigger.image.dst;
                dst.set_width(dst_w);
                dst.offset((trigger.image.dst.width() - dst_w) as i32, 0);
                (src, dst)
            }
            TriggerDirection::Right => {
                let src_w = ((tex_info.width as f32 * value as f32) / 256.0) as u32;
                let dst_w = ((trigger.image.dst.width() as f32 * value as f32) / 256.0) as u32;

                let src = Rect::new(0, 0, src_w, tex_info.height);
                let mut dst = trigger.image.dst;
                dst.set_width(dst_w);
                (src, dst)
            }
        };

        self.canvas.copy(&trigger.image.tex, src, dst)?;
        Ok(())
    }

    fn update(&mut self, textures: &mut TextureCache, state: ControllerState) -> Result<()> {
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        self.canvas.clear();

        self.draw_image(&textures.background)?;
        if state.a {
            textures.a
                .as_ref()
                .map(|i| self.draw_image(i))
                .unwrap_or(Ok(()))?;
        }
        if state.b {
            textures.b
                .as_ref()
                .map(|i| self.draw_image(i))
                .unwrap_or(Ok(()))?;
        }
        if state.x {
            textures.x
                .as_ref()
                .map(|i| self.draw_image(i))
                .unwrap_or(Ok(()))?;
        }
        if state.y {
            textures.y
                .as_ref()
                .map(|i| self.draw_image(i))
                .unwrap_or(Ok(()))?;
        }
        if state.up {
           textures.up
                .as_ref()
                .map(|i| self.draw_image(i))
                .unwrap_or(Ok(()))?;
        }
        if state.down {
            textures.down
                .as_ref()
                .map(|i| self.draw_image(i))
                .unwrap_or(Ok(()))?;
        }
        if state.left {
            textures.left
                .as_ref()
                .map(|i| self.draw_image(i))
                .unwrap_or(Ok(()))?;
        }
        if state.right {
            textures.right
                .as_ref()
                .map(|i| self.draw_image(i))
                .unwrap_or(Ok(()))?;
        }
        if state.start {
            textures.start
                .as_ref()
                .map(|i| self.draw_image(i))
                .unwrap_or(Ok(()))?;
        }
        if state.l_digital {
            textures.l_digital
                .as_ref()
                .map(|i| self.draw_image(i))
                .unwrap_or(Ok(()))?;
        }
        if state.r_digital {
            textures.r_digital
                .as_ref()
                .map(|i| self.draw_image(i))
                .unwrap_or(Ok(()))?;
        }
        if state.z {
            textures.z.as_ref()
                .as_ref()
                .map(|i| self.draw_image(i))
                .unwrap_or(Ok(()))?;
        }

        textures.analog
            .as_ref()
            .map(|i| self.draw_analog(i, state.analog))
            .unwrap_or(Ok(()))?;
        textures.c
            .as_ref()
            .map(|i| self.draw_analog(i, state.c))
            .unwrap_or(Ok(()))?;

        textures.l_analog
            .as_ref()
            .map(|i| self.draw_trigger(i, state.l_analog))
            .unwrap_or(Ok(()))?;
        textures.r_analog
            .as_ref()
            .map(|i| self.draw_trigger(i, state.r_analog))
            .unwrap_or(Ok(()))?;

        self.canvas.present();
        Ok(())
    }

    pub fn run(&mut self, base: PathBuf, conf: ThemeConfiguration) -> Result<()> {
        let tex_cache_creator = self.canvas.texture_cache_creator(base);
        let mut tex = tex_cache_creator.texture_cache(&conf)?;

        let mut event_pump = self.sdl.event_pump()?;
        'running: loop {

            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} =>
                        break 'running,
                    _ => {}
                }
            }

            let state = *self.state.lock().unwrap();
            self.update(&mut tex, state)?;

            thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }

        Ok(())
    }
}
