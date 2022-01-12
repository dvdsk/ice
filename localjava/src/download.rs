mod stream;
pub use stream::unpack_stream;

#[derive(Debug)]
pub enum Error {
    Empty
}

#[derive(Clone)]
pub struct Progress;


struct Download {
}
