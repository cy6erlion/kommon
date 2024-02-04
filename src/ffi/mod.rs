//! kommon tools for working with FFI
use std::ffi::CStr;
use std::ffi::c_char;

pub use result::{FFIJSON, FFIResult};
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
