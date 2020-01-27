#![cfg(not(windows))]

use cellophane::{FreeWrapper, FromPointer, HasPointer};
use std::{ffi::CStr, os::raw::c_void};

#[repr(C)]
#[derive(Debug, Clone)]
struct TestStruct {
    first: u32,
    second: u32,
}

#[link(name = "malloc")]
extern "system" {
    fn malloc_string() -> *mut c_void;
    fn malloc_struct(_: *mut *mut TestStruct);
}

#[test]
fn test_malloc_string() {
    let fw = FreeWrapper::from_ptr(unsafe { malloc_string() as *mut _ as _ });
    let s = unsafe { CStr::from_ptr(fw.0 as *const _ as _) };
    let s = s.to_string_lossy().to_string();
    assert_eq!(s, String::from("test 123"));
}

#[test]
fn test_malloc_struct() {
    let mut fw = FreeWrapper::new();
    unsafe {
        malloc_struct(fw.mut_ref() as *mut _ as _);
    }

    let ts: TestStruct = unsafe { fw.read() };
    assert_eq!(ts.first, 1);
    assert_eq!(ts.second, 2);
}
