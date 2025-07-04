// use minio::s3::Client;
// use minio::s3::types::S3Api;
// use minio::s3::response::BucketExistsResponse;

// #[tokio::main]
// async fn main() {
//     let client: Client = Default::default(); // configure your client
//     let exists: BucketExistsResponse = client
//         .bucket_exists("my-bucket")
//         .send()
//         .await
//         .expect("request failed");

//     println!("Bucket exists: {}", exists.exists);
// }

use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello, carl!" }));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}