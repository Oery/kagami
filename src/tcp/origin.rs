#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Origin {
    Client,
    Server,
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
