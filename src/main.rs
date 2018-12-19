mod input_window;
mod configuration;
mod texture_cache;
mod controller_state;
mod input_reader;
mod error;

use std::error::Error;
use std::process;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::thread;

use crate::input_window::InputWindow;
use crate::configuration::{Configuration, InputSource};
use crate::controller_state::ControllerState;
use crate::input_reader::InputReader;
use crate::input_reader::dtm_reader::DtmReader;
use crate::input_reader::serial_reader::SerialReader;
//use crate::input_reader::sa2_reader::Sa2Reader;

fn main() {
    // Read from configuration file.
    let conf = Configuration::from_path("conf.yaml").unwrap_or_barf("Error opening \"conf.yaml\" configuration file");
    let theme = conf.theme;
    let base = conf.theme_path;
    // Take input from whatever input method is specified in the config file.
    let mut reader: Box<dyn InputReader> = match conf.input {
        InputSource::Dtm(path) => {
            let dtm_reader = DtmReader::from_path(&path).unwrap_or_barf("Could not open dtm file");
            Box::new(dtm_reader)
        }
        InputSource::Sa2(_exe_name) => unimplemented!(), //Box::new(Sa2Reader::new()),
        InputSource::Serial(path) => {
            let serial_reader = SerialReader::from_path(&path).unwrap_or_barf("Could not open serial port");
            Box::new(serial_reader)
        }
    };

    // Make a controller state to share across threads.
    let state_mutex = Arc::new(Mutex::new(ControllerState::default()));
    // And a channel to tell us when the display thread has exited.
    let (done_sender, done_receiver) = mpsc::channel::<()>();

    let state_mutex_copy = state_mutex.clone();
    // Start display thread.
    thread::spawn(move || {
        let mut iw = InputWindow::new(&theme, state_mutex_copy).unwrap_or_barf("Could not make window");
        match iw.run(base, theme) {
            Err(e) => println!("Error in display thread: {}", e),
            _ => {}
        }
        // Send done signal when display thread has ended.
        done_sender.send(()).unwrap_or_barf("Could not send done signal");
    });

    // Input-reader loop.
    loop {
        // Check if display thread has ended.
        if done_receiver.try_recv().is_ok() {
            break;
        }
        // Read new input.
        let new_state = reader.read_next_input().unwrap_or_barf("Error reading input");
        // Update mutex.
        let mut state = state_mutex.lock().unwrap_or_barf("Error updating controller state");
        *state = new_state;
    }
}

fn barf(message: &str) -> ! {
    println!("Error: {}", message);
    process::exit(1);
}

trait UnwrapOrBarf<T> {
    fn unwrap_or_barf(self, message: &str) -> T;
}

impl<T, E> UnwrapOrBarf<T> for Result<T, E>
    where E: Error,
{
    fn unwrap_or_barf(self, message: &str) -> T {
        self.unwrap_or_else(|err| {
            barf(&format!("{}: {}", message, err));
        })
    }
}