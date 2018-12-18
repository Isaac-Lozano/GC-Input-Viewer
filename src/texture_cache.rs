use std::path::{Path, PathBuf};

use sdl2::render::{Canvas, TextureCreator, Texture};
use sdl2::image::LoadTexture;
use sdl2::rect::Rect;
use sdl2::video::{Window, WindowContext};

use crate::configuration::{ThemeConfiguration, ImageConf, AnalogConf};

pub struct Image<'a> {
    pub tex: Texture<'a>,
    pub dst: Rect,
}

pub struct Analog<'a> {
    pub image: Image<'a>,
    pub range: (i32, i32),
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
    pub l_analog: Option<Image<'a>>,
    pub r_analog: Option<Image<'a>>,
    pub l_digital: Option<Image<'a>>,
    pub r_digital: Option<Image<'a>>,
    pub z: Option<Image<'a>>,
}

pub struct TextureCacheCreator<T> {
    path: PathBuf,
    tex_creator: TextureCreator<T>,
}

impl<T> TextureCacheCreator<T> {
    fn read_image<'a>(&'a self, conf: &ImageConf) -> Image<'a> {
        let final_path = self.path.join(&conf.path);
        let tex = self.tex_creator.load_texture(final_path).unwrap();
        let (w, h) = conf.size.unwrap_or_else(|| {
            let query = tex.query();
            (query.width, query.height)
        });

        let dst = Rect::new(conf.dst.0, conf.dst.1, w, h);

        Image {
            tex: tex,
            dst: dst,
        }
    }

    fn read_analog<'a>(&'a self, conf: &AnalogConf) -> Analog<'a> {
        let image = self.read_image(&conf.image);

        Analog {
            image: image,
            range: conf.range,
        }
    }

    pub fn texture_cache(&self, conf: &ThemeConfiguration) -> TextureCache {
        let background = self.read_image(&conf.background);
        let a = conf.a.as_ref().map(|i| self.read_image(i));
        let b = conf.b.as_ref().map(|i| self.read_image(i));
        let x = conf.x.as_ref().map(|i| self.read_image(i));
        let y = conf.y.as_ref().map(|i| self.read_image(i));
        let up = conf.up.as_ref().map(|i| self.read_image(i));
        let down = conf.down.as_ref().map(|i| self.read_image(i));
        let left = conf.left.as_ref().map(|i| self.read_image(i));
        let right = conf.right.as_ref().map(|i| self.read_image(i));
        let start = conf.start.as_ref().map(|i| self.read_image(i));
        let a_marker = conf.analog.as_ref().map(|i| self.read_analog(i));
        let c_marker = conf.c.as_ref().map(|i| self.read_analog(i));
        let l_analog = conf.l_analog.as_ref().map(|i| self.read_image(i));
        let r_analog = conf.r_analog.as_ref().map(|i| self.read_image(i));
        let l_digital = conf.l_digital.as_ref().map(|i| self.read_image(i));
        let r_digital = conf.r_digital.as_ref().map(|i| self.read_image(i));
        let z = conf.z.as_ref().map(|i| self.read_image(i));

        TextureCache {
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
        }
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