use tokio::io::{AsyncRead, AsyncWrite};

pub trait AsyncIO: AsyncRead + AsyncWrite {}
impl<T: AsyncRead + AsyncWrite + ?Sized> AsyncIO for T {}