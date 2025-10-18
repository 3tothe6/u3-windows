#![windows_subsystem = "windows"]

use std::{
    convert::Infallible,
    fs::OpenOptions,
    os::windows::process::CommandExt,
    process::{Command, ExitStatus},
};

use windows::Win32::System::Threading::CREATE_NO_WINDOW;
use winrt_notification::Toast;

fn main() {
    let Err(e) = _main();
    Toast::new(Toast::POWERSHELL_APP_ID)
        .title(&format!("create-no-window error: {e:?}"))
        .show()
        .unwrap();
}

fn _main() -> Result<Infallible, Error> {
    let mut args = std::env::args_os().skip(1);
    let stdout = args.next().ok_or(Error::Args(ErrorArgs::Stdout))?;
    let stderr = args.next().ok_or(Error::Args(ErrorArgs::Stderr))?;
    let program = args.next().ok_or(Error::Args(ErrorArgs::Program))?;

    let stdout = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&stdout)
        .map_err(|e| Error::File { source: e, stdio: ErrorFileStdio::Stdout })?;
    let stderr = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&stderr)
        .map_err(|e| Error::File { source: e, stdio: ErrorFileStdio::Stderr })?;

    let status = Command::new(program)
        .args(args)
        .stdout(stdout)
        .stderr(stderr)
        .creation_flags(CREATE_NO_WINDOW.0)
        .status()
        .map_err(|e| Error::Status { source: e })?;
    Err(Error::Exit(status))
}

#[allow(dead_code)]
#[derive(Debug)]
enum Error {
    Args(ErrorArgs),
    File { source: std::io::Error, stdio: ErrorFileStdio },
    Status { source: std::io::Error },
    Exit(ExitStatus),
}

#[derive(Debug)]
enum ErrorArgs {
    Program,
    Stdout,
    Stderr,
}

#[derive(Debug)]
enum ErrorFileStdio {
    Stdout,
    Stderr,
}
