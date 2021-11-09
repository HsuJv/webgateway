pub enum ProtocalHandlerOutput {
    Ok,
    WsBuf(Vec<u8>),
    Err(String),
}

pub struct ProtocalHandler<T>
where
    T: ProtocalImpl,
{
    inner: T,
}

impl<T> ProtocalHandler<T>
where
    T: ProtocalImpl,
{
    pub fn new() -> Self {
        Self { inner: T::new() }
    }

    pub fn handle(&mut self, input: &[u8]) -> ProtocalHandlerOutput {
        self.inner.handle(input)
    }
}

pub trait ProtocalImpl {
    fn new() -> Self;
    fn handle(&mut self, input: &[u8]) -> ProtocalHandlerOutput;
}
