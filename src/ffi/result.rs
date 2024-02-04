//! Json serialized Rust result intented to be send via FFI

use serde::Serialize;
use std::ffi::c_char;
use std::ffi::CString;
use std::error::Error;

/// Json string to be send via FFI
pub type FFIJSON = *mut c_char;

/// A result send accross FFI
pub enum FFIResult<T: Serialize, E: Error>{
    /// Successfull result with a JSON payload
    Json(T),
    /// Successfull result with no payload
    Okay,
    /// Error result with the error included
    Error(E)
}

impl <T,E> FFIResult<T, E>
where
    T: Serialize,
    E: Error{
    /// Export result as a FFI compatible JSON string
    pub fn export(&self) -> FFIJSON{
	match self{
	    Self::Json(j) => {
		CString::new(serde_json::to_string(&j).unwrap())
		    .unwrap()
		    .into_raw()
	    },
	    Self::Okay => CString::new("{\"ffiOk\":true}").unwrap().into_raw(),
	    Self::Error(err) => {
		let err_msg = format!("{{\"ffierror\": {err}}}");
		CString::new(err_msg).unwrap().into_raw()
	    }
	}
    }
}
