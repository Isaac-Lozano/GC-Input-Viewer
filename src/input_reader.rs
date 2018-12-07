use crate::controller_state::ControllerState;

pub trait InputReader {
    fn read_next_input(&mut self) -> ControllerState;
}