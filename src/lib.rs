pub mod curd;
pub mod presign;
pub(crate) mod s3;
use aws_sdk_s3 as s3_sdk;

pub struct Client {
    s3: s3_sdk::Client,
    pub bucket: String,
}

impl Client {
    pub async fn new(b: impl ToString) -> Self {
        let s3 = s3::client().await;
        let bucket = b.to_string();
        Self { s3, bucket }
    }
}
