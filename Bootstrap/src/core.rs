use ctor::ctor;

use crate::{console, errors::DynErr, hooks, internal_failure, logging::logger};

#[ctor]
fn startup() {
    init().unwrap_or_else(|e| {
        internal_failure!("Failed to initialize MelonLoader: {}", e.to_string());
    })
}

fn init() -> Result<(), DynErr> {
    let current_exe = std::env::current_exe()?;
    let game_name = current_exe
        .file_name()
        .ok_or("Failed to get game name")?
        .to_str()
        .ok_or("Failed to get game name")?;

    if !game_name.starts_with("VRChat") {
        return Ok(());
    }

    console::init()?;
    logger::init()?;

    hooks::init_hook::hook()?;

    console::null_handles()?;

    Ok(())
}

pub fn shutdown() {
    std::process::exit(0);
}
