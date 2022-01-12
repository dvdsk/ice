pub mod download;
pub use download::Progress;
use futures::TryStream;

#[derive(Debug)]
pub enum Error {
    Empty,
}

pub async fn download_stream(
) -> Result<impl TryStream<Ok = Progress, Error = download::Error>, Error> {
    Ok(download::unpack_stream().await.unwrap())
}
