use bincode::{DefaultOptions, Options};
use bytes::{BufMut, BytesMut};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::io::{self, Read};
use std::marker::PhantomData;
use tokio_util::codec::{Decoder, Encoder};

/// Bincode based codec for use with `tokio-codec`
pub struct BinCodec<T> {
    config: DefaultOptions,
    _pd: PhantomData<T>,
}

impl<T> BinCodec<T> {
    /// Provides a bincode based codec
    pub fn new() -> Self {
        let config = bincode::options();
        BinCodec::with_config(config)
    }

    /// Provides a bincode based codec from the bincode config
    pub fn with_config(config: DefaultOptions) -> Self {
        BinCodec {
            config,
            _pd: PhantomData,
        }
    }
}

impl<T> Decoder for BinCodec<T>
    where
            for<'de> T: Deserialize<'de>,
{
    type Item = T;
    type Error = bincode::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if !buf.is_empty() {
            let mut reader = Reader::new(&buf[..]);
            let message = self.config.deserialize_from(&mut reader)?;
            buf.split_to(reader.amount());
            Ok(Some(message))
        } else {
            Ok(None)
        }
    }
}

impl<T> Encoder<T> for BinCodec<T>
    where
        T: Serialize,
{
    //type Item = T;
    type Error = bincode::Error;

    fn encode(&mut self, item: T, buf: &mut BytesMut) -> Result<(), Self::Error> {
        let size = self.config.serialized_size(&item)?;
        buf.reserve(size as usize);
        let message = self.config.serialize(&item)?;
        buf.put(&message[..]);
        Ok(())
    }
}

impl<T> fmt::Debug for BinCodec<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("BinCodec").finish()
    }
}

#[derive(Debug)]
struct Reader<'buf> {
    buf: &'buf [u8],
    amount: usize,
}

impl<'buf> Reader<'buf> {
    pub fn new(buf: &'buf [u8]) -> Self {
        Reader { buf, amount: 0 }
    }

    pub fn amount(&self) -> usize {
        self.amount
    }
}

impl<'buf, 'a> Read for &'a mut Reader<'buf> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let bytes_read = self.buf.read(buf)?;
        self.amount += bytes_read;
        Ok(bytes_read)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::{Future, Sink, Stream};
    use std::net::SocketAddr;
    use tokio::{
        net::{TcpListener, TcpStream},
    };
    use tokio_util::codec::Framed;

    #[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
    enum Mock {
        One,
        Two,
    }

    #[test]
    fn it_works() {
        println!("sadas");
    }
}