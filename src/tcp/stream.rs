use crate::{
    kagami::callbacks::manager::CallbackManager,
    tcp::{processor::process_packets, utils::split_packets, AtomicState, Origin},
};

use std::sync::{atomic::AtomicBool, Arc};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    sync::mpsc::{Receiver, Sender},
};

pub async fn handle_stream(
    mut stream: TcpStream,
    mut rx: Receiver<Vec<u8>>,
    tx: Sender<Vec<u8>>,
    origin: Origin,
    s: Arc<AtomicState>,
    is_compression_enabled: Arc<AtomicBool>,
    callbacks: Arc<CallbackManager>,
) -> std::io::Result<()> {
    let (mut reader, mut writer) = stream.split();
    let mut buf = vec![0; 16384]; // Stores raw packets
    let mut partial_buf = Vec::new(); // Stores partial packets

    loop {
        tokio::select! {
            result = reader.read(&mut buf) => {
                match result {
                    Ok(0) => break, // EOF
                    Ok(n) => {
                        let mut raw_packets = split_packets(buf[..n].to_vec(), &mut partial_buf).unwrap();
                        // Direct buffer transmission
                        // if tx.send(buf[..n].to_vec()).await.is_err() {
                        //     eprintln!("Failed to send data from {}", origin);
                        //     break;
                        // }

                        process_packets(&mut raw_packets, &is_compression_enabled, &s, &origin, &callbacks).await;

                        let sent_packets: Vec<u8> = raw_packets.into_iter()
                            .flat_map(|(length, data)| length.into_iter().chain(data))
                            .collect();

                        if tx.send(sent_packets).await.is_err() {
                            eprintln!("Failed to send data from {}", origin);
                            break;
                        };

                    }
                    Err(e) => {
                        eprintln!("Error reading from {}: {:?}", origin, e);
                        return Err(e);
                    }
                }
            }
            Some(data) = rx.recv() => {
                if let Err(e) = writer.write_all(&data).await {
                    eprintln!("Error writing to {}: {:?}", origin, e);
                    return Err(e);
                }
            }
            else => break,
        }
    }

    Ok(())
}
