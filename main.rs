#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::Manager;
use theseus::prelude::*;
use std::env;
use std::process::Command;
use std::io::Error;
use std::collections::HashMap;

mod api;
mod error;

#[cfg(target_os = "macos")]
mod macos;

// Should be called in launcher initialization
#[tracing::instrument(skip_all)]
#[tauri::command]
async fn initialize_state(app: tauri::AppHandle) -> api::Result<()> {
    theseus::EventState::init(app).await?;
    let s = State::get().await?;
    State::update();

    s.children.write().await.rescue_cache().await?;
    Ok(())
}

#[tauri::command]
fn is_dev() -> bool {
    cfg!(debug_assertions)
}

// Toggles decorations
#[tauri::command]
async fn toggle_decorations(b: bool, window: tauri::Window) -> api::Result<()> {
    window.set_decorations(b).map_err(|e| {
        theseus::Error::from(theseus::ErrorKind::OtherError(format!(
            "Failed to toggle decorations: {}",
            e
        )))
    })?;
    Ok(())
}

#[derive(Clone, serde::Serialize)]
struct Payload {
    args: Vec<String>,
    cwd: String,
}

// if Tauri app is called with arguments, then those arguments will be treated as commands
// ie: deep links or filepaths for .mrpacks
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let first_arg = &args[1];
        let status = Command::new("cp")
        .arg(first_arg)
        .arg("/root/.config/com.modrinth.theseus/profiles/test/mods/test.jar")
        .status();
    }

    let _log_guard = theseus::start_logger();

    tracing::info!("Initialized tracing subscriber. Loading Modrinth App!");

    let profiles = tauri::async_runtime::block_on(async {
        //theseus::EventState::init(app).await?;
        let s = State::get().await?;
        State::update();
    
        s.children.write().await.rescue_cache().await?;


        let res = profile::list(Some(true)).await?;

        Ok::<HashMap<ProfilePathId, Profile>, theseus::Error>(res)
    }).unwrap();

    // ProfilePathId can do .path on one of these

    // uuid,
    // install_stage: ProfileInstallStage::NotInstalled,
    // path: PathBuf::new().join(&name),
    // metadata: ProfileMetadata {
    //     name,
    //     icon: None,
    //     icon_url: None,
    //     groups: vec![],
    //     game_version: version,
    //     loader: ModLoader::Vanilla,
    //     loader_version: None,
    //     linked_data: None,
    //     date_created: Utc::now(),
    //     date_modified: Utc::now(),
    //     last_played: None,
    //     submitted_time_played: 0,
    //     recent_time_played: 0,
    // },
    // projects: HashMap::new(),
    // java: None,
    // memory: None,
    // resolution: None,
    // fullscreen: None,
    // hooks: None,
    // modrinth_update_version: None,


    // pub struct Project {
    //     pub sha512: String,
    //     pub disabled: bool,
    //     pub metadata: ProjectMetadata,
    //     pub file_name: String,
    // }


    for (key, value) in &profiles {
        let projects = &value.projects;
        for (new_key, new_value) in projects {
            let relative_path = key.0.to_path_buf().into_os_string().into_string().unwrap();
            println!("{}: {}", relative_path, new_value.file_name);
        }
    }

    // let mut builder = tauri::Builder::default();

    // builder = builder.setup(|app| {
    //     let win = app.get_window("main").unwrap();
    //     win.show().unwrap();

    //     tauri::async_runtime::block_on(async {
    //         theseus::EventState::init(app.handle()).await?;
    //         let s = State::get().await?;
    //         State::update();
    //         tracing::info!("LOOL tracing subscriber. Loading Modrinth App!");
    //         s.children.write().await.rescue_cache().await?;

    //         app.handle().exit(0);

    //         Ok::<(), theseus::Error>(())
    //     });
    //     Ok(())
    // });

    // builder
    // .run(tauri::generate_context!())
    // .expect("error while running tauri application");
}
