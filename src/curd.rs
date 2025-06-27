use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::types::ObjectIdentifier;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Errs {
    #[error("read file failed: {0}")]
    ReadFile(String),
    #[error("something went wrong while put object file: {0}")]
    Put(String),
    #[error("something went wrong while delete object file: {0}")]
    Delete(String),
    #[error("something went wrong while list object file: {0}")]
    List(String),
}

impl super::Client {
    /// put an object and specify the content type
    pub async fn put_bytes_with_ct(&self, key: &str, bs: Vec<u8>, ct: &str) -> Result<(), Errs> {
        let bs = ByteStream::from(bs);
        let _ = self
            .s3
            .put_object()
            .bucket(&self.bucket)
            .key(key)
            .body(bs)
            .content_type(ct)
            .send()
            .await
            .map_err(|e| Errs::Put(e.to_string()))?;
        Ok(())
    }

    /// put an object with a key
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
            .map_err(|e| Errs::Put(e.to_string()))?;
        Ok(())
    }

    /// put an file and specify the content type
    pub async fn put_file_with_ct(
        &self,
        key: &str,
        path: impl AsRef<std::path::Path>,
        ct: &str,
    ) -> Result<(), Errs> {
        let bs = ByteStream::from_path(path)
            .await
            .map_err(|e| Errs::ReadFile(e.to_string()))?;
        let _ = self
            .s3
            .put_object()
            .bucket(&self.bucket)
            .key(key)
            .body(bs)
            .content_type(ct)
            .send()
            .await
            .map_err(|e| Errs::Put(e.to_string()))?;
        Ok(())
    }

    /// put an file with a key
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
            .map_err(|e| Errs::Put(e.to_string()))?;
        Ok(())
    }

    /// delete an object by key
    pub async fn delete(&self, key: &str) -> Result<(), Errs> {
        let _ = self
            .s3
            .delete_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await
            .map_err(|e| Errs::Delete(e.to_string()))?;
        Ok(())
    }

    /// delete objects by a prefix
    pub async fn batch_delete(&self, prefix: &str) -> Result<(), Errs> {
        let mut continuation_token = None;
        let client = &self.s3;
        let bucket = &self.bucket;
        loop {
            let mut resp = client.list_objects_v2().bucket(bucket).prefix(prefix);

            if let Some(token) = continuation_token {
                resp = resp.continuation_token(token);
            }

            let list_objects_output = resp.send().await.map_err(|e| Errs::List(e.to_string()))?;

            let contents = list_objects_output.contents();
            if contents.is_empty() {
                println!("No objects found with prefix '{prefix}'.");
                break;
            }

            let mut objects_to_delete = Vec::new();
            for object in contents {
                if let Some(key) = object.key() {
                    match ObjectIdentifier::builder().key(key).build() {
                        Ok(obj) => objects_to_delete.push(obj),
                        Err(e) => {
                            eprintln!("{e}");
                            continue;
                        }
                    }
                }
            }

            if !objects_to_delete.is_empty() {
                let delete_builder = aws_sdk_s3::types::Delete::builder();
                let delete_input = delete_builder.set_objects(Some(objects_to_delete)).build(); // This line might need adjustment based on current SDK, ensuring it handles potential errors if build() returns Result

                match delete_input {
                    Ok(delete_payload) => {
                        let delete_objects_output = client
                            .delete_objects()
                            .bucket(bucket)
                            .delete(delete_payload)
                            .send()
                            .await
                            .map_err(|e| Errs::Delete(e.to_string()))?;
                        for d in delete_objects_output.deleted() {
                            println!("Successfully deleted: {}", d.key().unwrap_or("Unknown key"));
                        }

                        for err in delete_objects_output.errors() {
                            eprintln!(
                                "Error deleting object {}: {}",
                                err.key().unwrap_or("Unknown key"),
                                err.message().unwrap_or("Unknown error")
                            );
                        }
                    }
                    Err(build_error) => {
                        // Handle the error from `delete_builder.set_objects(...).build()`
                        // This part depends on how the SDK's `Delete::builder()` and `build()` methods return errors.
                        // For example, if `build()` returns a `Result<Delete, SomeErrorType>`:
                        eprintln!("Error building delete payload: {build_error:?}");
                        // Potentially break or return an error
                        break;
                    }
                }
            }

            if list_objects_output.is_truncated() == Some(true) {
                continuation_token = list_objects_output
                    .next_continuation_token()
                    .map(String::from);
            } else {
                break;
            }
        }

        println!("Finished deleting objects with prefix '{prefix}'.");
        Ok(())
    }
}
