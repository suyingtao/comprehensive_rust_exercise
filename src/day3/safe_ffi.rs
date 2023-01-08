// TODO: remove this when you're done with your implementation.
#![allow(unused_imports, unused_variables, dead_code)]

use std::fmt;

mod abc {
    use std::os::raw::{c_char, c_int, c_long, c_ulong, c_ushort};

    // Opaque type. See https://doc.rust-lang.org/nomicon/ffi.html.
    #[repr(C)]
    pub struct DIR {
        _data: [u8; 0],
        _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
    }

    // Layout as per readdir(3) and definitions in /usr/include/x86_64-linux-gnu.
    #[repr(C)]
    pub struct dirent {
        pub d_ino: c_long,
        pub d_off: c_ulong,
        pub d_reclen: c_ushort,
        pub d_type: c_char,
        pub d_name: [c_char; 256],
    }

    extern "C" {
        pub fn opendir(s: *const c_char) -> *mut DIR;
        pub fn readdir(s: *mut DIR) -> *const dirent;
        pub fn closedir(s: *mut DIR) -> c_int;
    }
}

use std::ffi::{CStr, CString, OsStr, OsString, c_char};
use std::os::unix::ffi::OsStrExt;

#[derive(Debug)]
struct DirectoryIterator {
    path: CString,
    dir: *mut ffi::DIR,
}

impl DirectoryIterator {
    fn new(path: &str) -> Result<DirectoryIterator, String> {
        // Call opendir and return a Ok value if that worked,
        // otherwise return Err with a message.
        let path = CString::new(path).unwrap();
        let dir = unsafe { ffi::opendir(path.as_ptr()) };
        if dir.is_null() {
            return Err(format!("Path: {path:?} is null"));
        } else {
            return Ok(DirectoryIterator { path, dir });
        }
    }
}
struct MyDirent {
    d_name: String,
    d_type: String,
}


impl fmt::Debug for MyDirent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ name: '{}', type: '{}' }}", self.d_name, self.d_type)
    }
}

impl Iterator for DirectoryIterator {
    type Item = MyDirent;
    fn next(&mut self) -> Option<MyDirent> {
        // Keep calling readdir until we get a NULL pointer back.
        let dirent = unsafe {
            ffi::readdir(self.dir)
        };
        if dirent.is_null() {
            return None;
        };
        let d_name = unsafe { CStr::from_ptr((*dirent).d_name.as_ptr()) };
        let d_type = unsafe { (*dirent).d_type };
        let os_str = OsStr::from_bytes(d_name.to_bytes());
        let mut a = os_str.to_owned();
        a.push(OsString::from(d_type.to_string()));
        let name_str = String::from(d_name.to_str().unwrap());
        let type_str = {
            match d_type {
                4 => "dir",
                8 => "file",
                _ => ""
            }
        };
        let d = MyDirent {d_name: name_str, d_type: type_str.to_string() };
        Some(d)
    }
}

impl Drop for DirectoryIterator {
    fn drop(&mut self) {
        // Call closedir as needed.
        if !self.dir.is_null()  {
            unsafe {
                ffi::closedir(self.dir);
            }
        }
    }
}

fn main() -> Result<(), String> {
    let path = "./src";
    let iter = DirectoryIterator::new(path)?;
    println!("path: {path}");
    println!("files: {:#?}", iter.collect::<Vec<_>>());
    Ok(())
}
