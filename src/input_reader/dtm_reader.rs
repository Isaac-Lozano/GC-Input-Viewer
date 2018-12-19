use std::io::BufReader;
use std::fs::File;
use std::thread;
use std::time::{Instant, Duration};

use dtm2txt::decoder::dtm_decoder::DtmDecoder;
use dtm2txt::dtm::Dtm;

use crate::error::Result;
use crate::input_reader::InputReader;
use crate::controller_state::ControllerState;

pub struct DtmReader {
    dtm: Dtm,
    frame: usize,
    playback_start: Option<Instant>,
}

impl DtmReader {
    pub fn from_file(file: File) -> Result<DtmReader> {
        let buf = BufReader::new(file);
        let dtm_decoder = DtmDecoder::new(buf);
        let dtm = dtm_decoder.decode()?;

        Ok(DtmReader {
            dtm: dtm,
            frame: 0,
            playback_start: None,
        })
    }

    pub fn from_path(path: &str) -> Result<DtmReader> {
        let file = File::open(path)?;
        Self::from_file(file)
    }
}

impl InputReader for DtmReader {
    fn read_next_input(&mut self) -> Result<ControllerState> {
        if self.playback_start.is_none() {
            self.playback_start = Some(Instant::now());
        }
        let wait_till = self.playback_start.unwrap() + (Duration::new(1, 0) * self.frame as u32) / 60;
        let now = Instant::now();

        if wait_till > now {
            thread::sleep(wait_till.duration_since(now));
        }

        let mut state = ControllerState::default();
        let current = self.dtm.controller_data[self.frame];

        state.a = current.a;
        state.b = current.b;
        state.x = current.x;
        state.y = current.y;
        state.up = current.up;
        state.down = current.down;
        state.left = current.left;
        state.right = current.right;
        state.start = current.start;
        state.analog = (current.analog_x, current.analog_y);
        state.c = (current.c_x, current.c_y);
        state.l_analog = current.l_pressure;
        state.r_analog = current.r_pressure;
        state.l_digital = current.l;
        state.r_digital = current.r;
        state.z = current.z;

        if self.frame + 1 != self.dtm.controller_data.len() {
            self.frame += 1;
        }

        Ok(state)
    }
}