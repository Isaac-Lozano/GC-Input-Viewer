use std::io::{Read, BufRead, BufReader};

use serialport::SerialPort;

use crate::input_reader::InputReader;
use crate::controller_state::ControllerState;

pub struct SerialReader<S> {
    port: BufReader<S>,
}

impl SerialReader<Box<dyn SerialPort>> {
    pub fn from_path(path: &str) -> SerialReader<Box<dyn SerialPort>> {
        let port = serialport::open(path).unwrap();

        SerialReader {
            port: BufReader::new(port),
        }
    }
}

impl<R> InputReader for SerialReader<R>
    where R: Read
{
    fn read_next_input(&mut self) -> ControllerState {
        let mut buf = String::new();
        self.port.read_line(&mut buf).unwrap();

        let mut state = ControllerState::default();
        let mut order = [&mut state.a, &mut state.b, &mut state.x, &mut state.y];

        for (ch, button) in buf.chars().zip(order.iter_mut()) {
            **button = ch == '1';
        }

        state
    }
}