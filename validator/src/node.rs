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


// Send
pub(crate) type Tx  = mpsc::UnboundedSender<String>;
// Receive
pub(crate) type Rx = mpsc::UnboundedReceiver<String>;

pub(crate) struct PeerContext {
    pub(crate) peers: HashMap<SocketAddr,Tx>,
}

impl PeerContext {
    pub(crate) fn new() -> Self {
        PeerContext {
            peers: HashMap::new(),
        }
    }

    // 广播消息
    pub(crate) async fn broadcast(&mut self,sender: SocketAddr,message: &str) {
        // 遍历所有节点信息
        for peer in self.peers.iter_mut() {
            // 如果节点不是发送者就将信息转发给它（注意：*peer.0 取的就是 Tuple结构的第一个值）
            if *peer.0 != sender {
                let _ = peer.1.send(message.into());
            }
        }
    }
}

pub(crate) struct Peer {
    pub(crate) lines: Framed<TcpStream,LinesCodec>,
    pub(crate) rx: Rx,
}

impl Peer {
    pub(crate) async fn new(peer_context: Arc<Mutex<PeerContext>>,lines: Framed<TcpStream,LinesCodec>)-> io::Result<Peer> {
        let addr = lines.get_ref().peer_addr()?;
        // May cause memory overflow
        let(tx,rx) = mpsc::unbounded_channel();
        peer_context.lock().await.peers.insert(addr,tx);
        Ok(Peer {
            lines,rx
        })
    }
}