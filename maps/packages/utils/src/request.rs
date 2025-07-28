use futures::{
    stream::{self}, Stream, StreamExt
};
use std::{pin::Pin, sync::Arc};
use url::Url;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Malformed URL error")]
    ParseUrlError(#[from] url::ParseError),

    #[error("Request error")]
    RequestError(#[from] reqwest::Error),
}

type DownloadList = Vec<Url>;

pub fn concurrent(download_list: DownloadList, concurrent_requests: usize) -> Pin<Box<dyn Stream<Item = Result<reqwest::Response, reqwest::Error>> + Send>> {
    let http_client = Arc::new(reqwest::Client::new());
    stream::iter(download_list)
        .map(move |url| {
            let http_client = http_client.clone();
            async move {
                http_client.get(url.clone()).send().await
            }
        })
        .buffered(concurrent_requests)
        .boxed()
}
