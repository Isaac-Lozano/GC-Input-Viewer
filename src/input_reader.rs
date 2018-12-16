pub mod dtm_reader;
pub mod file_reader;
pub mod serial_reader;

use crate::controller_state::ControllerState;

pub trait InputReader {
    fn read_next_input(&mut self) -> ControllerState;
}