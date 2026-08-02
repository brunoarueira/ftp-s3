use aws_config::BehaviorVersion;
use aws_sdk_s3 as s3;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::primitives::SdkBody;
use futures_util::TryStreamExt;
use hyper::Body;
use suppaftp::tokio::AsyncDataStream;
use tokio_util::codec;

const ENDPOINT: &str = "http://localhost:4566";
const BUCKET_NAME: &str = "sync";
const REGION: &str = "local";

pub async fn save_to_s3<
    T: std::marker::Send + suppaftp::tokio::TokioTlsStream + std::marker::Sync + 'static,
>(
    entry: &str,
    buffer: AsyncDataStream<T>,
) -> Result<(), s3::Error> {
    let config = aws_config::defaults(BehaviorVersion::latest())
        .region(REGION)
        .endpoint_url(ENDPOINT)
        .load()
        .await;
    let client = s3::Client::new(&config);

    let framed_stream = codec::FramedRead::new(buffer, codec::BytesCodec::new())
        .map_ok(|bytes_mut| bytes_mut.freeze());

    let body_impl = Body::wrap_stream(framed_stream);

    let sdk_body = SdkBody::from_body_0_4(body_impl);

    let byte_stream = ByteStream::new(sdk_body);

    let response = client
        .put_object()
        .bucket(BUCKET_NAME)
        .key(entry)
        .body(byte_stream)
        .send()
        .await?;

    println!(
        "Upload with success, Version ID: {:?}",
        response.version_id()
    );

    Ok(())
}
