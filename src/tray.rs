use std::sync::{atomic::AtomicBool, Arc};
use tray_item::{IconSource, TIError, TrayItem};

use crate::autostart::{is_autostart_enabled, toggle_startup};

#[cfg(target_os = "windows")]
enum Message {
    Quit,
    ToggleOn,
    ToggleOff,
}

#[cfg(target_os = "windows")]
pub fn tray(terminate: Arc<AtomicBool>) -> Result<(), TIError> {
    let mut tray = TrayItem::new("Mafia Engine", IconSource::Resource(""))?;

    let (tx, rx) = std::sync::mpsc::sync_channel(1);

    let enable_tx = tx.clone();
    tray.add_menu_item("Enable Auto-Start", move || {
        let _ = enable_tx.send(Message::ToggleOn);
    })?;

    let disable_tx = tx.clone();
    tray.add_menu_item("Disable Auto-Start", move || {
        let _ = disable_tx.send(Message::ToggleOff);
    })?;

    tray.inner_mut().add_separator().unwrap();

    let quit_tx = tx.clone();
    tray.add_menu_item("Quit", move || {
        let _ = quit_tx.send(Message::Quit);
    })?;

    let terminate_clone = Arc::clone(&terminate);
    loop {
        match rx.recv() {
            Ok(Message::Quit) => {
                println!("Quit");
                terminate_clone.store(true, std::sync::atomic::Ordering::SeqCst);
                std::process::exit(0);
            }
            Ok(Message::ToggleOn) => {
                let autostart = is_autostart_enabled().unwrap_or(false);
                if !autostart {
                    if let Err(err) = toggle_startup(true) {
                        println!("Failed to toggle startup: {}", err);
                    } else {
                        println!("Startup enabled");
                    }
                }
            }
            Ok(Message::ToggleOff) => {
                let autostart = is_autostart_enabled().unwrap_or(false);
                if autostart {
                    if let Err(err) = toggle_startup(false) {
                        println!("Failed to toggle startup: {}", err);
                    } else {
                        println!("Startup disabled");
                    }
                }
            }
            _ => {}
        }
    }
}

#[cfg(target_os = "macos")]
pub fn tray(terminate: Arc<AtomicBool>) -> Result<(), TIError> {
    let mut tray = TrayItem::new(
        "Mafia Engine",
        IconSource::Data {
            height: 512,
            width: 512,
            data: crate::ICON.to_vec(),
        },
    )?;

    let inner = tray.inner_mut();

    inner.add_menu_item("Enable Auto-Start", move || {
        let is_enabled = is_autostart_enabled().unwrap_or(false);
        if !is_enabled {
            if let Err(err) = toggle_startup(!is_enabled) {
                println!("Failed to toggle startup: {}", err);
            } else {
                println!(
                    "Startup {}",
                    if !is_enabled { "enabled" } else { "disabled" }
                );
            }
        }
    })?;

    inner.add_menu_item("Disable Auto-Start", move || {
        let is_enabled = is_autostart_enabled().unwrap_or(false);
        if is_enabled {
            if let Err(err) = toggle_startup(!is_enabled) {
                println!("Failed to toggle startup: {}", err);
            } else {
                println!(
                    "Startup {}",
                    if !is_enabled { "enabled" } else { "disabled" }
                );
            }
        }
    })?;

    inner.add_quit_item("Quit");
    inner.display();

    let terminate_clone = Arc::clone(&terminate);
    tray.add_menu_item("Quit", move || {
        terminate_clone.store(true, std::sync::atomic::Ordering::SeqCst);
        std::process::exit(0);
    })?;

    Ok(())
}
