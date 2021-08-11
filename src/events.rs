pub enum Event {
    DiscoverPeers,
    MessagePeer(Message)
}

pub struct Message {
    message: String,
    author: String
}
