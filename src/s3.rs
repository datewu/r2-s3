use aws_config::SdkConfig;
use aws_sdk_s3 as s3;
// using tokio::sync::OnceCell ensures that initialization happens only once, even in the presence of concurrency or parallelism
use tokio::sync::OnceCell;

async fn s3_conf() -> SdkConfig {
    let endpoint_url = std::env::var("R2_S3_URL").expect("R2_S3_URL must be set");
    aws_config::from_env()
        .region("auto")
        .endpoint_url(endpoint_url)
        .load()
        .await
}

// Static OnceCell to hold the SdkConfig, initialized once.
static S3_CONFIG: OnceCell<SdkConfig> = OnceCell::const_new();

async fn get_s3_config() -> &'static SdkConfig {
    S3_CONFIG.get_or_init(s3_conf).await
}

pub(crate) async fn client() -> s3::Client {
    let conf = get_s3_config().await;
    s3::Client::new(conf)
}
