pub struct Peer {
    peer_address: String,
    listen_port: String,
    peers_in_env: Vec<String>
}

impl Peer {
    pub fn new(ip: String, port: String) -> Self {
        Peer {
            peer_address: ip,
            listen_port: port,
            peers_in_env: Vec::new()
        }
    }

    pub fn get_ip(&mut self) -> String {
        self.peer_address.clone()
    }

    pub fn get_port(&mut self) -> String {
        self.listen_port.clone()
    }

}
