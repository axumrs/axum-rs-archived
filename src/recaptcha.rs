use serde::{Deserialize, Serialize};

use crate::{
    error::{AppError, AppErrorType},
    Result,
};

#[derive(Serialize)]
pub struct VerifyRequest {
    pub secret: String,
    pub response: String,
}
#[derive(Deserialize)]
pub struct VerifyResponse {
    pub success: bool,
}
pub async fn verify(response: String, secret: String) -> Result<bool> {
    let req = VerifyRequest { secret, response };
    let client = reqwest::Client::new();
    let res = client
        .post("https://www.google.com/recaptcha/api/siteverify")
        .form(&req)
        .send()
        .await
        .map_err(|err| {
            tracing::error!(" POST {:?}", err);
            AppError::from_err(err, AppErrorType::HttpError)
        })?;
    let res = res.text().await.map_err(|err| {
        tracing::error!("TEXT {:?}", err);
        AppError::from_err(err, AppErrorType::HttpError)
    })?;
    tracing::debug!("{:?}", res);
    let res: VerifyResponse = serde_json::from_str(&res).map_err(|err| {
        tracing::error!(" DESERIALIZE {:?}", err);
        AppError::from_err(err, AppErrorType::HttpError)
    })?;
    Ok(res.success)
}
