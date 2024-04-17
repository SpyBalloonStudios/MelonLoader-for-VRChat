use std::ptr::null;

use std::{thread};

use ctor::ctor;

use crate::{console, errors::DynErr, hooks, internal_failure, logging::logger};

#[ctor]
fn startup() {
    let current_exe = std::env::current_exe().expect("Failed to get current executable");
    let game_name = current_exe
        .file_name()
        .expect("Failed to get file name from current executable")
        .to_str()
        .expect("Failed to convert file name to string");


    if !game_name.to_lowercase().starts_with("vrchat") {
        return; 
    }
    console::init().expect("Failed to get current executable");
  
    init().unwrap_or_else(|e| {
        internal_failure!("Failed to initialize MelonLoader: {}", e.to_string());
    });
    
}

fn init() -> Result<(), DynErr> {
    console::init()?;

    logger::init()?;

        // Installiere einen Hook. Auch hier verwenden wir `unwrap`.
    hooks::init_hook::hook()?;

        // Null die Handles zur Konsole. Und wieder verwenden wir `unwrap`.
        console::null_handles()?;

    Ok(())
}

pub fn shutdown() {
    std::process::exit(0);
}
