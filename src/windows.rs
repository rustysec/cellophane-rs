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
#[derive(FromPointer)]
pub struct LocalFreeWrapper(pub *mut c_void);

/// Frees the specified global memory object and invalidates its handle.
///
/// A handle to the global memory object. This handle is returned by either the GlobalAlloc or GlobalReAlloc function. It is not safe to free memory allocated with LocalAlloc.
#[freeing(GlobalFree)]
#[derive(FromPointer)]
pub struct GlobalFreeWrapper(pub *mut c_void);

/// The LsaFreeReturnBuffer function frees the memory used by a buffer previously allocated by the LSA.
#[freeing(LsaFreeReturnBuffer)]
#[derive(FromPointer)]
pub struct LsaFreeReturnBufferWrapper(pub *mut c_void);

/// The NetApiBufferFree function frees the memory that the NetApiBufferAllocate function allocates. Applications should also call NetApiBufferFree to free the memory that other network management functions use internally to return information.
#[freeing(NetApiBufferFree)]
#[derive(FromPointer)]
pub struct NetApiBufferFreeWrapper(pub *mut c_void);

/// Closes an open object handle.
#[freeing(CloseHandle)]
#[derive(FromPointer)]
pub struct CloseHandleWrapper(pub *mut c_void);

/// The FreeSid function frees a security identifier (SID) previously allocated by using the AllocateAndInitializeSid function.
#[freeing(FreeSid)]
#[derive(FromPointer)]
pub struct FreeSidWrapper(pub *mut c_void);

#[link(name = "wintrust")]
extern "system" {
    fn CryptCATAdminReleaseContext(h_cat_admin: *mut c_void, dw_flags: u32) -> u32;

    fn CryptCATAdminReleaseCatalogContext(
        h_cat_admin: *mut c_void,
        h_cat_info: *mut c_void,
        dw_flags: u32,
    ) -> u32;
}

/// The CryptCATAdminReleaseContext function releases the handle previously assigned by the CryptCATAdminAcquireContext function. This function has no associated import library. You must use the LoadLibrary and GetProcAddress functions to dynamically link to Wintrust.dll.
#[derive(FromPointer, HasPointer)]
pub struct CryptCATAdminReleaseContextWrapper(pub *mut c_void);

impl Drop for CryptCATAdminReleaseContextWrapper {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe {
                CryptCATAdminReleaseContext(self.0, 0);
            }
        }
    }
}

/// The CryptCATAdminReleaseCatalogContext function releases a handle to a catalog context previously returned by the CryptCATAdminAddCatalog function. This function has no associated import library. You must use the LoadLibrary and GetProcAddress functions to dynamically link to Wintrust.dll.
pub struct CryptCATAdminReleaseCatalogContextWrapper<'ctx>(
    *mut c_void,
    &'ctx CryptCATAdminReleaseContextWrapper,
);

impl<'ctx> CryptCATAdminReleaseCatalogContextWrapper<'ctx> {
    /// Builds a new wrapper with a null pointer.
    pub fn new(_: *mut c_void, admin: &'ctx CryptCATAdminReleaseContextWrapper) -> Self {
        CryptCATAdminReleaseCatalogContextWrapper(std::ptr::null_mut(), admin)
    }

    /// Returns a non-mutable reference to the internal pointer.
    pub fn ptr(&self) -> *const c_void {
        self.0
    }

    /// Reads type `T` from the pointer
    ///
    /// # Safety
    ///
    /// This function uses unsafe `read`.
    pub unsafe fn read<T>(&self) -> T {
        std::ptr::read(self.0 as *const _)
    }

    /// Reads type `T` from pointer at `offset`
    ///
    /// # Safety
    ///
    /// This function uses unsafe `read`.
    pub unsafe fn read_offset<T>(&self, offset: isize) -> T {
        std::ptr::read(self.0.offset(offset) as *const _)
    }

    /// Reads the `n`th type `T` from the pointer
    ///
    /// # Safety
    ///
    /// This function uses unsafe `read`.
    pub unsafe fn nth<T>(&self, n: usize) -> T {
        std::ptr::read(
            self.0
                .offset(n as isize * std::mem::size_of::<T>() as isize) as *const _,
        )
    }
}

impl<'ctx> Drop for CryptCATAdminReleaseCatalogContextWrapper<'ctx> {
    fn drop(&mut self) {
        unsafe {
            CryptCATAdminReleaseCatalogContext(self.1.ptr() as _, self.0, 0);
        }
    }
}

#[allow(missing_docs)]
#[link(name = "crypt32")]
extern "system" {
    pub fn CertFreeCertificateContext(context: *const c_void) -> u32;

    pub fn CertCloseStore(cert_store: *const c_void, flags: u32) -> u32;

    pub fn CryptMsgClose(crypt_msg: *const c_void) -> u32;
}

/// The CertFreeCertificateContext function frees a certificate context by decrementing its reference count. When the reference count goes to zero, CertFreeCertificateContext frees the memory used by a certificate context.
#[freeing(CertFreeCertificateContext)]
#[derive(FromPointer)]
pub struct CertFreeCertificateContextWrapper(pub *mut c_void);

/// The CryptMsgClose function closes a cryptographic message handle. At each call to this function, the reference count on the message is reduced by one. When the reference count reaches zero, the message is fully released.
#[freeing(CryptMsgClose)]
#[derive(FromPointer)]
pub struct CryptMsgCloseWrapper(pub *mut c_void);

/// The CertCloseStore function closes a certificate store handle and reduces the reference count on the store. There needs to be a corresponding call to CertCloseStore for each successful call to the CertOpenStore or CertDuplicateStore functions.
#[derive(FromPointer, HasPointer)]
pub struct CertCloseStoreWrapper(pub *mut c_void);

impl Drop for CertCloseStoreWrapper {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe {
                CertCloseStore(self.0, 0);
            }
        }
    }
}
