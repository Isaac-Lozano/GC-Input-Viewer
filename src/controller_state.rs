#[derive(Clone, Copy, Debug)]
pub struct ControllerState {
    pub a: bool,
    pub b: bool,
    pub x: bool,
    pub y: bool,
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub start: bool,
    pub analog: (u8, u8),
    pub c: (u8, u8),
    pub l_analog: u8,
    pub r_analog: u8,
    pub l_digital: bool,
    pub r_digital: bool,
    pub z: bool,
}

impl Default for ControllerState {
    fn default() -> Self {
        ControllerState {
            a: false,
            b: false,
            x: false,
            y: false,
            up: false,
            down: false,
            left: false,
            right: false,
            start: false,
            analog: (128, 128),
            c: (128, 128),
            l_analog: 0,
            r_analog: 0,
            l_digital: false,
            r_digital: false,
            z: false,
        }
    }
}