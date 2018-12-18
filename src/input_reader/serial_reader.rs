use std::io::{Read, BufRead, BufReader};
use std::time::Duration;

use serialport::{SerialPort, SerialPortSettings, DataBits, FlowControl, Parity, StopBits};

use crate::input_reader::InputReader;
use crate::controller_state::ControllerState;

pub struct SerialReader<S> {
    port: BufReader<S>,
}

impl SerialReader<Box<dyn SerialPort>> {
    pub fn from_path(path: &str) -> SerialReader<Box<dyn SerialPort>> {
        let s = SerialPortSettings {
            baud_rate: 115200,
            data_bits: DataBits::Eight,
            flow_control: FlowControl::None,
            parity: Parity::None,
            stop_bits: StopBits::One,
            timeout: Duration::from_millis(10),
        };
        let port = serialport::open_with_settings(path, &s).unwrap();

        SerialReader {
            port: BufReader::new(port),
        }
    }
}

fn read_byte<C>(chars: &mut C) -> u8
    where C: Iterator<Item = char>,
{
    let mut value = 0;
    value |= (chars.next().unwrap() == '1') as u8;
    value <<= 1;
    value |= (chars.next().unwrap() == '1') as u8;
    value <<= 1;
    value |= (chars.next().unwrap() == '1') as u8;
    value <<= 1;
    value |= (chars.next().unwrap() == '1') as u8;
    value <<= 1;
    value |= (chars.next().unwrap() == '1') as u8;
    value <<= 1;
    value |= (chars.next().unwrap() == '1') as u8;
    value <<= 1;
    value |= (chars.next().unwrap() == '1') as u8;
    value <<= 1;
    value |= (chars.next().unwrap() == '1') as u8;
    value
}

impl<R> InputReader for SerialReader<R>
    where R: Read
{
    fn read_next_input(&mut self) -> ControllerState {
        let mut buf = String::new();

        loop {
            buf.clear();
            match self.port.read_line(&mut buf) {
                Ok(num_chars) => {
//                    println!("{:?} {}", buf, num_chars);

                    if num_chars == 65 {
                        break;
                    }
                }
                Err(_) => {}
            }
        }

        let mut state = ControllerState::default();
        let mut chars = buf.chars().skip(3);
        state.start = chars.next().unwrap() == '1';
        state.y = chars.next().unwrap() == '1';
        state.x = chars.next().unwrap() == '1';
        state.b = chars.next().unwrap() == '1';
        state.a = chars.next().unwrap() == '1';
        chars.next().unwrap();
        state.l_digital = chars.next().unwrap() == '1';
        state.r_digital = chars.next().unwrap() == '1';
        state.z = chars.next().unwrap() == '1';
        state.up = chars.next().unwrap() == '1';
        state.down = chars.next().unwrap() == '1';
        state.right = chars.next().unwrap() == '1';
        state.left = chars.next().unwrap() == '1';

        let analog_x = read_byte(&mut chars);
        let analog_y = read_byte(&mut chars);
        state.analog = (analog_x, analog_y);


        let c_x = read_byte(&mut chars);
        let c_y = read_byte(&mut chars);
        state.c = (c_x, c_y);

        state.l_analog = read_byte(&mut chars);
        state.r_analog = read_byte(&mut chars);

//        println!("{:?}", state);

        state
    }
}