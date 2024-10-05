use crate::kagami::callbacks::manager::CallbackManager;
use crate::tcp::{stream::handle_stream, AtomicState, Origin, State};

use std::sync::{atomic::AtomicBool, Arc};
use tokio::{net::TcpStream, sync::mpsc};

pub async fn handle_client_conn(
    client_conn: TcpStream,
    remote_addr: &std::net::SocketAddr,
    callbacks: Arc<CallbackManager>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mc_state = Arc::new(AtomicState::new(State::HandShaking));
    let is_compression_enabled = Arc::new(AtomicBool::new(false));

    let server_conn = TcpStream::connect(&remote_addr).await?;

    let (tx_client, rx_client) = mpsc::channel::<Vec<u8>>(16384);
    let (tx_server, rx_server) = mpsc::channel::<Vec<u8>>(16384);

    let handle_server = tokio::spawn(handle_stream(
        server_conn,
        rx_server,
        tx_client,
        Origin::Server,
        Arc::clone(&mc_state),
        Arc::clone(&is_compression_enabled),
        Arc::clone(&callbacks),
    ));

    let handle_client = tokio::spawn(handle_stream(
        client_conn,
        rx_client,
        tx_server,
        Origin::Client,
        Arc::clone(&mc_state),
        Arc::clone(&is_compression_enabled),
        Arc::clone(&callbacks),
    ));

    let (server_result, client_result) = tokio::join!(handle_server, handle_client);

    server_result??;
    client_result??;

    Ok(())
}
