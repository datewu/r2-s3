use crate::s3;
use aws_sdk_s3::presigning::{PresigningConfig, PresigningConfigError};
use std::time::Duration;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PreSignErr {
    #[error(transparent)]
    PresignCfg(#[from] PresigningConfigError),
    #[error("presign failed {0}")]
    Presign(String),
}

// one weeks
const DEFAULT_EXPIRE_IN: Duration = Duration::from_secs(60 * 60 * 24 * 7 - 30);

/// Generate a presigned URL which expired in one week by default for a PUT request to S3.
/// hopefully some worker will put the file to S3 in one week.
pub async fn presigned_put(
    bucket: &str,
    key: &str,
    duration: Option<Duration>,
) -> Result<String, PreSignErr> {
    let client = s3::client().await;
    let duration = duration.unwrap_or(DEFAULT_EXPIRE_IN);
    let presign_cfg = PresigningConfig::expires_in(duration)?;

    let presigned_req = client
        .put_object()
        .bucket(bucket)
        .key(key)
        .presigned(presign_cfg)
        .await
        .map_err(|e| PreSignErr::Presign(e.to_string()))?;
    let url = presigned_req.uri();

    Ok(url.to_string())
}
