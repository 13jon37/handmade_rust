use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

use winapi::shared::basetsd::LONG_PTR;
use winapi::shared::minwindef::DWORD;
use winapi::shared::ntdef::HANDLE;

pub(crate) const INVALID_HANDLE_VALUE: HANDLE = (-1 as LONG_PTR) as HANDLE;
pub(crate) const OPEN_EXISTING: DWORD = 3;

// Wide char array C/C++ ex. == WCHAR *string = L"String";
pub fn create_wide_char(string: &str) -> Vec<u16> {
    let mut result: Vec<u16> = OsStr::new(string).encode_wide().collect();
    result.push(0); // add null terminator

    result
} // using function with .as_ptr() method ex. create_wide_char("Monke Game").as_ptr();
