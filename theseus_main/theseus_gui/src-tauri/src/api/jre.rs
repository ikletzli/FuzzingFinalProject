use std::path::PathBuf;

use crate::api::Result;
use theseus::prelude::JavaVersion;
use theseus::prelude::*;


/// Get all JREs that exist on the system
 
pub async fn jre_get_all_jre() -> Result<Vec<JavaVersion>> {
    Ok(jre::get_all_jre().await?)
}

// Finds the installation of Java 8, if it exists
 
pub async fn jre_find_filtered_jres(
    jres: Vec<JavaVersion>,
    version: String,
    allow_higher: bool,
) -> Result<Vec<JavaVersion>> {
    Ok(jre::find_filtered_jres(&version, jres, allow_higher).await?)
}

// Autodetect Java globals, by searching the users computer.
// Selects from the given JREs, and returns a new JavaGlobals
// Returns a *NEW* JavaGlobals that can be put into Settings
 
pub async fn jre_autodetect_java_globals(
    java_8: Vec<JavaVersion>,
    java_17: Vec<JavaVersion>,
    java_18plus: Vec<JavaVersion>,
) -> Result<JavaGlobals> {
    Ok(jre::autodetect_java_globals(java_8, java_17, java_18plus).await?)
}

// Validates java globals, by checking if the paths exist
// If false, recommend to direct them to reassign, or to re-guess
 
pub async fn jre_validate_globals() -> Result<bool> {
    Ok(jre::validate_globals().await?)
}

// Validates JRE at a given path
// Returns None if the path is not a valid JRE
 
pub async fn jre_get_jre(path: PathBuf) -> Result<Option<JavaVersion>> {
    jre::check_jre(path).await.map_err(|e| e.into())
}

// Tests JRE of a certain version
 
pub async fn jre_test_jre(
    path: PathBuf,
    major_version: u32,
    minor_version: u32,
) -> Result<bool> {
    Ok(jre::test_jre(path, major_version, minor_version).await?)
}

// Auto installs java for the given java version
 
pub async fn jre_auto_install_java(java_version: u32) -> Result<PathBuf> {
    Ok(jre::auto_install_java(java_version).await?)
}

// Gets the maximum memory a system has available.
 
pub async fn jre_get_max_memory() -> Result<u64> {
    Ok(jre::get_max_memory().await?)
}
