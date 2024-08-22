use std::fs;

pub const PLIST_NAME: &str = "com.mafiaengine.Discord Mafia Advertisement";

pub fn is_autostart_enabled() -> Result<bool, Box<dyn std::error::Error>> {
    #[cfg(target_os = "windows")]
    {
        let startup_dir =
            std::env::var("APPDATA")? + r"\Microsoft\Windows\Start Menu\Programs\Startup";
        let shortcut_path =
            std::path::PathBuf::from(startup_dir).join(format!("{}.lnk", PLIST_NAME));

        Ok(shortcut_path.exists())
    }

    #[cfg(target_os = "macos")]
    {
        let plist_name = format!("{}.plist", PLIST_NAME);
        let home_dir = dirs::home_dir().ok_or("Could not get home directory")?;
        let launch_agents_dir = home_dir.join("Library/LaunchAgents");
        let plist_path = launch_agents_dir.join(plist_name);
        Ok(plist_path.exists())
    }
}

pub fn toggle_startup(enable: bool) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_os = "windows")]
    {
        let startup_dir =
            std::env::var("APPDATA")? + r"\Microsoft\Windows\Start Menu\Programs\Startup";
        let shortcut_path =
            std::path::PathBuf::from(startup_dir).join(format!("{}.lnk", PLIST_NAME));

        if enable {
            if !shortcut_path.exists() {
                let executable_path = std::env::current_exe()?;
                create_shortcut(&shortcut_path, &executable_path)?;
            }
        } else {
            if shortcut_path.exists() {
                fs::remove_file(shortcut_path)?;
            }
        }
    }

    #[cfg(target_os = "macos")]
    {
        let plist_name = format!("{}.plist", PLIST_NAME);
        let home_dir = dirs::home_dir().ok_or("Could not get home directory")?;
        let launch_agents_dir = home_dir.join("Library/LaunchAgents");
        let plist_path = launch_agents_dir.join(plist_name);

        if enable {
            if !plist_path.exists() {
                let executable_path = std::env::current_exe()?;
                let plist_content = format!(
                    r#"<?xml version="1.0" encoding="UTF-8"?>
                    <!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
                    <plist version="1.0">
                    <dict>
                        <key>Label</key>
                        <string>{}</string>
                        <key>ProgramArguments</key>
                        <array>
                            <string>{}</string>
                        </array>
                        <key>RunAtLoad</key>
                        <true/>
                    </dict>
                    </plist>"#,
                    PLIST_NAME,
                    executable_path.display()
                );
                fs::write(&plist_path, plist_content)?;
            }
        } else {
            if plist_path.exists() {
                fs::remove_file(plist_path)?;
            }
        }
    }

    Ok(())
}

#[cfg(target_os = "windows")]
fn create_shortcut(
    shortcut_path: &std::path::PathBuf,
    target_path: &std::path::PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let wsh = r#"
        Set oWS = WScript.CreateObject("WScript.Shell")
        sLinkFile = "<SHORTCUT_PATH>"
        Set oLink = oWS.CreateShortcut(sLinkFile)
        oLink.TargetPath = "<TARGET_PATH>"
        oLink.Save
    "#;
    let wsh = wsh.replace("<SHORTCUT_PATH>", shortcut_path.to_str().unwrap());
    let wsh = wsh.replace("<TARGET_PATH>", target_path.to_str().unwrap());

    let wsh_file = std::env::temp_dir().join("create_shortcut.vbs");
    fs::write(&wsh_file, wsh)?;
    std::process::Command::new("wscript")
        .arg(wsh_file)
        .status()?;

    Ok(())
}
