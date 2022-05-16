#![warn(rust_2018_idioms)]

use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, Mutex};
use tokio_stream::StreamExt;
use tokio_util::codec::{Framed, LinesCodec,LengthDelimitedCodec};

use futures::SinkExt;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::io;
use std::net::SocketAddr;
use std::sync::Arc;
use bytes::Bytes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 存储所有连接节点数据
    let state = Arc::new(Mutex::new(Shared::new()));
    let listener = TcpListener::bind("127.0.0.1:9060").await.unwrap();
    loop {
        let (stream,addr) = listener.accept().await.unwrap();
        println!("有客户端连接:{:#?}",addr);

        // 克隆存储所有连接节点数据的引用
        let state = Arc::clone(&state);

        // 异步处理
        tokio::spawn(async move {
            if let Err(e) = process(state, stream, addr).await {
                println!("an error occurred; error = {:?}", e);
            }
        });
    }
}

// 消息发送通道
type Tx  = mpsc::UnboundedSender<String>;

// 消息接受通道
type Rx = mpsc::UnboundedReceiver<String>;

// 存储连接节点信息（节点地址和数据发送通道）
struct Shared {
    peers: HashMap<SocketAddr,Tx>,
}

// 全局共享信息用于存储所有连接节点相关信息
impl Shared {
    fn new() -> Self {
        Shared {
            peers: HashMap::new(),
        }
    }

    // 广播消息
    async fn broadcast(&mut self,sender: SocketAddr,message: &str) {
        // 遍历所有节点信息
        for peer in self.peers.iter_mut() {
            // 如果节点不是发送者就将信息转发给它（注意：*peer.0 取的就是 Tuple结构的第一个值）
            if *peer.0 != sender {
                let _ = peer.1.send(message.into());
            }
        }
    }
}

// 一个节点
struct Peer {
    // 一行消息
    lines: Framed<TcpStream,LengthDelimitedCodec>,
    // 消息接收管道
    rx: Rx,
}

impl Peer {
    // 创建节点
    async fn new(state: Arc<Mutex<Shared>>,lines: Framed<TcpStream,LengthDelimitedCodec>)-> io::Result<Peer> {
        let addr = lines.get_ref().peer_addr()?;
        // 创建无界通道，得到发送和接收数据通道（注意：可能导致内存溢出）
        let(tx,rx) = mpsc::unbounded_channel();
        // 将节点的地址和数据发送通道存储到成员变量
        state.lock().await.peers.insert(addr,tx);
        // 返回节点
        Ok(Peer {
            lines,rx
        })
    }
}

// 处理客户端连接
 async fn process(state: Arc<Mutex<Shared>>,stream: TcpStream,addr: SocketAddr) -> Result<(), Box<dyn Error>> {
    // 利用行解码器创建数据框架
    let mut lines = Framed::new(stream,LengthDelimitedCodec::new());
    // 向客户端发送“请输入用户名”
    lines.send(Bytes::from("Please enter your username:")).await?;

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
        let msg = format!("节点: {:#?} 加入集群",username);
        state.broadcast(addr,&msg).await;
    }
    // 处理接收数据
    loop {
        tokio::select! {
            // 接收数据将其交给解码器
            Some(msg) = peer.rx.recv() => {
                peer.lines.send(Bytes::from(msg)).await?;
            }
            // 解码一行数据
            result = peer.lines.next() => match result {
                // 收到消息后将消息广播给其它几点
                Some(Ok(msg)) => {
                    let mut state = state.lock().await;
                    let msg = format!("{:#?}: {:#?}", username, msg);
                    println!("{}",&msg);
                    // 广播消息
                    state.broadcast(addr, &msg).await;
                }
                // 消息接收错误
                Some(Err(e)) => {
                    println!(
                        "节点 {:#?} 处理接收消息发送错误; error = {:?}",
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

        let msg = format!("节点: {:#?} 离开集群", username);
        println!("{}", msg);
        state.broadcast(addr, &msg).await;
    }

    Ok(())


}