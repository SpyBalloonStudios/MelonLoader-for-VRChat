use ctor::ctor;
use std::{thread, time::Duration, path::PathBuf};

use crate::{console, errors::DynErr, hooks, internal_failure, logging::logger};

#[ctor]
fn startup() {
     if !is_vrchat_process() {
        return;
    }
    thread::sleep(Duration::from_secs(10));
    
    init().unwrap_or_else(|e| {
        internal_failure!("Failed to initialize MelonLoader: {}", e.to_string());
    })
}

fn init() -> Result<(), DynErr> {
    console::init()?;
    logger::init()?;

    hooks::init_hook::hook()?;

    console::null_handles()?;

    Ok(())
}

fn is_vrchat_process() -> bool {
    std::env::current_exe()
        .ok()
        .and_then(|path| path.file_name().map(|name| name.to_string_lossy().starts_with("VRChat")))
        .unwrap_or(false)
}

pub fn shutdown() {
    std::process::exit(0);
}
