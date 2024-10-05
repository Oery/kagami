use crate::kagami::callbacks::manager::CallbackManager;
use crate::minecraft::packets::handshake::client::SetProtocol;
use crate::serialization::{deserialize, VarIntReader};
use crate::tcp::{utils::RawPacket, AtomicState, Origin, State};

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use tokio::io::AsyncReadExt;

// IDs of packets that may change the state of the connection
const STATE_PACKETS: [i32; 3] = [0x00, 0x02, 0x03];

pub async fn process_packets(
    packets: &mut Vec<RawPacket>,
    is_compression_enabled: &Arc<AtomicBool>,
    s: &Arc<AtomicState>,
    origin: &Origin,
    callbacks: &Arc<CallbackManager>,
) {
    for packet in packets {
        let state = s.load(Ordering::Relaxed);

        let mut packet_data: Vec<u8> = Vec::new();
        let (ref mut _length, ref mut data) = packet;

        let mut reader = std::io::Cursor::new(&data);

        let id = match is_compression_enabled.load(Ordering::Relaxed) {
            true => {
                let _data_length = reader.read_varint().unwrap();
                reader.read_varint().unwrap()
            }
            false => reader.read_varint().unwrap(),
        };

        let _ = reader.read_to_end(&mut packet_data).await;

        // TODO: Those packets should be migrated to the callback system once it's implemented
        if STATE_PACKETS.contains(&id) {
            match id {
                0x00 if state == State::HandShaking && origin == &Origin::Client => {
                    let packet: SetProtocol = deserialize(&packet_data).unwrap();
                    s.store(packet.next_state, Ordering::Relaxed);
                    continue;
                }
                0x02 if state == State::Login => {
                    s.store(State::Play, Ordering::Relaxed);
                    continue;
                }
                0x03 if state == State::Login => {
                    is_compression_enabled.store(true, Ordering::Relaxed);
                    continue;
                }
                _ => {}
            }
        }

        callbacks.handle_packet(id, &packet_data, origin, &state);

        // IF PACKET WAS MODIFIED, GENERATE NEW LENGTH
    }
}
