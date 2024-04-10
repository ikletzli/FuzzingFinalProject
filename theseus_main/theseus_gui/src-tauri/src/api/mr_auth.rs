use crate::api::Result;
use theseus::prelude::*;


 
pub async fn authenticate_begin_flow(provider: &str) -> Result<String> {
    Ok(theseus::mr_auth::authenticate_begin_flow(provider).await?)
}

 
pub async fn authenticate_await_completion() -> Result<ModrinthCredentialsResult>
{
    Ok(theseus::mr_auth::authenticate_await_complete_flow().await?)
}

 
pub async fn cancel_flow() -> Result<()> {
    Ok(theseus::mr_auth::cancel_flow().await?)
}

 
pub async fn login_pass(
    username: &str,
    password: &str,
    challenge: &str,
) -> Result<ModrinthCredentialsResult> {
    Ok(theseus::mr_auth::login_password(username, password, challenge).await?)
}

 
pub async fn login_2fa(code: &str, flow: &str) -> Result<ModrinthCredentials> {
    Ok(theseus::mr_auth::login_2fa(code, flow).await?)
}

 
pub async fn create_account(
    username: &str,
    email: &str,
    password: &str,
    challenge: &str,
    sign_up_newsletter: bool,
) -> Result<ModrinthCredentials> {
    Ok(theseus::mr_auth::create_account(
        username,
        email,
        password,
        challenge,
        sign_up_newsletter,
    )
    .await?)
}

 
pub async fn refresh() -> Result<()> {
    Ok(theseus::mr_auth::refresh().await?)
}

 
pub async fn logout() -> Result<()> {
    Ok(theseus::mr_auth::logout().await?)
}

 
pub async fn get() -> Result<Option<ModrinthCredentials>> {
    Ok(theseus::mr_auth::get_credentials().await?)
}
