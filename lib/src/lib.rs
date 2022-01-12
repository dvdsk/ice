use futures::{stream, StreamExt, TryStream, TryStreamExt};

#[derive(Debug)]
pub enum E {
    Empty,
}
pub struct P;

pub async fn unpack_stream() -> Result<impl TryStream<Ok = P, Error = E>, E> {
    let state = ();

    let stream = stream::try_unfold(state, state_machine);
    // this is needed as try_next needs Pin<TryStream> an TryStream is
    // not implemented for Pin<TryStream>
    Ok(stream.into_stream().boxed())
}

async fn state_machine(_: ()) -> Result<Option<(P, ())>, E> {
    Ok(None)
}

