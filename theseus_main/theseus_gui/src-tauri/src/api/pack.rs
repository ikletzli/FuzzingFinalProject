use crate::api::Result;

use theseus::{
    pack::{
        install_from::{CreatePackLocation, CreatePackProfile},
        install_mrpack::install_zipped_mrpack,
    },
    prelude::*,
};

 
pub async fn pack_install(
    location: CreatePackLocation,
    profile: ProfilePathId,
) -> Result<ProfilePathId> {
    Ok(install_zipped_mrpack(location, profile).await?)
}

 
pub fn pack_get_profile_from_pack(
    location: CreatePackLocation,
) -> Result<CreatePackProfile> {
    Ok(pack::install_from::get_profile_from_pack(location))
}
