//! Host-owned transcript display; the harness receives no display capability.
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use crate::standalone::{shell_word, Ui};
use anyhow::{bail, Context, Result};

pub(crate) fn relay(mut log: File, finished: &AtomicBool) -> Result<()> {
    let mut buffer = [0; 8192];
    loop {
        let size = log.read(&mut buffer)?;
        if size > 0 {
            render(&buffer[..size], &mut std::io::stderr().lock())?;
        } else if finished.load(Ordering::Acquire) {
            // The writer has stopped. A final read covers bytes appended
            // between the preceding EOF and observing completion.
            let size = log.read(&mut buffer)?;
            render(&buffer[..size], &mut std::io::stderr().lock())?;
            if size == 0 {
                return Ok(());
            }
        } else {
            std::thread::sleep(Duration::from_millis(100));
        }
    }
}

fn render(bytes: &[u8], writer: &mut impl Write) -> Result<()> {
    for character in String::from_utf8_lossy(bytes).chars() {
        if character.is_control() && character != '\n' && character != '\t' {
            write!(writer, "{}", character.escape_default())?;
        } else {
            write!(writer, "{character}")?;
        }
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn watch(log: &Path, status: &Path) -> Result<()> {
    let mut log = File::open(log).context("opening standalone transcript")?;
    let mut buffer = [0; 8192];
    loop {
        let size = log.read(&mut buffer)?;
        render(&buffer[..size], &mut std::io::stdout().lock())?;
        if size == 0 {
            if let Ok(status) = std::fs::read_to_string(status) {
                println!("\ngrove run: {status}");
                return Ok(());
            }
            std::thread::sleep(Duration::from_millis(100));
        }
    }
}

pub(crate) fn open(ui: Ui, kind: &str, log: &Path, status: &Path) -> Result<()> {
    if matches!(ui, Ui::Inline) {
        return Ok(());
    }
    let executable = std::env::current_exe()?;
    let result = if std::env::var_os("TMUX").is_some() {
        let words = [
            executable.as_os_str(),
            std::ffi::OsStr::new("run-log"),
            log.as_os_str(),
            status.as_os_str(),
        ];
        let shell_command = words
            .into_iter()
            .map(shell_word)
            .collect::<Result<Vec<_>>>()?
            .join(" ");
        Command::new("tmux")
            .args([
                "split-window",
                "-d",
                "-P",
                "-F",
                "#{pane_id}",
                &shell_command,
            ])
            .output()
    } else if std::env::var_os("ZELLIJ").is_some() {
        Command::new("zellij")
            .args([
                "action",
                "new-pane",
                "--name",
                &format!("grove:{kind}"),
                "--no-focus",
                "--",
            ])
            .arg(&executable)
            .arg("run-log")
            .arg(log)
            .arg(status)
            .output()
    } else if matches!(ui, Ui::Pane) {
        bail!("--ui pane needs an existing tmux or Zellij session; use --ui inline here");
    } else {
        return Ok(());
    };
    match result {
        Ok(output) if output.status.success() => Ok(()),
        result if matches!(ui, Ui::Auto) => {
            eprintln!("grove run: pane unavailable ({result:?}); displaying inline");
            Ok(())
        }
        result => bail!("cannot open the requested task pane ({result:?}); use --ui inline or repair the mux connection"),
    }
}

#[cfg(test)]
#[path = "../tests/internal/run_display.rs"]
mod tests;
