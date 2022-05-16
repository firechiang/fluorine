#![warn(rust_2018_idioms)]

use futures::{SinkExt, StreamExt};
use tokio::io;
use tokio::net::TcpStream;
use tokio_util::codec::{BytesCodec, FramedRead, FramedWrite,Framed, LinesCodec,LengthDelimitedCodec};
use std::env;
use std::error::Error;
use std::io::{Bytes, Read};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio::sync::mpsc;

use std::thread;
use std::process::exit;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use std::thread::sleep;
use std::io::Write;
use bytes::BytesMut;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:9060").await?;
    let(mut ri,mut wi) = tokio::io::split(stream);
    //let(mut ri,mut wi) = stream.split();
    let mut stream_read = FramedRead::new(ri,LengthDelimitedCodec::new());
    let mut stream_write = FramedWrite::new(wi,LengthDelimitedCodec::new());
    // 发送名称
    stream_write.send(bytes::Bytes::from("maomao")).await?;
    // 定时发送数据
    tokio::spawn(async move  {
        loop {
            // 程序暂停4秒
            sleep(Duration::from_secs(3));
            let timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
            let msg = format!("{:#?}", timestamp);
            stream_write.send(bytes::Bytes::from(msg)).await;
        }
    });

    // 接收数据
    tokio::spawn(async move {
        loop {
            let data = match stream_read.next().await {
                Some(Ok(line)) => line,
                _ => {
                    println!("没读到数据");
                    //String::from("")
                    BytesMut::from("")
                }
            };
            println!("{:#?}",data);
        }
    });
    println!("客户端已启动!");
    sleep(Duration::from_secs(3000));
    Ok(())
}
