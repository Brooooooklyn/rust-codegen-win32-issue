use std::ffi::{CStr, CString};

mod ffi {
  use std::os::raw::c_char;

  extern "C" {
    pub fn concat_string(input: *const c_char) -> *mut c_char;
  }
}

fn main() {
  let a = CString::new(" world!").expect("Init CString failed");
  let out = unsafe { ffi::concat_string(a.as_ptr()) };
  let out_str = unsafe { CStr::from_ptr(out) };
  println!("{:?}", out_str);
}
