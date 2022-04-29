use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, Mutex};
use tokio_stream::StreamExt;
use tokio_util::codec::{Framed, LinesCodec};

use futures::SinkExt;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::io;
use std::net::SocketAddr;
use std::sync::Arc;
mod node;

use crate::node::*;

#[macro_use]
extern crate log;

use log::{LevelFilter, Record, Level, Metadata};

struct MyLogger;

impl log::Log for MyLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        // Output to console or file
        println!("{}:{} - {}", record.level(),record.target(),record.args());
    }
    fn flush(&self) {}
}

const LOG: MyLogger = MyLogger {};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    log::set_logger(&LOG);
    log::set_max_level(LevelFilter::Debug);

    let peer_context = Arc::new(Mutex::new(PeerContext::new()));
    let listener = TcpListener::bind("127.0.0.1:9060").await.unwrap();
    let local_addr = listener.local_addr();
    info!("Binding {:#?}",local_addr.unwrap());
    loop {
        let (stream,addr) = listener.accept().await.unwrap();
        info!("New peer node connected {:#?}",addr);
        let peer_context = Arc::clone(&peer_context);
        // Asynchronous processing
        tokio::spawn(async move {
            if let Err(e) = process(peer_context, stream, addr).await {
                error!("New peer node processing {:?}", e);
            }
        });
    }
}





// 处理客户端连接
async fn process(state: Arc<Mutex<PeerContext>>,stream: TcpStream,addr: SocketAddr) -> Result<(), Box<dyn Error>> {
    // 利用行解码器创建数据框架
    let mut lines = Framed::new(stream,LinesCodec::new());
    // 向客户端发送“请输入用户名”
    lines.send("Please enter your username:").await?;

    let username = match lines.next().await {
        Some(Ok(line)) => line,
        _ => {
            println!("没有获取到用户名");
            return Ok(());
        }
    };
    // 创建节点信息
    let mut peer = Peer::new(state.clone(),lines).await?;
    // 新节点加入广播到集群
    {
        let mut state = state.lock().await;
        let msg = format!("节点: {} 加入集群",username);
        state.broadcast(addr,&msg).await;
    }
    // 处理接收数据
    loop {
        tokio::select! {
            // 接收数据将其交给解码器
            Some(msg) = peer.rx.recv() => {
                peer.lines.send(&msg).await?;
            }
            // 解码一行数据
            result = peer.lines.next() => match result {
                // 收到消息后将消息广播给其它几点
                Some(Ok(msg)) => {
                    let mut state = state.lock().await;
                    let msg = format!("{}: {}", username, msg);
                    println!("{}",&msg);
                    // 广播消息
                    state.broadcast(addr, &msg).await;
                }
                // 消息接收错误
                Some(Err(e)) => {
                    println!(
                        "节点 {} 处理接收消息发送错误; error = {:?}",
                        username,
                        e
                    );
                }
                // 没有消息了，或者消息读取完成
                None => break,
            },
        }
    }

    // 节点离开集群清除相关信息
    {
        let mut state = state.lock().await;
        state.peers.remove(&addr);

        let msg = format!("节点: {} 离开集群", username);
        println!("{}", msg);
        state.broadcast(addr, &msg).await;
    }

    Ok(())


}