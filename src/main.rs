use r2_s3::Client;
use std::env;

#[tokio::main]
async fn main() {
    print_usage();
    let mut args = env::args();
    args.next();
    let bucket = args.next().unwrap_or("my-bucket".to_string());
    let key = args.next().unwrap_or("test-fold/test-put".to_string());
    let client = Client::new(&bucket).await;

    let url = client
        .presigned_put(&key, None)
        .await
        .expect("should be ok");
    println!("presign put url for {bucket} {key} is: \n'{url}'");
}

fn print_usage() {
    println!("Usage: presign_put bucket_key(default: test-fold/test-put)");
    println!("Then use curl command the output url put your file to R2 storage.");
    println!("`curl --upload-file your-file 'https://output-url'`");
}
