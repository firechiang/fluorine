#![warn(rust_2018_idioms)]

use futures::{SinkExt, StreamExt};
use tokio::io;
use tokio::net::TcpStream;
use tokio_util::codec::{BytesCodec, FramedRead, FramedWrite,Framed, LinesCodec};
use std::env;
use std::error::Error;
use std::io::Read;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio::sync::mpsc;

use std::thread;
use std::process::exit;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use std::thread::sleep;
use std::io::Write;
use tokio::io::{AsyncReadExt, AsyncWriteExt};


//use serde_derive::{Serialize, Deserialize};
// use rmp_serde::Serializer;
// use serde::{Serialize, Deserialize};
//
// #[derive(Serialize, Deserialize, Debug)]
// pub struct Message {
//     x: f32,
//     msg: String,
// }
//
// impl Message {
//     pub fn pack(&self) -> Vec<u8> {
//         let mut buf = Vec::new();
//         self.serialize(&mut Serializer::new(&mut buf)).unwrap();
//         buf
//     }
//
//     pub fn unpack(buf: Vec<u8>) -> Message {
//         rmp_serde::from_slice::<Message>(&buf).unwrap()
//     }
// }
//
// fn main() {
//     let msg = Message {
//         x: 20.0,
//         msg: "sdfsdfsdfsd".to_string()
//     };
//     let byte = msg.pack();
//     let m = Message::unpack(byte);
//     //println!("{:#?}",byte);
//     println!("{:#?}",m);
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:9060").await?;
    let(mut ri,mut wi) = tokio::io::split(stream);
    //let(mut ri,mut wi) = stream.split();
    let mut stream_read = FramedRead::new(ri,LinesCodec::new());
    let mut stream_write = FramedWrite::new(wi,LinesCodec::new());
    // 发送名称
    stream_write.send("tiantan").await?;
    // 定时发送数据
    tokio::spawn(async move  {
        loop {
            // 程序暂停4秒
            sleep(Duration::from_secs(3));
            let timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
            let msg = format!("{:#?}\n1111", timestamp);
            stream_write.send(msg).await;
        }
    });

    // 接收数据
    tokio::spawn(async move {
        loop {
            let data = match stream_read.next().await {
                Some(Ok(line)) => line,
                _ => {
                    println!("没读到数据");
                    String::from("")
                }
            };
            println!("{}",data);
        }
    });
    println!("客户端已启动!");
    sleep(Duration::from_secs(3000));
    Ok(())
}
