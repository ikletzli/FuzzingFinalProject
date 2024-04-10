use std::path::PathBuf;

use crate::api::Result;
use theseus::prelude::*;


// Get full settings
// invoke('plugin:settings|settings_get')
 
pub async fn settings_get() -> Result<Settings> {
    let res = settings::get().await?;
    Ok(res)
}

// Set full settings
// invoke('plugin:settings|settings_set', settings)
 
pub async fn settings_set(settings: Settings) -> Result<()> {
    settings::set(settings).await?;
    Ok(())
}

// Change config directory
// Seizes the entire State to do it
// invoke('plugin:settings|settings_change_config_dir', new_dir)
 
pub async fn settings_change_config_dir(new_config_dir: PathBuf) -> Result<()> {
    settings::set_config_dir(new_config_dir).await?;
    Ok(())
}

 
pub async fn settings_is_dir_writeable(
    new_config_dir: PathBuf,
) -> Result<bool> {
    let res = settings::is_dir_writeable(new_config_dir).await?;
    Ok(res)
}
