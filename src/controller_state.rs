pub struct ControllerState {
    pub a: bool,
    pub b: bool,
    pub x: bool,
    pub y: bool,
    pub start: bool,
}

impl Default for ControllerState {
    fn default() -> Self {
        ControllerState {
            a: false,
            b: false,
            x: false,
            y: false,
            start: false,
        }
    }
}