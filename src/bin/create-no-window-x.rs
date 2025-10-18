#![windows_subsystem = "windows"]

use std::{
    convert::Infallible,
    fs::OpenOptions,
    os::windows::process::CommandExt,
    path::Path,
    process::{Command, ExitStatus},
};

use chrono::Local;
use u3::chrono::ext::Ext;
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
    let out_dir = args.next().ok_or(Error::Args(ErrorArgs::OutDir))?;
    let program = args.next().ok_or(Error::Args(ErrorArgs::Program))?;

    let out_dir = Path::new(&out_dir);
    match std::fs::create_dir(out_dir) {
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => (),
        r => r.map_err(|e| Error::Dir1 { source: e })?,
    }
    let out_dir = out_dir.join(Local::now().format_for_filename().to_string());
    std::fs::create_dir(&out_dir).map_err(|e| Error::Dir2 { source: e })?;
    let stdout = OpenOptions::new()
        .create(true)
        .append(true)
        .open(out_dir.join("stdout.txt"))
        .map_err(|e| Error::File { source: e, stdio: ErrorFileStdio::Stdout })?;
    let stderr = OpenOptions::new()
        .create(true)
        .append(true)
        .open(out_dir.join("stderr.txt"))
        .map_err(|e| Error::File { source: e, stdio: ErrorFileStdio::Stderr })?;

    let status = Command::new(program)
        .args(args)
        .current_dir(&out_dir)
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
    Dir1 { source: std::io::Error },
    Dir2 { source: std::io::Error },
    File { source: std::io::Error, stdio: ErrorFileStdio },
    Status { source: std::io::Error },
    Exit(ExitStatus),
}

#[derive(Debug)]
enum ErrorArgs {
    Program,
    OutDir,
}

#[derive(Debug)]
enum ErrorFileStdio {
    Stdout,
    Stderr,
}
