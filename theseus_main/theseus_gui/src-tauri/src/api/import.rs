use std::path::PathBuf;

use crate::api::Result;
use theseus::pack::import::ImportLauncherType;

use theseus::pack::import;
use theseus::prelude::ProfilePathId;

/// Gets a list of importable instances from a launcher type and base path
/// eg: get_importable_instances(ImportLauncherType::MultiMC, PathBuf::from("C:/MultiMC"))
/// returns ["Instance 1", "Instance 2"]
 
pub async fn import_get_importable_instances(
    launcher_type: ImportLauncherType,
    base_path: PathBuf,
) -> Result<Vec<String>> {
    Ok(import::get_importable_instances(launcher_type, base_path).await?)
}

/// Import an instance from a launcher type and base path
/// profile_path should be a blank profile for this purpose- if the function fails, it will be deleted
/// eg: import_instance(ImportLauncherType::MultiMC, PathBuf::from("C:/MultiMC"), "Instance 1")
 
pub async fn import_import_instance(
    profile_path: ProfilePathId,
    launcher_type: ImportLauncherType,
    base_path: PathBuf,
    instance_folder: String,
) -> Result<()> {
    import::import_instance(
        profile_path,
        launcher_type,
        base_path,
        instance_folder,
    )
    .await?;
    Ok(())
}

/// Checks if this instance is valid for importing, given a certain launcher type
/// eg: is_valid_importable_instance(PathBuf::from("C:/MultiMC/Instance 1"), ImportLauncherType::MultiMC)
 
pub async fn import_is_valid_importable_instance(
    instance_folder: PathBuf,
    launcher_type: ImportLauncherType,
) -> Result<bool> {
    Ok(
        import::is_valid_importable_instance(instance_folder, launcher_type)
            .await,
    )
}

/// Returns the default path for the given launcher type
/// None if it can't be found or doesn't exist
 
pub async fn import_get_default_launcher_path(
    launcher_type: ImportLauncherType,
) -> Result<Option<PathBuf>> {
    Ok(import::get_default_launcher_path(launcher_type))
}
