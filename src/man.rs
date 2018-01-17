
use std::mem;
use std::ptr;

use libc::{self, c_void};

/// Manually-managed pointer type.  You must manually call `.free()` on these values somewhere if
/// you don't want to screw yourself later on.
///
/// Since this uses the libc `malloc` function, it can be passed into C code and freed with the
/// `free` function there if you need to.  And vice-versa.  Although usually this only works well
/// with simpler structs and primitive arrays.
///
/// **WARNING:** Everything to do with this type is incredibly unsafe.  You're better off using
/// something else in nearly every case.
#[derive(Copy, Eq, PartialEq)]
pub struct Man<T>(*mut T);

impl<T> Man<T> {

    /// Creates a new manually-managed pointer instance.
    pub unsafe fn new(mut val: T) -> Man<T> {

        // First we allocate it on the heap.
        let (size, ptr) = typed_malloc::<T>();
        ptr::copy_nonoverlapping(&mut val, ptr, size);

        // We have to make sure it's not *actually* dropped naturally, as it "still exists" in the new location.
        mem::forget(val);

        // Then just return the constructed pointer.
        Man(ptr as *mut T)

    }

    /// Creates a `Man<T>` from a raw pointer.
    pub unsafe fn from_raw(p: *const T) -> Man<T> {
        Man(p as *mut T)
    }

    /// Frees the underlying data.  This doesn't do anything to prevent double-frees.
    pub unsafe fn free(self) {

        // Drop the value.
        ptr::drop_in_place(self.0);

        // Now we can free the memory we requested.
        libc::free(self.0 as *mut c_void);

    }

}

impl<T> Clone for Man<T> where T: Clone {
    fn clone(&self) -> Self {
        unsafe { Man::new(self.as_ref().clone()) }
    }
}

/// Allocates space for the type on the heap, returning a tuple of the bytes allocated and a pointer to the heap range.
unsafe fn typed_malloc<T>() -> (usize, *mut T) {
    let size = mem::size_of::<T>();
    match libc::malloc(size) as usize {
        0 => panic!("malloc failed on size {}", size),
        n @ _ => (size, n as *mut T)
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

#[cfg(test)]
mod tests {

    use super::Man;

    #[test]
    pub fn test_man_dereference() {
        unsafe {
            let p = Man::new(42);
            assert_eq!(*p.as_ref(), 42);
            p.free()
        }
    }

    #[test]
    pub fn test_man_string() {
        unsafe {
            let p = Man::new(String::from("foobar"));
            assert_eq!(p.as_ref().as_str(), "foobar");
            p.free()
        }
    }

    #[test]
    pub fn test_man_big_drop() {
        const KILOBYTE: usize = 1024;
        unsafe {

            /*
             * End up allocating hopefully around 64 gigabytes.  If we don't free it right then
             * it'll overflow the heap on most machines and crash.
             */
            for _ in 0..(64 * 1024 * 1024) {
                let p = Man::new(Box::new([0u8; KILOBYTE])); // the bigger data is stored elsewhere with this
                //p.free();
            }

        }
    }

}
