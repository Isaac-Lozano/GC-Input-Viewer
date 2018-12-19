use std::io::{Read, BufRead, BufReader};
use std::time::Duration;

use serialport::{SerialPort, SerialPortSettings, DataBits, FlowControl, Parity, StopBits};

use crate::error::Result;
use crate::input_reader::InputReader;
use crate::controller_state::ControllerState;

pub struct SerialReader<S> {
    port: BufReader<S>,
}

impl SerialReader<Box<dyn SerialPort>> {
    pub fn from_path(path: &str) -> Result<SerialReader<Box<dyn SerialPort>>> {
        let s = SerialPortSettings {
            baud_rate: 115200,
            data_bits: DataBits::Eight,
            flow_control: FlowControl::None,
            parity: Parity::None,
            stop_bits: StopBits::One,
            timeout: Duration::from_millis(10),
        };
        let port = serialport::open_with_settings(path, &s)?;

        Ok(SerialReader {
            port: BufReader::new(port),
        })
    }
}

impl<R> InputReader for SerialReader<R>
    where R: Read
{
    fn read_next_input(&mut self) -> Result<ControllerState> {
        let mut buf = String::new();

        loop {
            buf.clear();
            self.port.read_line(&mut buf)?;
            let mut reader = StateReader::new(buf.chars());
            if let Some(state) = reader.read_state() {
                return Ok(state);
            }
        }
    }
}

struct StateReader<I> {
    iter: I,
}

impl<I> StateReader<I>
    where I: Iterator<Item = char>,
{
    fn new(iter: I) -> StateReader<I> {
        StateReader {
            iter: iter,
        }
    }

    fn read_bool(&mut self) -> Option<bool> {
        self.iter.next().map(|ch| ch == '1')
    }

    fn read_byte(&mut self) -> Option<u8> {
        let mut value = 0;
        value |= (self.iter.next()? == '1') as u8;
        value <<= 1;
        value |= (self.iter.next()? == '1') as u8;
        value <<= 1;
        value |= (self.iter.next()? == '1') as u8;
        value <<= 1;
        value |= (self.iter.next()? == '1') as u8;
        value <<= 1;
        value |= (self.iter.next()? == '1') as u8;
        value <<= 1;
        value |= (self.iter.next()? == '1') as u8;
        value <<= 1;
        value |= (self.iter.next()? == '1') as u8;
        value <<= 1;
        value |= (self.iter.next()? == '1') as u8;
        Some(value)
    }

    fn read_state(&mut self) -> Option<ControllerState> {
        let mut state = ControllerState::default();
        self.iter.next()?;
        self.iter.next()?;
        self.iter.next()?;
        state.start = self.read_bool()?;
        state.y = self.read_bool()?;
        state.x = self.read_bool()?;
        state.b = self.read_bool()?;
        state.a = self.read_bool()?;
        self.iter.next()?;
        state.l_digital = self.read_bool()?;
        state.r_digital = self.read_bool()?;
        state.z = self.read_bool()?;
        state.up = self.read_bool()?;
        state.down = self.read_bool()?;
        state.right = self.read_bool()?;
        state.left = self.read_bool()?;

        let analog_x = self.read_byte()?;
        let analog_y = self.read_byte()?;
        state.analog = (analog_x, analog_y);


        let c_x = self.read_byte()?;
        let c_y = self.read_byte()?;
        state.c = (c_x, c_y);

        state.l_analog = self.read_byte()?;
        state.r_analog = self.read_byte()?;

        Some(state)
    }
}
