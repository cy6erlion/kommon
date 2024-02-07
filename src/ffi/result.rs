//! Json serialized Rust result intented to be send via FFI

use serde::Serialize;
use std::error::Error;
use std::ffi::c_char;
use std::ffi::CString;

/// Json string to be send via FFI
pub type FFIJSON = *mut c_char;

/// A result send accross FFI
pub struct FFIResult;

impl FFIResult {
    /// Export JSON via FFI
    pub fn json<T: Serialize>(j: T) -> FFIJSON {
        CString::new(serde_json::to_string(&j).unwrap())
            .unwrap()
            .into_raw()
    }

    /// An okay result
    pub fn okay() -> FFIJSON {
        CString::new("{\"ffiOk\":true}").unwrap().into_raw()
    }

    /// A none result
    pub fn none() -> FFIJSON {
        CString::new("{\"ffinone\":true}").unwrap().into_raw()
    }

    /// Export error as a FFI compatible JSON string
    pub fn error<E: Error>(err: E) -> FFIJSON {
        let err_msg = format!("{{\"ffiError\": {err}}}");
        CString::new(err_msg).unwrap().into_raw()
    }
}
