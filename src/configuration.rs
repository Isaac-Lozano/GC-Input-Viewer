pub struct ImageConf {
    pub path: String,
    pub dst: (i32, i32),
    pub size: Option<(u32, u32)>,
}

pub struct Configuration {
    pub size: (u32, u32),
    pub background: ImageConf,
    pub a: ImageConf,
    pub b: ImageConf,
    pub x: ImageConf,
    pub y: ImageConf,
    pub start: ImageConf,
}