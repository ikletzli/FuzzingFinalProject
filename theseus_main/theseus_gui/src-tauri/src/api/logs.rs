use crate::api::Result;
use theseus::{
    logs::{self, CensoredString, LatestLogCursor, Logs},
    prelude::ProfilePathId,
};

/*
A log is a struct containing the filename string, stdout, and stderr, as follows:

pub struct Logs {
    pub filename:  String,
    pub stdout: String,
    pub stderr: String,
}
*/


/// Get all Logs for a profile, sorted by filename
 
pub async fn logs_get_logs(
    profile_path: ProfilePathId,
    clear_contents: Option<bool>,
) -> Result<Vec<Logs>> {
    let val = logs::get_logs(profile_path, clear_contents).await?;

    Ok(val)
}

/// Get a Log struct for a profile by profile id and filename string
 
pub async fn logs_get_logs_by_filename(
    profile_path: ProfilePathId,
    filename: String,
) -> Result<Logs> {
    Ok(logs::get_logs_by_filename(profile_path, filename).await?)
}

/// Get the stdout for a profile by profile id and filename string
 
pub async fn logs_get_output_by_filename(
    profile_path: ProfilePathId,
    filename: String,
) -> Result<CensoredString> {
    let profile_path = if let Some(p) =
        crate::profile::get(&profile_path, None).await?
    {
        p.profile_id()
    } else {
        return Err(theseus::Error::from(
            theseus::ErrorKind::UnmanagedProfileError(profile_path.to_string()),
        )
        .into());
    };

    Ok(logs::get_output_by_filename(&profile_path, &filename).await?)
}

/// Delete all logs for a profile by profile id
 
pub async fn logs_delete_logs(profile_path: ProfilePathId) -> Result<()> {
    Ok(logs::delete_logs(profile_path).await?)
}

/// Delete a log for a profile by profile id and filename string
 
pub async fn logs_delete_logs_by_filename(
    profile_path: ProfilePathId,
    filename: String,
) -> Result<()> {
    Ok(logs::delete_logs_by_filename(profile_path, &filename).await?)
}

/// Get live log from a cursor
 
pub async fn logs_get_latest_log_cursor(
    profile_path: ProfilePathId,
    cursor: u64, // 0 to start at beginning of file
) -> Result<LatestLogCursor> {
    Ok(logs::get_latest_log_cursor(profile_path, cursor).await?)
}
