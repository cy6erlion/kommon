//! kommon tools for working with FFI
use std::ffi::c_char;
use std::ffi::CStr;

pub use result::{FFIResult, FFIJSON};
mod result;

/// Convert a *const c_char to a rust String
pub unsafe fn as_rust_str(c: *const c_char) -> String {
    let cs = unsafe {
        assert!(!c.is_null());
        CStr::from_ptr(c)
    };
    let s = cs.to_str().unwrap();
    s.to_string()
}
