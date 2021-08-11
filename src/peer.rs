pub struct Peer {
    peer_address: String,
    listen_port: String,
    peers_in_env: Vec<String>
}

impl Peer {
    pub fn new(ip: String, port: String) -> Peer {
        Peer {
            peer_address: ip,
            listen_port: port,
            peers_in_env: Vec::new()
        }
    }


}
