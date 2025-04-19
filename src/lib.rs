use std::ffi::{CStr, c_void};
use widestring::U16CString;
use windows_sys::Win32::{
    Foundation::{BOOL, HINSTANCE, TRUE},
    System::SystemServices::DLL_PROCESS_ATTACH,
};
mod textractor_ws;

// windows-rs does not define DWORD and LPVOID
// https://github.com/microsoft/windows-rs/issues/881
#[allow(clippy::upper_case_acronyms)]
type DWORD = u32;
#[allow(clippy::upper_case_acronyms)]
type LPVOID = *mut c_void;

fn get_property(info_array: *const InfoForExtension, property_name: &str) -> i64 {
    let mut p = info_array;
    unsafe {
        while !p.is_null() {
            let current = &*p;
            if current.name.is_null() {
                break;
            }
            let cs1 = CStr::from_ptr(current.name).to_str().unwrap();
            if cs1 == property_name {
                return current.value;
            }
            p = p.add(1);
        }
    }
    0
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
    if sentence.is_null() {
        return sentence;
    }

    let u16_str = unsafe { U16CString::from_ptr_str(sentence) };
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
