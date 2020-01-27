#![cfg(windows)]

use cellophane::*;
use std::ptr;
use winapi::um::ntlsa::LsaEnumerateLogonSessions;

#[cfg(windows)]
#[test]
fn lsa_free_return_buffer() {
    let mut logon_sessions = LsaFreeReturnBufferWrapper::from_ptr(ptr::null_mut());
    let mut logon_session_count: u32 = 0;

    if {
        unsafe {
            LsaEnumerateLogonSessions(
                &mut logon_session_count,
                &mut logon_sessions.0 as *mut _ as _,
            )
        }
    } != 0
    {
        assert!(false);
    }

    assert!(true)
}
