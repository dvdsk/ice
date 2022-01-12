use std::hash::{Hash, Hasher};
use std::pin::Pin;

use futures::stream::{self, BoxStream};
use futures::{Stream, TryStreamExt};

enum Event {
    Empty,
}

pub fn sub(count: usize) -> iced::Subscription<Event> {
    iced::Subscription::from_recipe(JavaSetup {})
}

pub struct JavaSetup {}

#[derive(Default)]
pub struct State {
    stream: Option<
        Pin<Box<dyn Stream<Item = Result<localjava::Progress, localjava::download::Error>>>>,
    >,
    phase: Phase,
}

impl Default for Phase {
    fn default() -> Self {
        Self::Download
    }
}

pub enum Phase {
    Download,
    Streaming,
}

impl<H, I> iced_native::subscription::Recipe<H, I> for JavaSetup
where
    H: Hasher,
{
    type Output = Event;

    fn hash(&self, state: &mut H) {
        struct Marker;
        std::any::TypeId::of::<Marker>().hash(state);
    }

    fn stream(self: Box<Self>, _input: BoxStream<'static, I>) -> BoxStream<'static, Self::Output> {
        Box::pin(stream::unfold(State::default(), move |state| async move {
            match &state.phase {
                Download => match localjava::download_stream().await {
                    Ok(stream) => {
                        let stream = Pin::new(Box::new(stream));
                        state.phase = Phase::Streaming;
                        state.stream = Some(stream);
                        Some((Event::Empty, state))
                    }
                    Err(error) => Some((Event::Empty, state)),
                },
                Streaming => match state.stream.unwrap().try_next().unwrap().await {
                    Some(Ok(e)) => Some((Event::Empty, state)),
                    Some(Err(e)) => Some((Event::Empty, state)),
                    None => Some((Event::Empty, state)),
                },
            }
        }))
    }
}
