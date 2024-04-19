#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use theseus::prelude::*;
use std::fs;
use std::io::Error;
use std::collections::HashMap;
use crate::api::Result;
use std::{env, path::PathBuf, process::Command};
mod api;
mod error;

#[cfg(target_os = "macos")]
mod macos;

pub fn show_in_folder(path: PathBuf) -> Result<()> {
    {
        #[cfg(target_os = "windows")]
        {
            if path.is_dir() {
                Command::new("explorer")
                    .args([&path]) // The comma after select is not a typo
                    .spawn()?;
            } else {
                Command::new("explorer")
                    .args(["/select,", &path.to_string_lossy()]) // The comma after select is not a typo
                    .spawn()?;
            }
        }

        #[cfg(target_os = "linux")]
        {
            use std::fs::metadata;
            use std::path::PathBuf;

            if path.to_string_lossy().to_string().contains(',') {
                // see https://gitlab.freedesktop.org/dbus/dbus/-/issues/76
                let new_path = match metadata(&path)?.is_dir() {
                    true => path,
                    false => {
                        let mut path2 = PathBuf::from(path);
                        path2.pop();
                        path2
                    }
                };
                Command::new("xdg-open").arg(&new_path).spawn()?;
            } else {
                Command::new("xdg-open").arg(&path).spawn()?;
            }
        }

        #[cfg(target_os = "macos")]
        {
            if path.is_dir() {
                Command::new("open").args([&path]).spawn()?;
            } else {
                Command::new("open")
                    .args(["-R", &path.as_os_str().to_string_lossy()])
                    .spawn()?;
            }
        }

        Ok::<(), theseus::Error>(())
    }?;

    Ok(())
}

fn is_dev() -> bool {
    cfg!(debug_assertions)
}

#[derive(Clone, serde::Serialize)]
struct Payload {
    args: Vec<String>,
    cwd: String,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let path = &args[1]; // zwCoPr4q
        let version_id = fs::read_to_string(path).unwrap();

        profile::harness(&version_id).await;
    }
}
