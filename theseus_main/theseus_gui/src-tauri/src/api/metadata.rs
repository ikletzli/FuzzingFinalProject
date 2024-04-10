use crate::api::Result;
use daedalus::minecraft::VersionManifest;
use daedalus::modded::Manifest;

/// Gets the game versions from daedalus
 
pub async fn metadata_get_game_versions() -> Result<VersionManifest> {
    Ok(theseus::metadata::get_minecraft_versions().await?)
}

/// Gets the fabric versions from daedalus
 
pub async fn metadata_get_fabric_versions() -> Result<Manifest> {
    Ok(theseus::metadata::get_fabric_versions().await?)
}

/// Gets the forge versions from daedalus
 
pub async fn metadata_get_forge_versions() -> Result<Manifest> {
    Ok(theseus::metadata::get_forge_versions().await?)
}

/// Gets the quilt versions from daedalus
 
pub async fn metadata_get_quilt_versions() -> Result<Manifest> {
    Ok(theseus::metadata::get_quilt_versions().await?)
}

/// Gets the quilt versions from daedalus
 
pub async fn metadata_get_neoforge_versions() -> Result<Manifest> {
    Ok(theseus::metadata::get_neoforge_versions().await?)
}
