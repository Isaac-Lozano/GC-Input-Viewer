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
#[cfg(windows)]
use crate::input_reader::sa2_reader::Sa2Reader;

const SONIC_ADVENTURE_2_EXE: &'static str = "sonic2App.exe";

fn main() {
    // Print out version info.
    println!("GC Input Viewer by OnVar.");
    println!("Version {}", env!("CARGO_PKG_VERSION"));

    // Read from configuration file.
    let conf = Configuration::from_path("conf.yaml").unwrap_or_barf("Error opening \"conf.yaml\" configuration file");
    let theme = conf.theme;
    let base = conf.theme_path;

    // Take input from whatever input method is specified in the config file.
    let mut reader = get_input(conf.input);

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

#[cfg(windows)]
fn get_input(input: InputSource) -> Box<dyn InputReader> {
    match input {
        InputSource::Dtm(path) => {
            let dtm_reader = DtmReader::from_path(&path).unwrap_or_barf("Could not open dtm file");
            Box::new(dtm_reader)
        }
        InputSource::Sa2(exe_name_opt) => {
            let exe_name = exe_name_opt.unwrap_or(SONIC_ADVENTURE_2_EXE.into());
            let sa2_reader = Sa2Reader::new(exe_name).unwrap_or_barf("Could not open SA2 reader");
            Box::new(sa2_reader)
        }
        InputSource::Serial(path) => {
            let serial_reader = SerialReader::from_path(&path).unwrap_or_barf("Could not open serial port");
            Box::new(serial_reader)
        }
    }
}

#[cfg(not(windows))]
fn get_input(input: InputSource) -> Box<dyn InputReader> {
    match input {
        InputSource::Dtm(path) => {
            let dtm_reader = DtmReader::from_path(&path).unwrap_or_barf("Could not open dtm file");
            Box::new(dtm_reader)
        }
        InputSource::Sa2(exe_name_opt) => {
            barf("Process memory reading only available on Windows")
        }
        InputSource::Serial(path) => {
            let serial_reader = SerialReader::from_path(&path).unwrap_or_barf("Could not open serial port");
            Box::new(serial_reader)
        }
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