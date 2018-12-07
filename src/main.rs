mod input_window;
mod configuration;
mod texture_cache;
mod controller_state;
mod input_reader;
mod file_reader;
mod dtm_reader;

use std::thread;
use std::sync::{Arc, Mutex};

use crate::input_window::InputWindow;
use crate::configuration::{Configuration, ImageConf, AnalogConf};
use crate::controller_state::ControllerState;
use crate::input_reader::InputReader;
use crate::file_reader::FileReader;
use crate::dtm_reader::DtmReader;

fn main() {
    println!("Hello, world!");

    let conf = Configuration {
        size: (512, 256),
        background: ImageConf {
            path: "resources/background.png".into(),
            dst: (0, 0),
            size: None,
        },
        a: ImageConf {
            path: "resources/a.png".into(),
            dst: (302, 59),
            size: None,
        },
        b: ImageConf {
            path: "resources/b.png".into(),
            dst: (245, 110),
            size: None,
        },
        x: ImageConf {
            path: "resources/x.png".into(),
            dst: (384, 50),
            size: None,
        },
        y: ImageConf {
            path: "resources/y.png".into(),
            dst: (288, 12),
            size: None,
        },
        start: ImageConf {
            path: "resources/start.png".into(),
            dst: (298, 188),
            size: None,
        },
        analog: AnalogConf {
            image: ImageConf {
                path: "resources/analog_marker.png".into(),
                dst: (96, 113),
                size: None,
            },
            range: (96, 96),
        },
        c: AnalogConf {
            image: ImageConf {
                path: "resources/c_marker.png".into(),
                dst: (380, 197),
                size: None,
            },
            range: (34, 34),
        },
        l_analog: ImageConf {
            path: "resources/trigger_a.png".into(),
            dst: (443, 51),
            size: None,
        },
        r_analog: ImageConf {
            path: "resources/trigger_a.png".into(),
            dst: (472, 51),
            size: None,
        },
        l_digital: ImageConf {
            path: "resources/trigger_d.png".into(),
            dst: (443, 220),
            size: None,
        },
        r_digital: ImageConf {
            path: "resources/trigger_d.png".into(),
            dst: (472, 220),
            size: None,
        },
    };

    let state_mutex = Arc::new(Mutex::new(ControllerState::default()));

    let conf_copy = conf.clone();
    let state_mutex_copy = state_mutex.clone();
    let child = thread::spawn(move || {
        let mut iw = InputWindow::new(&conf_copy, state_mutex_copy).unwrap();
        iw.run(conf_copy);
    });

    //let mut reader = DtmReader::from_path("test.dtm");
    //let mut reader = DtmReader::from_path("Mission_Street_m1_in_146.40.dtm");
    //let mut reader = DtmReader::from_path("Mission_Street_m3_in_119.10.dtm");
    let mut reader = DtmReader::from_path("EggQuartersM3_1049_D4.dtm");
    loop {
        let new_state = reader.read_next_input();
        let mut state = state_mutex.lock().unwrap();
        *state = new_state;
    }

    child.join().unwrap();
}