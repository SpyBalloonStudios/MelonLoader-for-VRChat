use std::ptr::null;

use std::{thread};

use ctor::ctor;

use crate::{console, errors::DynErr, hooks, internal_failure, logging::logger};

#[ctor]
fn startup() {
    console::init().expect("Failed to get current executable");
    let current_exe = std::env::current_exe().expect("Failed to get current executable");
    let game_name = current_exe
        .file_name()
        .expect("Failed to get file name from current executable")
        .to_str()
        .expect("Failed to convert file name to string");


    if !game_name.to_lowercase().starts_with("vrchat") {
        return; 
    }

    thread::spawn(|| {
        init().unwrap_or_else(|e| {
          
            eprintln!("Failed to initialize MelonLoader: {}", e);
        });
    }); 
}

fn init() -> Result<(), DynErr> {
    console::init()?;
    println!("Health Check");

    logger::init()?;

    hooks::init_hook::hook()?;

    console::null_handles()?;

    Ok(())
}

pub fn shutdown() {
    std::process::exit(0);
}
