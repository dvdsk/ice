use futures::{stream, StreamExt, TryStream, TryStreamExt};

#[derive(Debug)]
pub enum Error {
    Empty,
}

pub async fn download_stream() -> Result<impl TryStream<Ok = Progress, Error = Error>, Error> {
    Ok(unpack_stream().await.unwrap())
}

pub async fn unpack_stream() -> Result<impl TryStream<Ok = Progress, Error = Error>, Error> {
    let state = Download {};

    let stream = stream::try_unfold(state, state_machine);
    // this is needed as try_next needs Pin<TryStream> an TryStream is
    // not implemented for Pin<TryStream> this is due to trait aliasses
    // not yet being stable, and will not be a problem in the future.
    // this line of code can be removed when trait aliasses are stabalized
    // let mut stream = stream.into_stream().boxed();
    Ok(stream.into_stream().boxed())
}

async fn state_machine(_state: Download) -> Result<Option<(Progress, Download)>, Error> {
    Ok(None)
}

#[derive(Clone)]
pub struct Progress;
struct Download {}
