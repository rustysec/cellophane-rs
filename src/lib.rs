//! Convenience wrappers for various types of FFI pointers that need to be freed eventually.
//!
//! On some platforms, such as windows, there are many different flavors of `free` behavior. Some
//! of these are specific to types of allocators or structures. `cellophane` helps call the
//! appropriate freeing functions by wrapping the pointer in a new type, and freeing on `Drop`.
//!
//! # Example
//!
//! ```no_run
//! extern crate cellophane;
//!
//! use cellophane::{HasPointer, FromPointer, FreeWrapper};
//! #[cfg(not(windows))]
//! pub use std::os::raw::c_void;
//! #[cfg(windows)]
//! pub use winapi::ctypes::c_void;
//!
//! extern "system" {
//!     fn malloc(_: u32) -> *mut c_void;
//!     fn c_lib_needs_a_ptr(_: *const c_void);
//! }
//!
//! let pointer = FreeWrapper::from_ptr(unsafe { malloc(128) });
//!
//! // do something with this ...
//! unsafe {
//!     c_lib_needs_a_ptr(pointer.ptr());
//! }
//!
//! // there's now no need to manually free this pointer, it is automatically cleaned up when `FreeWrapper` goes out of scope
//! // and `Drop` is called.
//! ```

#![warn(missing_docs)]

#[cfg(windows)]
pub mod windows;

pub use cellophane_derive::*;
#[cfg(not(windows))]
use std::os::raw::c_void;
#[cfg(windows)]
use winapi::ctypes::c_void;
#[cfg(windows)]
pub use windows::*;

/// Defines access to raw pointer types.
pub trait HasPointer {
    /// Builds a new wrapper with a null pointer.
    fn new() -> Self;

    /// Returns non-mutable internal pointer.
    fn ptr(&self) -> *const c_void;

    /// Returns non-mutable internal pointer.
    fn mut_ptr(&mut self) -> *mut c_void;

    /// Returns a mutable reference to the internal pointer
    fn mut_ref(&mut self) -> &mut *mut c_void;

    /// Reads type `T` from the pointer
    ///
    /// # Safety
    ///
    /// This function uses unsafe `read`.
    unsafe fn read<T>(&self) -> T;

    /// Reads type `T` from pointer at `offset`
    ///
    /// # Safety
    ///
    /// This function uses unsafe `read`.
    unsafe fn read_offset<T>(&self, offset: isize) -> T;

    /// Reads the `n`th type `T` from the pointer
    ///
    /// # Safety
    ///
    /// This function uses unsafe `read`.
    unsafe fn nth<T>(&self, n: usize) -> T;
}

/// Wrapper can be built from a single simple pointer
pub trait FromPointer {
    /// Builds a new wrapper over an existing pointer.
    fn from_ptr(_: *mut c_void) -> Self;
}

#[cfg(not(all(target_env = "msvc", windows)))]
extern "system" {
    fn free(_: *mut c_void);
}

/// Free object using built in (libc) `free` function.
#[cfg(not(all(target_env = "msvc", windows)))]
#[freeing(free)]
#[derive(FromPointer)]
pub struct FreeWrapper(pub *mut c_void);
