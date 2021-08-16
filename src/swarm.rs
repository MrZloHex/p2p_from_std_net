use super::Peer;
use super::ping::Ping;

pub struct Swarm {
    behaviour: Ping,
    peer: Peer
}

impl Swarm {
    pub fn new(peer: Peer, ping: Ping) -> Self {
        Swarm {
            behaviour: ping,
            peer
        }
    }

    pub fn listen(&mut self) {
        let listening_address = format!("{}:{}", self.peer.get_ip(), self.peer.get_port());
        println!("{}",  listening_address);
    }
}
