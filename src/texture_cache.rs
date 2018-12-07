use sdl2::render::{TextureCreator, Texture, TextureAccess};
use sdl2::image::LoadTexture;
use sdl2::rect::Rect;

use crate::configuration::{Configuration, ImageConf, AnalogConf};

pub trait TextureCreatorExt {
    fn texture_cache(&self, _: &Configuration) -> TextureCache;
}

pub struct Image<'a> {
    pub tex: Texture<'a>,
    pub dst: Rect,
}

pub struct Analog<'a> {
    pub image: Image<'a>,
    pub range: (i32, i32),
}

pub struct TextureCache<'a> {
    pub vmu: Texture<'a>,
    pub background: Image<'a>,
    pub a: Image<'a>,
    pub b: Image<'a>,
    pub x: Image<'a>,
    pub y: Image<'a>,
    pub start: Image<'a>,
    pub analog: Analog<'a>,
    pub c: Analog<'a>,
    pub l_analog: Image<'a>,
    pub r_analog: Image<'a>,
    pub l_digital: Image<'a>,
    pub r_digital: Image<'a>,
}

fn read_image<'a, T>(tex_creator: &'a TextureCreator<T>, conf: &ImageConf) -> Image<'a> {
    let tex = tex_creator.load_texture(&conf.path).unwrap();
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

fn read_analog<'a, T>(tex_creator: &'a TextureCreator<T>, conf: &AnalogConf) -> Analog<'a> {
    let image = read_image(tex_creator, &conf.image);

    Analog {
        image: image,
        range: conf.range,
    }
}

impl<T> TextureCreatorExt for TextureCreator<T> {
    fn texture_cache(&self, conf: &Configuration) -> TextureCache {
        let vmu = self.create_texture(
                None,
                TextureAccess::Target,
                48,
                32)
            .unwrap();

        let background = read_image(self, &conf.background);
        let a = read_image(self, &conf.a);
        let b = read_image(self, &conf.b);
        let x = read_image(self, &conf.x);
        let y = read_image(self, &conf.y);
        let start = read_image(self, &conf.start);
        let a_marker = read_analog(self, &conf.analog);
        let c_marker = read_analog(self, &conf.c);
        let l_analog = read_image(self, &conf.l_analog);
        let r_analog = read_image(self, &conf.r_analog);
        let l_digital = read_image(self, &conf.l_digital);
        let r_digital = read_image(self, &conf.r_digital);

        TextureCache {
            vmu: vmu,
            background: background,
            a: a,
            b: b,
            x: x,
            y: y,
            start: start,
            analog: a_marker,
            c: c_marker,
            l_analog: l_analog,
            r_analog: r_analog,
            l_digital: l_digital,
            r_digital: r_digital,
        }
    }
}