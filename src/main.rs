mod input_window;
mod configuration;
mod texture_cache;
mod controller_state;
mod input_reader;

use std::thread;
use std::sync::{Arc, Mutex};

use crate::input_window::InputWindow;
use crate::configuration::ConfigReader;
use crate::configuration::json_config::JsonConfig;
use crate::controller_state::ControllerState;
use crate::input_reader::InputReader;
use crate::input_reader::dtm_reader::DtmReader;

fn main() {
    let mut conf_reader = JsonConfig::from_path("skins/onvar_theme/onvar_theme.json");

    let conf = conf_reader.read_config();
    let base = conf_reader.get_path_base();

    let state_mutex = Arc::new(Mutex::new(ControllerState::default()));

    let conf_copy = conf.clone();
    let state_mutex_copy = state_mutex.clone();
    let child = thread::spawn(move || {
        let mut iw = InputWindow::new(&conf_copy, state_mutex_copy).unwrap();
        iw.run(base, conf_copy);
    });

    //let mut reader = DtmReader::from_path("test.dtm");
    //let mut reader = DtmReader::from_path("Mission_Street_m1_in_146.40.dtm");
    //let mut reader = DtmReader::from_path("Mission_Street_m3_in_119.10.dtm");
    let mut reader = DtmReader::from_path("/home/onvar/Documents/sa2/tas/EggQuartersM3_1049_D4.dtm");
    //let mut reader = SerialReader::from_path("COM11");
    loop {
        let new_state = reader.read_next_input();
        let mut state = state_mutex.lock().unwrap();
        *state = new_state;
    }

    child.join().unwrap();
}