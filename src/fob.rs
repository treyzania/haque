
use std::mem;
use std::ptr;

use std::fs::File;
use std::os::ext::io::AsRawFd;

use libc::{self, c_void, c_int};

/// This is a neat little type for storing a data structure in a file.  This doesn't work across
/// endiannesses, but it's also easy for setting up structures shared between processes, as long
/// as they're flat structures (aka, they're `Copy`).
///
/// It's automatically unmapped when it's dropped.
///
/// "Filed Object" -> "Fob"
pub struct Fob<T: Copy>(File, *mut T);

pub enum MmapError {
    Null
}

const MAP_PROT: c_int = libc::PROT_READ | libc::PROT_WRITE;
const MAP_FLAGS: c_int = libc::MAP_SHARED;

impl<T> Fob<T> {

    /// Creates a new file-backed object.
    pub fn create(f: File, mut val: T) -> Result<Fob<T>, MmapError> {

        // First we allocate it on the heap.
        let size = mem::size_of::<T>();
        let map = libc::mmap64(0 as *mut c_void, size, MAP_PROT, MAP_FLAGS, f.as_raw_fd(), 0);

        // TODO Pay attention to `errno` stuff.

        if map != (0 as *mut c_void) {

            // Copy the struct into the mapping.
            ptr::copy_nonoverlapping(&mut val, ptr, size);

            // We have to make sure it's not *actually* dropped naturally, as it "still exists" in the new location.
            mem::forget(val);

            Ok(Fob(f, map as *mut T))

        } else {
            Err(MmapError::Null)
        }

    }

}

impl<T> AsRef<T> for Man<T> {
    fn as_ref(&self) -> &T {
        unsafe { mem::transmute::<*mut T, &T>(self.0) }
    }
}

impl<T> AsMut<T> for Man<T> {
    fn as_mut(&mut self) -> &mut T {
        unsafe { mem::transmute::<*mut T, &mut T>(self.0) }
    }
}

impl<T> Drop for Fob<T> {
    fn drop(&mut self) {

        // Make sure to call Drop on it.
        ptr::drop_in_place::<T>(self.1);

        // And then unmap it.
        libc::munmap(self.1, mem::size_of::<T>());

    }
}

// TODO Tests.
