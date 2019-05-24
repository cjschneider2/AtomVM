use std::ffi::CStr;
use std::fs::{OpenOptions, File};
use std::io::Seek;
use std::ptr;
use std::os::unix::io::AsRawFd;
use std::path::PathBuf;
use std::os::raw::c_char;

#[repr(C)]
pub struct MappedFile {
    fd: File,
    mapped: *mut libc::c_void,
    size: u64,
}

#[no_mangle]
pub extern "C" fn mapped_file_open_beam(file_name: *const c_char) -> *mut MappedFile {

    let c_str = unsafe {
        CStr::from_ptr(file_name)
            .to_str()
            .expect("can't convert CStr to path")
    };

    let mut path = PathBuf::new();
    path.push(c_str);

    let mut mf = Box::new({
        let mut fd = OpenOptions::new()
            .read(true)
            .write(false)
            .create(false)
            .open(&path)
            .expect("Unable to open file");

        let size = fd.metadata().expect("Couldn't fet file size").len();

        let mapped = unsafe {
            libc::mmap(
                ptr::null_mut(),  // addr
                size as usize,    // size
                libc::PROT_READ,  // protections
                libc::MAP_SHARED, // flags
                fd.as_raw_fd(),   // file descriptor
                0,                // offset
            )
        };

        MappedFile { fd, size, mapped }
    });

    Box::into_raw(mf)
}

#[no_mangle]
pub extern "C" fn mapped_file_close(mf: *const MappedFile) {
    unsafe {
        libc::munmap((*mf).mapped, (*mf).size as usize);
    }
}
