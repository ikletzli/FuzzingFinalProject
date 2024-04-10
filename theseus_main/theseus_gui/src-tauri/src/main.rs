#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use theseus::prelude::*;
use std::io::Error;
use std::collections::HashMap;
use crate::api::Result;
use std::{env, path::PathBuf, process::Command};
mod api;
mod error;

#[cfg(target_os = "macos")]
mod macos;

// Should be called in launcher initialization
// #[tracing::instrument(skip_all)]
// async fn initialize_state(app: tauri::AppHandle) -> api::Result<()> {
//     theseus::EventState::init(app).await?;
//     let s = State::get().await?;
//     State::update();

//     s.children.write().await.rescue_cache().await?;
//     Ok(())
// }

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

#[tokio::main]
// if Tauri app is called with arguments, then those arguments will be treated as commands
// ie: deep links or filepaths for .mrpacks
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let first_arg = &args[1];
        let status = Command::new("cp")
        .arg(first_arg)
        .arg("/root/.config/com.modrinth.theseus/profiles/test/mods/test.jar")
        .status();
    }

    let profiles = async_io::block_on(async {
        profile::big_update().await?;
        Ok::<(), theseus::Error>(())
    });


    // let profiles = async_io::block_on(async {
    //     let res = profile::list(Some(false)).await?;
    //     Ok::<HashMap<ProfilePathId, Profile>, theseus::Error>(res)
    // }).unwrap();

    // // ProfilePathId can do .path on one of these

    // // uuid,
    // // install_stage: ProfileInstallStage::NotInstalled,
    // // path: PathBuf::new().join(&name),
    // // metadata: ProfileMetadata {
    // //     name,
    // //     icon: None,
    // //     icon_url: None,
    // //     groups: vec![],
    // //     game_version: version,
    // //     loader: ModLoader::Vanilla,
    // //     loader_version: None,
    // //     linked_data: None,
    // //     date_created: Utc::now(),
    // //     date_modified: Utc::now(),
    // //     last_played: None,
    // //     submitted_time_played: 0,
    // //     recent_time_played: 0,
    // // },
    // // projects: HashMap::new(),
    // // java: None,
    // // memory: None,
    // // resolution: None,
    // // fullscreen: None,
    // // hooks: None,
    // // modrinth_update_version: None,


    // // pub struct Project {
    // //     pub sha512: String,
    // //     pub disabled: bool,
    // //     pub metadata: ProjectMetadata,
    // //     pub file_name: String,
    // // }


    // let profiles = async_io::block_on(async {
    //     for (instance_path, value) in &profiles {
    //         let projects = &value.projects;
    //         for (project_path, project) in projects {
    //             //profile::update_project(&instance_path, &project_path, None).await?;
    //             let fullPath = profile::get_mod_full_path(&instance_path, &project_path).await.unwrap();
    //             show_in_folder(fullPath);
    //             profile::toggle_disable_project(&instance_path, &project_path).await?;
    //             profile::toggle_disable_project(&instance_path, &project_path).await?;
    //             profile::remove_project(&instance_path, &project_path).await?;
    //         }
    //     }
    //     Ok::<(), theseus::Error>(())
    // });
}
