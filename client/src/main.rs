use std::pin::Pin;

use futures::stream;
use futures::Stream;

pub fn main() {}

#[derive(Default)]
pub struct State {
    stream: Option<Pin<Box<dyn Stream<Item = Result<lib::P, lib::E>>>>>,
}

fn stream() {
    Box::pin(stream::unfold(State::default(), move |state| async move {
        let stream = lib::download_stream().await.unwrap();
        let stream = Pin::new(Box::new(stream));
        state.stream = Some(stream);
        Some(((), state))
    }))
}
