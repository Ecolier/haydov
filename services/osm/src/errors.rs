use aws_sdk_s3::operation::{complete_multipart_upload::CompleteMultipartUploadError, create_multipart_upload::CreateMultipartUploadError, upload_part::UploadPartError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Malformed URL error")]
    ParseUrlError(#[from] url::ParseError),

    #[error("Request error")]
    RequestError(#[from] reqwest::Error),

    #[error("S3 create multipart upload error")]
    S3CreateMultipartError(#[from] aws_sdk_s3::error::SdkError<CreateMultipartUploadError>),

    #[error("S3 upload part error")]
    S3UploadPartError(#[from] aws_sdk_s3::error::SdkError<UploadPartError>),

    #[error("S3 complete multipart upload error")]
    S3CompleteMultipartError(#[from] aws_sdk_s3::error::SdkError<CompleteMultipartUploadError>),
}