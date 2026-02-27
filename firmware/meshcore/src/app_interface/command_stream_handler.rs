use crate::app_interface::companion_protocol::{CommandPacket, CommandPacketType};
use ringbuf::traits::{Consumer, Producer};
use zerocopy::TryFromBytes;

const BUFFER_SIZE: usize = 1024;
/// Handle streaming data (BLE RX, USB Serial, etc.)
pub struct CommandStreamHandler {
    /// using non-threadsafe implementation (as only used per interface thread)
    buffer: ringbuf::LocalRb<ringbuf::storage::Array<u8, BUFFER_SIZE>>,
}
impl CommandStreamHandler {
    pub fn new() -> Self {
        Self {
            buffer: ringbuf::LocalRb::default(),
        }
    }

    pub fn write(&mut self, buf: &[u8]) -> usize {
        self.buffer.push_slice(buf)
    }

    pub fn next_command(&mut self) -> Option<CommandPacket> {
        return loop {
            // see if the first readable byte is a packet type identifier
            // match self.buffer.try_peek()
            // {
            //     None => break None,
            //     Some(first_byte) => {
            //         let packet_type = CommandPacketType::from
            //         // match first_byte {
            //         //     CommandPacketType::AppStart => {}

            //         //     _ => {
            //         //         // drop the byte as it doesn't match any known packet type
            //         //         self.buffer.skip(1);
            //         //     }
            //         // }
            //     }
            // }
            break None;
        };
    }
}

#[cfg(test)]
mod command_stream_handler_tests {

    // use super::*;
    // use zerocopy::{IntoBytes, TryFromBytes};

    use crate::app_interface::{CommandStreamHandler, companion_protocol::CommandPacket};

    #[test]
    fn test_app_start() {
        // from https://github.com/meshcore-dev/MeshCore/blob/main/docs/companion_protocol.md#1-app-start
        const APP_START_DATA: [u8; 11] = [0x01, 0x03, 0x6d, 0x63, 0x63, 0x6c, 0x69, 0, 0, 0, 0];

        let mut handler = CommandStreamHandler::new();
        assert_eq!(APP_START_DATA.len(), handler.write(&APP_START_DATA));

        match handler.next_command() {
            Some(p) => assert_eq!(CommandPacket::AppStart, p),
            None => assert!(false, "Failed to find any command"),
        }
    }
}
