use aws_sdk_s3::primitives::ByteStream;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Errs {
    #[error("read file failed: {0}")]
    ReadFile(String),
    #[error("something went wrong while put object file: {0}")]
    PUT(String),
}

impl super::Client {
    pub async fn put_bytes(&self, key: &str, bs: Vec<u8>) -> Result<(), Errs> {
        let bs = ByteStream::from(bs);
        let _ = self
            .s3
            .put_object()
            .bucket(&self.bucket)
            .key(key)
            .body(bs)
            .send()
            .await
            .map_err(|e| Errs::PUT(e.to_string()))?;
        Ok(())
    }

    pub async fn put_file(&self, key: &str, path: impl AsRef<std::path::Path>) -> Result<(), Errs> {
        let bs = ByteStream::from_path(path)
            .await
            .map_err(|e| Errs::ReadFile(e.to_string()))?;
        let _ = self
            .s3
            .put_object()
            .bucket(&self.bucket)
            .key(key)
            .body(bs)
            .send()
            .await
            .map_err(|e| Errs::PUT(e.to_string()))?;
        Ok(())
    }
}
