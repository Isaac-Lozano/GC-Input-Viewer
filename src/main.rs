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
    println!("Hello, world!");

//    let mut conf_reader = StaticConfig {
//        conf: Configuration {
//            size: (512, 256),
//            background: ImageConf {
//                path: "resources/background.png".into(),
//                dst: (0, 0),
//                size: None,
//            },
//            a: ImageConf {
//                path: "resources/a.png".into(),
//                dst: (302, 59),
//                size: None,
//            },
//            b: ImageConf {
//                path: "resources/b.png".into(),
//                dst: (245, 110),
//                size: None,
//            },
//            x: ImageConf {
//                path: "resources/x.png".into(),
//                dst: (384, 50),
//                size: None,
//            },
//            y: ImageConf {
//                path: "resources/y.png".into(),
//                dst: (288, 12),
//                size: None,
//            },
//            up: ImageConf {
//                path: "resources/up.png".into(),
//                dst: (231, 170),
//                size: None,
//            },
//            down: ImageConf {
//                path: "resources/down.png".into(),
//                dst: (231, 205),
//                size: None,
//            },
//            left: ImageConf {
//                path: "resources/left.png".into(),
//                dst: (208, 193),
//                size: None,
//            },
//            right: ImageConf {
//                path: "resources/right.png".into(),
//                dst: (243, 193),
//                size: None,
//            },
//            start: ImageConf {
//                path: "resources/start.png".into(),
//                dst: (298, 188),
//                size: None,
//            },
//            analog: AnalogConf {
//                image: ImageConf {
//                    path: "resources/analog_marker.png".into(),
//                    dst: (96, 113),
//                    size: None,
//                },
//                range: (96, 96),
//            },
//            c: AnalogConf {
//                image: ImageConf {
//                    path: "resources/c_marker.png".into(),
//                    dst: (380, 197),
//                    size: None,
//                },
//                range: (34, 34),
//            },
//            l_analog: ImageConf {
//                path: "resources/trigger_a.png".into(),
//                dst: (443, 51),
//                size: None,
//            },
//            r_analog: ImageConf {
//                path: "resources/trigger_a.png".into(),
//                dst: (472, 51),
//                size: None,
//            },
//            l_digital: ImageConf {
//                path: "resources/trigger_d.png".into(),
//                dst: (443, 220),
//                size: None,
//            },
//            r_digital: ImageConf {
//                path: "resources/trigger_d.png".into(),
//                dst: (472, 220),
//                size: None,
//            },
//            z: ImageConf {
//                path: "resources/z.png".into(),
//                dst: (442, 17),
//                size: None,
//            },
//        }
//    };

    let mut conf_reader = JsonConfig::from_path("onvar_theme.json");

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