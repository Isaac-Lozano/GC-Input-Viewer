#[derive(Clone)]
pub struct ImageConf {
    pub path: String,
    pub dst: (i32, i32),
    pub size: Option<(u32, u32)>,
}

#[derive(Clone)]
pub struct AnalogConf {
    pub image: ImageConf,
    pub range: (i32, i32),
}

#[derive(Clone)]
pub struct Configuration {
    pub size: (u32, u32),
    pub background: ImageConf,
    pub a: ImageConf,
    pub b: ImageConf,
    pub x: ImageConf,
    pub y: ImageConf,
    pub start: ImageConf,
    pub analog: AnalogConf,
    pub c: AnalogConf,
}