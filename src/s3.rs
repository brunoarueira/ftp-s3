extern crate rusoto_core;
extern crate rusoto_s3;

use async_ftp::DataStream;
use rusoto_core::{Region};
use rusoto_s3::{S3Client, S3, PutObjectRequest, StreamingBody};
use futures::stream::{TryStreamExt};
use std::error::Error;
use tokio::io::BufReader;
use tokio_util::codec;

const ENDPOINT: &str = "http://localhost:4566";
const BUCKET_NAME: &str = "sync";
const REGION: &str = "local";

lazy_static! {
    static ref S3_CLIENT: S3Client = S3Client::new(Region::Custom {
        name: REGION.into(),
        endpoint: ENDPOINT.into()
    });
}

pub async fn save_to_s3(entry: &str, buffer: BufReader<DataStream>) -> Result<(), Box<dyn Error>> {
    let byte_stream =
        codec::FramedRead::new(buffer, codec::BytesCodec::new())
        .map_ok(|r| r.freeze());

    S3_CLIENT.put_object(PutObjectRequest {
        bucket: BUCKET_NAME.into(),
        key: String::from(entry),
        body: Some(StreamingBody::new(byte_stream)),
        acl: Some("public-read".to_string()),
        ..Default::default()
    }).await.expect(format!("could not upload {}", &entry).as_str());

    Ok(())
}
