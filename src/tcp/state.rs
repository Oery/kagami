use atomic_enum::atomic_enum;

// The state of the connection, default is handshake
// Serverbound Handshake -> Set State based on last byte (either login or status)
// Clientbound Login Success -> Set State to Play

#[atomic_enum]
#[derive(Eq, PartialEq)]
pub enum State {
    HandShaking,
    Status,
    Login,
    Play,
}
