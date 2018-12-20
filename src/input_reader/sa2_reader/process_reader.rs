use std::mem;
use std::ffi::CStr;
use std::vec::IntoIter;

use winapi::ctypes::c_void;
use winapi::shared::minwindef::{HMODULE, MAX_PATH};
use winapi::shared::ntdef::NULL;
use winapi::um::memoryapi;
use winapi::um::processthreadsapi;
use winapi::um::psapi;
use winapi::um::winnt::{HANDLE, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ};

use crate::error::{Error, Result};

const PROCESS_BUFFER_LEN: usize = 1024;

#[derive(Clone,Copy,Debug)]
pub struct ProcessHandle(HANDLE);

impl ProcessHandle {
    fn open_process(id: ProcessId, mode: u32) -> Result<ProcessHandle> {
        let handle;
        unsafe {
            handle = processthreadsapi::OpenProcess(mode, false as i32, id.0);
            if handle == NULL {
                return Err(Error::ProcessError("could not open process"));
            }
        }
        Ok(ProcessHandle(handle))
    }

    pub fn open_process_read_info(id: ProcessId) -> Result<ProcessHandle> {
        Self::open_process(id, PROCESS_QUERY_INFORMATION | PROCESS_VM_READ)
    }

    pub fn get_name(&self) -> Result<String> {
        let name;
        unsafe {
            let mut module = mem::uninitialized();
            let mut bytes_needed = mem::uninitialized();
            let result = psapi::EnumProcessModules(self.0, &mut module as *mut HMODULE, mem::size_of::<HMODULE>() as u32, &mut bytes_needed as *mut u32);
            if result == 0 {
                Err(Error::ProcessError("error in EnumProcessModules"));
            }
            let mut name_buffer = [0i8; MAX_PATH];
            let bytes_in_str = psapi::GetModuleBaseNameA(self.0, module, &mut name_buffer[0] as *mut i8, MAX_PATH as u32);
            let name_buffer: [u8; MAX_PATH] = mem::transmute(name_buffer);
            name = CStr::from_bytes_with_nul(&name_buffer[.. bytes_in_str as usize + 1])
                .map_err(|_| Error::ProcessError("error converting process name"))?
                .to_str()
                .map_err(|_| Error::ProcessError("error converting process name"))?
                .to_string();
        }
        Ok(name)
    }

    pub fn from_name(name: &str) -> Result<Option<ProcessHandle>> {
        for pid in ProcessIterator::new()? {
            if let Ok(process_name) = ProcessHandle::open_process_read_info(pid).and_then(|h| h.get_name()) {
                if &process_name == name {
                    return Ok(Some(handle))
                }
            }
        }
        Ok(None)
    }

    pub fn read_data(&self, address: u64, buf: &mut [u8]) -> usize {
        let mut bytes_read;
        unsafe {
            bytes_read = mem::uninitialized();
            let address = mem::transmute(address);
            let buf_addr = buf.as_mut_ptr() as *mut c_void;
            let result = memoryapi::ReadProcessMemory(self.0, address, buf_addr, buf.len(), &mut bytes_read as *mut usize);
            if result == 0 {
                panic!("Error in ReadProcessMemory");
            }
        }
        bytes_read
    }

    pub fn read_i32(&self, address: u64) -> i32 {
        let mut buf = [0; 4];
        let bytes_read = self.read_data(address, &mut buf);
        if bytes_read != 4 {
            panic!("Not enough bytes read");
        }
        let mut value = 0;
        value |= buf[3] as i32;
        value <<= 8;
        value |= buf[2] as i32;
        value <<= 8;
        value |= buf[1] as i32;
        value <<= 8;
        value |= buf[0] as i32;
        value
    }

    pub fn read_u32(&self, address: u64) -> u32 {
        let mut buf = [0; 4];
        let bytes_read = self.read_data(address, &mut buf);
        if bytes_read != 4 {
            panic!("Not enough bytes read");
        }
        let mut value = 0;
        value |= buf[3] as u32;
        value <<= 8;
        value |= buf[2] as u32;
        value <<= 8;
        value |= buf[1] as u32;
        value <<= 8;
        value |= buf[0] as u32;
        value
    }
}

#[derive(Clone,Copy,Debug)]
pub struct ProcessId(u32);

#[derive(Clone,Debug)]
pub struct ProcessIterator {
    iter: IntoIter<u32>,
}

impl ProcessIterator {
    pub fn new() -> ProcessIterator {
        let mut buffer = vec![0; PROCESS_BUFFER_LEN];

        unsafe {
            let buf_ptr = buffer.as_mut_ptr();
            let mut returned_bytes = 0u32;
            let result = psapi::EnumProcesses(buf_ptr, (PROCESS_BUFFER_LEN * mem::size_of::<u32>()) as u32, &mut returned_bytes as *mut u32);
            if result == 0 {
                panic!("Error in EnumProcess");
            }
            buffer.set_len(returned_bytes as usize / mem::size_of::<u32>());
        }

        ProcessIterator {
            iter: buffer.into_iter(),
        }
    }
}

impl Iterator for ProcessIterator {
    type Item = ProcessId;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(ProcessId)
    }
}
