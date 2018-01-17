
use std::mem;

use libc::{self, c_void};

/// Manually-managed pointer type.  You must manually call `.free()` on these values somewhere if
/// you don't want to screw yourself later on.
///
/// **WARNING:** Everything to do with this type is incredibly unsafe.  You're better off using
/// something else in nearly every case.
#[derive(Copy, Eq, PartialEq)]
pub struct Man<T>(*mut T);

impl<T> Man<T> {

    /// Creates a new manually-managed pointer instance.
    pub unsafe fn new(val: T) -> Man<T> {

        let (size, ptr) = typed_malloc::<T>();
        libc::memcpy(ptr as *mut c_void, &val as *const T as *const c_void, size);
        Man(ptr as *mut T)

    }

    /// Frees the pointer.  This consumes it, but other references to the underlying data
    /// (identical to this one) may still exist.  If they do and you try to use them, it *will*
    /// cause undefined behavior.
    pub unsafe fn free(self) {

        // Do some horrible hacks to drop the contained value.
        // FIXME This causes segfaults.
        let buf = mem::uninitialized::<T>(); // What's horrible is that this doesn't need to be mut.
        libc::memcpy(&buf as *const T as *mut c_void, self.0 as *const c_void, mem::size_of::<T>());
        mem::drop(buf);

        // Actually free it.
        libc::free(self.0 as *mut c_void)

    }

    pub fn as_raw(&self) -> *const T {
        self.0 as *const T
    }

    pub fn as_mut_raw(&self) -> *mut T {
        self.0
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

#[cfg(test)]
mod tests {

    use super::Man;

    #[test]
    pub fn test_string() {
        unsafe {
            let p = Man::new(String::from("foobar"));
            assert_eq!(p.as_ref().as_str(), "foobar");
            p.free()
        }
    }

    #[test]
    pub fn test_dereference() {
        unsafe {
            let p = Man::new(42);
            assert_eq!(*p.as_ref(), 42);
            p.free()
        }
    }

}
