pub mod callbacks;

use std::sync::Arc;

use crate::kagami::callbacks::manager::CallbackManager;
use crate::minecraft::AnyPacket;
use crate::tcp::connection::handle_client_conn;

use callbacks::Actions;
use tokio::net::TcpListener;

pub struct Kagami {
    pub local_port: u16,
    pub remote_host: std::net::SocketAddr,
    pub callbacks: CallbackManager,
}

impl Kagami {
    pub fn new(remote_host: std::net::SocketAddr) -> Self {
        Self {
            local_port: 25565,
            remote_host,
            callbacks: CallbackManager::default(),
        }
    }

    pub fn register_callback<T: AnyPacket + 'static>(
        &mut self,
        callback: impl Fn(&T) -> Actions + Send + Sync + 'static,
    ) {
        self.callbacks.register(callback);
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let proxy_server = TcpListener::bind(format!("127.0.0.1:{}", self.local_port)).await?;

        let callbacks = std::mem::take(&mut self.callbacks);
        let callbacks = Arc::from(callbacks);

        while let Ok((client, _)) = proxy_server.accept().await {
            let remote_host = self.remote_host;
            let callbacks = callbacks.clone();

            tokio::spawn(async move {
                if let Err(e) = handle_client_conn(client, &remote_host, callbacks).await {
                    eprintln!("Error: {}", e);
                }
            });
        }
        Ok(())
    }

    pub fn builder(remote_host: std::net::SocketAddr) -> KagamiBuilder {
        KagamiBuilder::default(remote_host)
    }
}

pub struct KagamiBuilder {
    local_port: u16,
    remote_host: std::net::SocketAddr,
    callbacks: CallbackManager,
}

impl KagamiBuilder {
    pub fn default(remote_host: std::net::SocketAddr) -> Self {
        Self {
            local_port: 25565,
            remote_host,
            callbacks: CallbackManager::default(),
        }
    }

    pub fn local_port(mut self, local_port: u16) -> Self {
        self.local_port = local_port;
        self
    }

    pub fn build(self) -> Kagami {
        Kagami {
            local_port: self.local_port,
            remote_host: self.remote_host,
            callbacks: self.callbacks,
        }
    }
}
