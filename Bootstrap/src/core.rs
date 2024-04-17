use std::{thread, ptr::null_mut};
use std::time::Duration;
use winapi::um::libloaderapi::DisableThreadLibraryCalls;
use winapi::um::winuser::{FindWindowW};
use winapi::shared::minwindef::{DWORD, HMODULE, BOOL, TRUE, LPVOID};
use ctor::ctor;
use std::sync::Once;
use crate::{console, errors::DynErr, hooks, internal_failure, logging::logger};

static INIT: Once = Once::new();

#[ctor]
fn startup() {
    INIT.call_once(|| {
        thread::spawn(|| {
            // Prüfe, ob der aktuelle Prozess VRChat ist
            if !is_vrchat_running() {
                return;
            }

            if let Err(e) = init() {
                eprintln!("Failed to initialize: {}", e);
            }
        });
    });
}

fn is_vrchat_running() -> bool {
    loop {
        let window = unsafe { FindWindowW(null_mut(), wide_string("VRChat").as_ptr()) };
        if window.is_null() {
            thread::sleep(Duration::from_secs(1));
        } else {
            return true;
        }
    }
}

fn init() -> Result<(), &'static str> {
    console::init();
    logger::init();
    hooks::init_hook::hook();
    console::null_handles();

    Ok(())
}

fn wide_string(s: &str) -> Vec<u16> {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    OsStr::new(s).encode_wide().chain(std::iter::once(0)).collect()
}

#[no_mangle]
pub extern "system" fn DllMain(dll_module: HMODULE, call_reason: DWORD, _: LPVOID) -> BOOL {
    unsafe { DisableThreadLibraryCalls(dll_module); }
    if call_reason == 1 { // DLL_PROCESS_ATTACH
        startup();
    }
    TRUE
}


pub fn shutdown() {
    std::process::exit(0);
}