pub type ResponseStream<T> = futures_util::stream::Empty<T>;

pub fn empty_stream<T>() -> ResponseStream<T> {
    futures_util::stream::empty()
}
