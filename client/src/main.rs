use futures::{Stream, stream};

pub fn main() {}

struct State (Option<Box<dyn Stream<Item = Result<lib::P, lib::E>>>>);

fn stream() {
    Box::pin(stream::unfold(State(None), move |state| async move {
        let stream = lib::unpack_stream().await.unwrap();
        let stream = Box::new(stream);
        state.0 = Some(stream);
        Some(((), state))
    }))
}
