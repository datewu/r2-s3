# Usage

The following four environment variables must be set before setup the client

 ```shell
export AWS_ACCESS_KEY_ID=your_key_id
export AWS_SECRET_ACCESS_KEY=your_access_key
export AWS_REGION=auto

export R2_S3_URL=https://your-r2-hash-url.r2.cloudflarestorage.com

```

## Demo

```rust
use r2_s3::Client;

#[tokio::main]
async fn main() {
    let mut args = env::args();
    args.next();
    let bucket = args.next().unwrap_or("your-bucket".to_string());
    let key = args.next().unwrap_or("your_key".to_string());
    let client = Client::new(&bucket).await;

    let url = client
        .presigned_put(&key, None)
        .await
        .expect("should be ok");
    println!("presign put url for {bucket} {key} is: \n'{url}'");
}

```
