//! Windows specific wrappers
//!

use super::*;
use cellophane_derive::*;
use winapi::{
    ctypes::c_void,
    um::{
        handleapi::CloseHandle,
        lmapibuf::NetApiBufferFree,
        ntlsa::LsaFreeReturnBuffer,
        securitybaseapi::FreeSid,
        winbase::{GlobalFree, LocalFree},
    },
};

/// Frees the specified local memory object and invalidates its handle.
///
/// A handle to the local memory object. This handle is returned by either the LocalAlloc or LocalReAlloc function. It is not safe to free memory allocated with GlobalAlloc.
///
#[freeing(LocalFree)]
pub struct LocalFreeWrapper(*mut c_void);

/// Frees the specified global memory object and invalidates its handle.
///
/// A handle to the global memory object. This handle is returned by either the GlobalAlloc or GlobalReAlloc function. It is not safe to free memory allocated with LocalAlloc.
#[freeing(GlobalFree)]
pub struct GlobalFreeWrapper(*mut c_void);

/// The LsaFreeReturnBuffer function frees the memory used by a buffer previously allocated by the LSA.
#[freeing(LsaFreeReturnBuffer)]
pub struct LsaFreeReturnBufferWrapper(*mut c_void);

/// The NetApiBufferFree function frees the memory that the NetApiBufferAllocate function allocates. Applications should also call NetApiBufferFree to free the memory that other network management functions use internally to return information.
#[freeing(NetApiBufferFree)]
pub struct NetApiBufferFreeWrapper(*mut c_void);

/// Closes an open object handle.
#[freeing(CloseHandle)]
pub struct CloseHandleWrapper(*mut c_void);

/// The FreeSid function frees a security identifier (SID) previously allocated by using the AllocateAndInitializeSid function.
#[freeing(FreeSid)]
pub struct FreeSidWrapper(*mut c_void);
