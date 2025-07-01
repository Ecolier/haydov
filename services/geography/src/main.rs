use minio::s3::Client;
use minio::s3::types::S3Api;
use minio::s3::response::BucketExistsResponse;

#[tokio::main]
async fn main() {
    let client: Client = Default::default(); // configure your client

    let exists: BucketExistsResponse = client
        .bucket_exists("my-bucket")
        .send()
        .await
        .expect("request failed");

    println!("Bucket exists: {}", exists.exists);
}