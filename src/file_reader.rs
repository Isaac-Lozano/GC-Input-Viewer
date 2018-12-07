use std::io::{Read, BufRead, BufReader};
use std::fs::File;
use std::thread;

use crate::input_reader::InputReader;
use crate::controller_state::ControllerState;

pub struct FileReader<R> {
    file: BufReader<R>,
}

impl FileReader<File> {
    pub fn from_file(file: File) -> FileReader<File> {
        FileReader {
            file: BufReader::new(file),
        }
    }

    pub fn from_path(path: &str) -> FileReader<File> {
        let file = File::open(path).unwrap();

        FileReader {
            file: BufReader::new(file),
        }
    }
}

impl<R> InputReader for FileReader<R>
    where R: Read
{
    fn read_next_input(&mut self) -> ControllerState {
        thread::sleep_ms(1000 / 60);

        let mut buf = String::new();
        self.file.read_line(&mut buf).unwrap();

        let mut state = ControllerState::default();
        let mut order = [&mut state.a, &mut state.b, &mut state.x, &mut state.y];

        for (ch, button) in buf.chars().zip(order.iter_mut()) {
            **button = ch == '1';
        }

        state
    }
}