use std::path::{Path, PathBuf};

use sdl2::render::{Canvas, TextureCreator, Texture};
use sdl2::image::LoadTexture;
use sdl2::rect::Rect;
use sdl2::video::{Window, WindowContext};

use crate::configuration::{ThemeConfiguration, ImageConf, AnalogConf, TriggerConf, TriggerDirection};
use crate::error::Result;

pub struct Image<'a> {
    pub tex: Texture<'a>,
    pub dst: Rect,
}

pub struct Analog<'a> {
    pub image: Image<'a>,
    pub range: (i32, i32),
    pub line_from: Option<(i32, i32)>,
}

pub struct Trigger<'a> {
    pub image: Image<'a>,
    pub direction: TriggerDirection,
}

pub struct TextureCache<'a> {
    pub background: Image<'a>,
    pub a: Option<Image<'a>>,
    pub b: Option<Image<'a>>,
    pub x: Option<Image<'a>>,
    pub y: Option<Image<'a>>,
    pub up: Option<Image<'a>>,
    pub down: Option<Image<'a>>,
    pub left: Option<Image<'a>>,
    pub right: Option<Image<'a>>,
    pub start: Option<Image<'a>>,
    pub analog: Option<Analog<'a>>,
    pub c: Option<Analog<'a>>,
    pub l_analog: Option<Trigger<'a>>,
    pub r_analog: Option<Trigger<'a>>,
    pub l_digital: Option<Image<'a>>,
    pub r_digital: Option<Image<'a>>,
    pub z: Option<Image<'a>>,
}

pub struct TextureCacheCreator<T> {
    path: PathBuf,
    tex_creator: TextureCreator<T>,
}

impl<T> TextureCacheCreator<T> {
    fn read_image<'a>(&'a self, conf: &ImageConf) -> Result<Image<'a>> {
        let final_path = self.path.join(&conf.path);
        let tex = self.tex_creator.load_texture(final_path)?;
        let (w, h) = conf.size.unwrap_or_else(|| {
            let query = tex.query();
            (query.width, query.height)
        });

        let dst = Rect::new(conf.dst.0, conf.dst.1, w, h);

        Ok(Image {
            tex: tex,
            dst: dst,
        })
    }

    fn read_analog<'a>(&'a self, conf: &AnalogConf) -> Result<Analog<'a>> {
        let image = self.read_image(&conf.image)?;

        Ok(Analog {
            image: image,
            range: conf.range,
            line_from: conf.line_from,
        })
    }

    fn read_trigger<'a>(&'a self, conf: &TriggerConf) -> Result<Trigger<'a>> {
        let image = self.read_image(&conf.image)?;

        Ok(Trigger {
            image: image,
            direction: conf.direction,
        })
    }

    pub fn texture_cache(&self, conf: &ThemeConfiguration) -> Result<TextureCache> {
        let background = self.read_image(&conf.background)?;
        let a = match conf.a.as_ref() {
            Some(image) => Some(self.read_image(image)?),
            None => None,
        };
        let b = match conf.b.as_ref() {
            Some(image) => Some(self.read_image(image)?),
            None => None,
        };
        let x = match conf.x.as_ref() {
            Some(image) => Some(self.read_image(image)?),
            None => None,
        };
        let y = match conf.y.as_ref() {
            Some(image) => Some(self.read_image(image)?),
            None => None,
        };
        let up = match conf.up.as_ref() {
            Some(image) => Some(self.read_image(image)?),
            None => None,
        };
        let down = match conf.down.as_ref() {
            Some(image) => Some(self.read_image(image)?),
            None => None,
        };
        let left = match conf.left.as_ref() {
            Some(image) => Some(self.read_image(image)?),
            None => None,
        };
        let right = match conf.right.as_ref() {
            Some(image) => Some(self.read_image(image)?),
            None => None,
        };
        let start = match conf.start.as_ref() {
            Some(image) => Some(self.read_image(image)?),
            None => None,
        };
        let a_marker = match conf.analog.as_ref() {
            Some(analog) => Some(self.read_analog(analog)?),
            None => None,
        };
        let c_marker = match conf.c.as_ref() {
            Some(analog) => Some(self.read_analog(analog)?),
            None => None,
        };
        let l_analog = match conf.l_analog.as_ref() {
            Some(trigger) => Some(self.read_trigger(trigger)?),
            None => None,
        };
        let r_analog = match conf.r_analog.as_ref() {
            Some(trigger) => Some(self.read_trigger(trigger)?),
            None => None,
        };
        let l_digital = match conf.l_digital.as_ref() {
            Some(image) => Some(self.read_image(image)?),
            None => None,
        };
        let r_digital = match conf.r_digital.as_ref() {
            Some(image) => Some(self.read_image(image)?),
            None => None,
        };
        let z = match conf.z.as_ref() {
            Some(image) => Some(self.read_image(image)?),
            None => None,
        };

        Ok(TextureCache {
            background: background,
            a: a,
            b: b,
            x: x,
            y: y,
            up: up,
            down: down,
            left: left,
            right: right,
            start: start,
            analog: a_marker,
            c: c_marker,
            l_analog: l_analog,
            r_analog: r_analog,
            l_digital: l_digital,
            r_digital: r_digital,
            z: z,
        })
    }
}

pub trait CanvasExt<T> {
    fn texture_cache_creator<P>(&self, path: P) -> TextureCacheCreator<T>
        where P: AsRef<Path>;
}

impl CanvasExt<WindowContext> for Canvas<Window> {
    fn texture_cache_creator<P>(&self, path: P) -> TextureCacheCreator<WindowContext>
        where P: AsRef<Path>,
    {
        let tex_creator = self.texture_creator();

        TextureCacheCreator {
            // TODO: I want Path to clone and PathBuf to move.
            path: path.as_ref().to_owned(),
            tex_creator: tex_creator,
        }
    }
}