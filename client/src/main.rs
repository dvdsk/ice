use std::hash::{Hash, Hasher};
use std::pin::Pin;

use futures::stream::{self, BoxStream};
use futures::{Stream, TryStreamExt};

pub fn main() {
    iced::Subscription::from_recipe(JavaSetup {});
}

pub struct JavaSetup {}

#[derive(Default)]
pub struct State {
    stream: Option<Pin<Box<dyn Stream<Item = Result<localjava::Progress, localjava::Error>>>>>,
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
    type Output = ();

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
                        Some(((), state))
                    }
                    Err(error) => Some(((), state)),
                },
                Streaming => match state.stream.unwrap().try_next().unwrap().await {
                    _ => todo!(),
                },
            }
        }))
    }
}
