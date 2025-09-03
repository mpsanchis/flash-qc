use std::process::{Command, Stdio};

fn tool_exists(tool: &str) -> bool {
    Command::new("which")
        .arg(tool)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

pub fn install(installer: Option<&str>) -> Result<(), String> {
    let command: &str;

    if let Some(installer) = installer {
        if !tool_exists(installer) {
            return Err(format!("Could not find tool '{}'", installer));
        }
        command = installer;
    } else {
        // No installer passed: try first uvx and then pipx
        if tool_exists("uvx") {
            command = "uvx";
        } else if tool_exists("pipx") {
            command = "pipx";
        } else {
            return Err(String::from(
                "Cannot install pre-commit: neither 'uvx' nor 'pipx' is installed. Make sure to run 'mise install'",
            ));
        }
    }

    Command::new(command)
        .arg("pre-commit")
        .arg("install")
        .status()
        .and_then(|status| {
            if !status.success() {
                Err(std::io::Error::other(""))
            } else {
                Ok(())
            }
        })
        .map_err(|e| e.to_string())
}
