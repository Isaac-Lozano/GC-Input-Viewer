mod input_window;
mod configuration;
mod texture_cache;
mod controller_state;
mod input_reader;

use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;

use crate::input_window::InputWindow;
use crate::configuration::{Configuration, InputSource};
use crate::controller_state::ControllerState;
use crate::input_reader::InputReader;
use crate::input_reader::dtm_reader::DtmReader;
use crate::input_reader::serial_reader::SerialReader;
use crate::input_reader::sa2_reader::Sa2Reader;

fn main() {
    let conf = Configuration::from_path("conf.yaml");
    let theme = conf.theme;
    let base = conf.theme_path;
    let mut reader: Box<dyn InputReader> = match conf.input {
        InputSource::Dtm(path) => Box::new(DtmReader::from_path(&path)),
        InputSource::Sa2(_exe_name) => Box::new(Sa2Reader::new()),
        InputSource::Serial(path) => Box::new(SerialReader::from_path(&path)),
    };

    let state_mutex = Arc::new(Mutex::new(ControllerState::default()));
    let (done_sender, done_receiver) = mpsc::channel::<()>();

    let state_mutex_copy = state_mutex.clone();
    thread::spawn(move || {
        let mut iw = InputWindow::new(&theme, state_mutex_copy).unwrap();
        iw.run(base, theme);
        done_sender.send(()).unwrap();
    });

    loop {
        if done_receiver.try_recv().is_ok() {
            break;
        }
        let new_state = reader.read_next_input();
        let mut state = state_mutex.lock().unwrap();
        *state = new_state;
    }
}