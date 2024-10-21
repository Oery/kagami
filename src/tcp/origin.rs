#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Origin {
    Client,
    Server,
}

impl Origin {
    pub fn from_module_path(module: &str) -> Origin {
        if module.contains("client") {
            return Origin::Client;
        } else if module.contains("server") {
            return Origin::Server;
        }

        panic!("A Packet must be defined in either a client or server module");
    }
}

impl std::fmt::Display for Origin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Origin::Client => "Client",
            Origin::Server => "Server",
        };
        write!(f, "{}", s)
    }
}
