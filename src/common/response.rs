pub type ResponseStream<T> = std::pin::Pin<Box<dyn futures_util::Stream<Item = T> + Send>>;

pub fn empty_stream<T>() -> ResponseStream<T>
where
    T: Send + 'static,
{
    Box::pin(futures_util::stream::empty())
}
