mod process_reader;

use std::thread;
use std::time::Duration;

use crate::controller_state::ControllerState;
use crate::error::Result;
use crate::input_reader::InputReader;
use crate::input_reader::sa2_reader::process_reader::ProcessHandle;

const SONIC_2_APP_EXE: &'static str = "sonic2app.exe";
const BUTTON_ADDR: u64 = 0x0000000001A52C4C;
const JOY_X_ADDR: u64 = 0x0000000001A52C50;
const JOY_Y_ADDR: u64 = 0x0000000001A52C54;

pub struct Sa2Reader {
    phandle: Option<ProcessHandle>,
    exe_name: Option<String>,
}

impl Sa2Reader {
    pub fn new(exe_name: Option<String>) -> Result<Sa2Reader> {
        let phandle = ProcessHandle::from_name_filter(|pname| {
            if let Some(ref name) = exe_name {
                pname == *name
            }
            else {
                pname.to_lowercase() == SONIC_2_APP_EXE
            }
        })?;

        Ok(Sa2Reader {
            phandle: phandle,
            exe_name: exe_name,
        })
    }
}

impl InputReader for Sa2Reader {
    fn read_next_input(&mut self) -> Result<ControllerState> {
        while self.phandle.is_none() {
            self.phandle = ProcessHandle::from_name_filter(|pname| {
                if let Some(ref name) = self.exe_name {
                    pname == *name
                }
                else {
                    pname.to_lowercase() == SONIC_2_APP_EXE
                }
            })?;
            thread::sleep(Duration::from_secs(1));
        }

        thread::sleep(Duration::from_micros(1000000 / 120));
        let phandle = self.phandle.as_ref().unwrap();

        let buttons = phandle.read_u32(BUTTON_ADDR)?;
        let joy_x = phandle.read_i32(JOY_X_ADDR)? + 0x80;
        let joy_y = phandle.read_i32(JOY_Y_ADDR)? + 0x80;

        let mut controller_state = ControllerState::default();
        controller_state.left = buttons & 0x0001 != 0;
        controller_state.right = buttons & 0x0002 != 0;
        controller_state.down = buttons & 0x0004 != 0;
        controller_state.up = buttons & 0x0008 != 0;
        controller_state.r_digital = buttons & 0x0020 != 0;
        controller_state.l_digital = buttons & 0x0040 != 0;
        controller_state.a = buttons & 0x0100 != 0;
        controller_state.b = buttons & 0x0200 != 0;
        controller_state.x = buttons & 0x0400 != 0;
        controller_state.y = buttons & 0x0800 != 0;
        controller_state.start = buttons & 0x1000 != 0;
        controller_state.analog = (joy_x as u8, joy_y as u8);

        Ok(controller_state)
    }
}
