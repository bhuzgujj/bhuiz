use std::fmt::{Display, Formatter, Pointer};

pub mod graphics;
pub mod controller;

pub fn cstr_ptr(s: &str) -> *const std::os::raw::c_char {
	std::ffi::CString::new(s).unwrap().into_raw()
}

pub fn cstring_ptr(s: &String) -> *const std::os::raw::c_char {
	std::ffi::CString::new(s.as_str()).unwrap().into_raw()
}

pub fn printable_cstr_ptr(s: *const ::std::os::raw::c_char) -> String {
	if s.is_null() {
		String::new()
	} else {
		format!("{}", Printer { s })
	}
}

struct Printer{
	s: *const ::std::os::raw::c_char
}

impl Display for Printer {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		self.s.fmt(f)
	}
}