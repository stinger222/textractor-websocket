use std::ffi::CStr;
use widestring::U16CString;
use windows_sys::Win32::{
    Foundation::{BOOL, HINSTANCE, TRUE},
    System::SystemServices::DLL_PROCESS_ATTACH,
};
use std::ffi::c_ulong;
use std::ffi::c_void;
mod textractor_ws;

// windows-rs does not define DWORD and LPVOID
// https://github.com/microsoft/windows-rs/issues/881
#[allow(clippy::upper_case_acronyms)]
type DWORD = c_ulong;
#[allow(clippy::upper_case_acronyms)]
type LPVOID = *mut c_void;

fn get_property(info_array: *const InfoForExtension, property_name: &str) -> i64 {
    let mut p = info_array;
    while !p.is_null() {
        unsafe {
            let p_name = (*p).name;
            let cs1 = CStr::from_ptr(p_name).to_str().unwrap();
            let cs2 = property_name;
            if cs1 == cs2 {
                let v = (*p).value;
                return v;
            }
            p = p.add(1);
        }
    }
    println!("Could not find property {}", property_name);
    panic!("Could not find property");
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct InfoForExtension {
    pub name: *mut ::std::os::raw::c_char,
    pub value: i64,
}

#[no_mangle]
pub extern "C" fn OnNewSentence(
    sentence: *const u16,
    sentence_info: *const InfoForExtension,
) -> *const u16 {
    let u16_str: U16CString;
    unsafe {
        u16_str = U16CString::from_ptr_str(sentence);
    }

    let safe_s = u16_str.to_string_lossy();
    let current_select = get_property(sentence_info, "current select");
    let text_number = get_property(sentence_info, "text number");
    if current_select != 0 && text_number > 1 {
        textractor_ws::handle(safe_s);
    }
    sentence
}

#[no_mangle]
pub extern "system" fn DllMain(
    _h_module: HINSTANCE,
    fdw_reason: DWORD,
    _lpv_reserved: LPVOID,
) -> BOOL {
    if fdw_reason == DLL_PROCESS_ATTACH {
        textractor_ws::start_server();
    }

    TRUE
}
