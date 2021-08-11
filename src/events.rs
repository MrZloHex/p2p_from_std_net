pub enum Event {
    DiscoverPeers,
    Message(Message)
}

pub struct Message {
    message: String,
    author: String
}
