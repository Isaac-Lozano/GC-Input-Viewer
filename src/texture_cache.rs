use std::path::{Path, PathBuf};

use sdl2::render::{Canvas, TextureCreator, Texture, TextureAccess};
use sdl2::image::LoadTexture;
use sdl2::rect::Rect;
use sdl2::video::{Window, WindowContext};

use crate::configuration::{Configuration, ImageConf, AnalogConf};

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
    pub a: Image<'a>,
    pub b: Image<'a>,
    pub x: Image<'a>,
    pub y: Image<'a>,
    pub up: Image<'a>,
    pub down: Image<'a>,
    pub left: Image<'a>,
    pub right: Image<'a>,
    pub start: Image<'a>,
    pub analog: Analog<'a>,
    pub c: Analog<'a>,
    pub l_analog: Image<'a>,
    pub r_analog: Image<'a>,
    pub l_digital: Image<'a>,
    pub r_digital: Image<'a>,
    pub z: Image<'a>,
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

    pub fn texture_cache(&self, conf: &Configuration) -> TextureCache {
        let background = self.read_image(&conf.background);
        let a = self.read_image(&conf.a);
        let b = self.read_image(&conf.b);
        let x = self.read_image(&conf.x);
        let y = self.read_image(&conf.y);
        let up = self.read_image(&conf.up);
        let down = self.read_image(&conf.down);
        let left = self.read_image(&conf.left);
        let right = self.read_image(&conf.right);
        let start = self.read_image(&conf.start);
        let a_marker = self.read_analog(&conf.analog);
        let c_marker = self.read_analog(&conf.c);
        let l_analog = self.read_image(&conf.l_analog);
        let r_analog = self.read_image(&conf.r_analog);
        let l_digital = self.read_image(&conf.l_digital);
        let r_digital = self.read_image(&conf.r_digital);
        let z = self.read_image(&conf.z);

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