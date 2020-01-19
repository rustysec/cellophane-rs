cellophane
==========

Convenience wrappers over various types of `free` behavior.

## Overview
Interfacing with other languages via [FFI](https://doc.rust-lang.org/nomicon/ffi.html) is a necessary part of the Rust
language. In some cases, allocations are performed in Rust and are automatically cleaned up via borrow checker rules.
In other cases, allocations happen outside of Rust and must be manually freed. 

Here's a quick example:

```rust
use std::os::raw::c_void;

extern "system" {
    fn malloc(_: u32) -> *mut c_void;
    fn free(_: *mut c_void);
    fn c_lib_needs_a_ptr(_: *const c_void);
}

let pointer = unsafe { malloc(128) };

// do something with this ...
unsafe {
    c_lib_needs_a_ptr(pointer);
}

// then we have to free it, or we have a memory leak
unsafe {
    free(pointer);
}
```

Being able to use a raw pointer in idiomatic Rust is extremely helpful in reducing memory leaks and helping adhere to 
the guarantees of standard Rust code.

This is where `cellophane` comes in handy.

```rust
extern crate cellophane;

use cellophane::{HasPointer, FreeWrapper};
use std::os::raw::c_void;

extern "system" {
    fn malloc(_: u32) -> *mut c_void;
    fn c_lib_needs_a_ptr(_: *const c_void);
}

let pointer = FreeWrapper::from_ptr(unsafe { malloc(128) });

// do something with this ...
unsafe {
    c_lib_needs_a_ptr(pointer.ptr());
}

// there's now no need to manually free this pointer, it is automatically
// cleaned up when `FreeWrapper` goes out of scope and `Drop` is called.
```

## Why `cellophane` exists
On some platforms, there are specific freeing functions used for some structures. One example is `LocalFree`, which safely
frees data allocated by `LocalAlloc`. It is _unsafe_ to free this data with `free` or any other freeing function. Cellophane
simplifies the tracking and freeing of the inner pointer to reduce the work developers need to do. 

## Currently Implemented Wrappers

All Platforms:
* FreeWrapper

Windows Specific:
* CloseHandleWrapper
* FreeSidWrapper
* GlobalFreeWrapper
* LocalFreeWrapper
* LsaFreeReturnBufferWrapper
* NetApiBufferFreeWrapper

## Contributing
If there is a particular allocation that requires a specific freeing function, please open an issue or a PR to add it to this
crate.
