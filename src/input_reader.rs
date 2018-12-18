pub mod dtm_reader;
pub mod serial_reader;
#[cfg(windows)]
pub mod sa2_reader;

use crate::controller_state::ControllerState;

pub trait InputReader {
    fn read_next_input(&mut self) -> ControllerState;
}