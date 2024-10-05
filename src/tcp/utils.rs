use crate::serialization::VarIntReader;

pub type RawPacket = (Vec<u8>, Vec<u8>);

pub fn split_packets(
    buffer: Vec<u8>,
    partial_buf: &mut Vec<u8>,
) -> std::io::Result<Vec<RawPacket>> {
    let mut packets = Vec::new();
    let mut b = partial_buf.clone();
    b.extend(&buffer);
    partial_buf.clear();

    let mut reader = std::io::Cursor::new(b);

    while reader.position() < reader.get_ref().len() as u64 {
        let start_pos = reader.position() as usize;

        match reader.read_varint_full() {
            Ok((packet_length, length_bytes)) => {
                let end_pos = reader.position() as usize;
                let total_length = (end_pos - start_pos) + packet_length as usize;

                if start_pos + total_length > reader.get_ref().len() {
                    // Partial packet, add to partial_buf
                    partial_buf.extend(&reader.get_ref()[start_pos..]);
                    break;
                }

                // Complete packet, add to packets
                let data = reader.get_ref()
                    [(start_pos + length_bytes.len())..start_pos + total_length]
                    .to_vec();
                packets.push((length_bytes, data));

                // Move the cursor to the next packet
                reader.set_position((start_pos + total_length) as u64);
            }
            Err(e) => {
                if e.kind() == std::io::ErrorKind::UnexpectedEof {
                    // Partial VarInt, add remaining bytes to partial_buf
                    partial_buf.extend(&reader.get_ref()[start_pos..]);
                    break;
                } else {
                    return Err(e);
                }
            }
        }
    }

    Ok(packets)
}
