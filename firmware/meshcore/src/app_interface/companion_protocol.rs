// use log::*;
// const TAG: &str = "Command Protocol";

/// TODO find design reference
// const MAX_MESSAGE_LENGTH: usize = 200;

/// Immplemented per zero-copy semantics (i.e. the object is mapped to the underlying buffer,
/// rather than copied onto the stack)
///
/// Guided by
/// https://github.com/meshcore-dev/MeshCore/blob/main/docs/companion_protocol.md#commands
///
/// additional info from
/// https://github.com/andrewdavidmackenzie/meshcore-rs/blob/master/src/commands/base.rs#L18
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, PartialEq)]
pub enum CommandPacket<'buffer> {
    AppStart {
        /// b"0x03" per design
        magic: &'buffer u8,
        /// b"mccli" padded with b'\0' per design
        label: &'buffer [u8; 9],
    },

    // SendChannelTxtMessage - TODO not described in public protocol
    SendChannelMessage {
        /// b"0x00" per design
        magic: &'buffer u8,
        channel_index: &'buffer u8,
        /// seconds since epoch (little-endian)
        timestamp: Option<chrono::DateTime<chrono::Utc>>,
        /// utf-8 of variable length
        // message: &'buffer [u8; MAX_MESSAGE_LENGTH],
        message: &'buffer [u8],
    },
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, PartialEq)]
pub enum CommandPacketError {
    EmptyBuffer,

    /// first byte used to determine packet type, if no match, this error is returned
    InvalidFirstByte,

    /// buffer doesn't contain enough for the found packet type
    IncompleteBuffer,

    /// no implementation for the packet
    NotImplemented,
}
impl<'buffer> CommandPacket<'buffer> {
    pub fn from_bytes(
        bytes: &'buffer [u8],
    ) -> Result<
        (CommandPacket<'buffer>, usize /* used bytes */),
        (CommandPacketError, usize /* used bytes */),
    > {
        if bytes.len() < 1 {
            return Err((CommandPacketError::EmptyBuffer, 0));
        }
        use zerocopy::TryFromBytes;
        match CommandPacketType::try_ref_from_prefix(bytes) {
            Err(_) => {
                return Err((CommandPacketError::InvalidFirstByte, 1));
            }

            Ok((packet_type, _)) => {
                match packet_type {
                    &CommandPacketType::AppStart => {
                        const LEN: usize = 11;
                        if bytes.len() < LEN {
                            return Err((CommandPacketError::IncompleteBuffer, 0));
                        }
                        return Ok((
                            CommandPacket::AppStart {
                                magic: &bytes[1],
                                label: bytes[2..11].try_into().unwrap(),
                            },
                            LEN,
                        ));
                    }

                    &CommandPacketType::SendChannelTxtMessage => {
                        // if we haven't implemented the packet comprehender,
                        // treat it like an illegal header
                        return Err((CommandPacketError::NotImplemented, 1));
                    }

                    &CommandPacketType::SendChannelMessage => {
                        const MIN_LEN: usize = 8; // must be atleast one message byte
                        if bytes.len() < MIN_LEN {
                            return Err((CommandPacketError::IncompleteBuffer, 0));
                        }

                        // for usability, convert the timestamp data to a DateTime
                        let timestamp_epoch_s = i32::from_le_bytes(bytes[3..7].try_into().unwrap());
                        let timestamp = chrono::DateTime::from_timestamp_secs(timestamp_epoch_s as i64);

                        return Ok((
                            CommandPacket::SendChannelMessage {
                                magic: &bytes[1],
                                channel_index: &bytes[2],
                                timestamp: timestamp,
                                // WTF - as there is no size parameter, assume we consume full buffer?
                                message: &bytes[(MIN_LEN - 1)..],
                            },
                            // as there is no message size parameter, consume everything
                            // NOTE - async producers will fail if they don't write full frames
                            bytes.len()
                        ));
                    }

                    // if we haven't implemented the packet comprehender,
                    // treat it like an illegal header
                    _ => return Err((CommandPacketError::NotImplemented, 1)),
                }
            }
        }
    }
}

#[cfg(test)]
mod command_packet_tests {

    use crate::app_interface::companion_protocol::CommandPacket;

    #[test]
    fn test_app_start_from_bytes() {
        const TEST_VECTOR: [u8; 11] = [0x01, 0x03, 0x6d, 0x63, 0x63, 0x6c, 0x69, 0, 0, 0, 0];
        match CommandPacket::from_bytes(&TEST_VECTOR) {
            Ok((packet, used)) => match packet {
                CommandPacket::AppStart { magic, label } => {
                    assert_eq!(TEST_VECTOR.len(), used);
                    assert_eq!(0x03, *magic);
                    assert_eq!(TEST_VECTOR[2..11], *label);
                }

                _ => panic!("failed to comprehend APP_START packet"),
            },
            Err(e) => panic!("unable to parse APP_START - {:?}", e),
        }
    }

    #[test]
    fn test_send_channel_message_from_bytes() {
        const TEST_VECTOR: [u8; 12] = [
            0x03, 0x00, 0x01, 0xD2, 0x02, 0x96, 0x49, 0x48, 0x65, 0x6C, 0x6C, 0x6F,
        ];
        match CommandPacket::from_bytes(&TEST_VECTOR) {
            Ok((packet, used)) => match packet {
                CommandPacket::SendChannelMessage {
                    magic,
                    channel_index,
                    timestamp,
                    message,
                } => {
                    assert_eq!(TEST_VECTOR.len(), used);
                    assert_eq!(0x00, *magic);
                    assert_eq!(1, *channel_index);
                    assert_eq!(chrono::DateTime::from_timestamp_secs(1234567890).unwrap(), timestamp.expect("invalid time value"));
                    assert_eq!(TEST_VECTOR[7..], *message);
                }

                _ => panic!("failed to comprehend APP_START packet"),
            },
            Err(e) => panic!("unable to parse APP_START - {:?}", e),
        }
    }
}

/// https://github.com/meshcore-dev/MeshCore/blob/main/docs/companion_protocol.md#commands
///
/// additional info from
/// https://github.com/andrewdavidmackenzie/meshcore-rs/blob/master/src/commands/base.rs#L18
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, PartialEq)]
#[repr(u8)]
#[derive(
    zerocopy::Immutable,
    zerocopy::KnownLayout,
    // zerocopy::Unaligned,
    zerocopy::TryFromBytes,
    zerocopy::IntoBytes,
)]
pub enum CommandPacketType {
    AppStart = 1,
    SendChannelTxtMessage = 2,
    SendChannelMessage = 3,
    GetContacts = 4,
    GetDeviceTime = 5,
    SetDeviceTime = 6,
    SendSelfAdvert = 7,
    SetAdvertName = 8,
    AddUpdateContact = 9,
    SyncNextMessage = 10,
    SetRadioParams = 11,
    SetRadioTxPower = 12,
    RestPath = 13,
    SetAdvertLatLon = 14,
    RemoveContact = 15,
    ShareContact = 16,
    ExportContact = 17,
    ImportContact = 18,
    Reboot = 19,
    GetBattaryAndStorage = 20,
    SetTuningParameters = 21,
    DeviceQuery = 22,
    ExportPrivateKey = 23,
    ImportPrivateKey = 24,
    SendRawData = 25,
    SendLogin = 26,
    SendStatusReq = 27,
    HasConnection = 28,
    Logout = 29,
    GetContactByKey = 30,
    GetChannel = 31,
    SetChannel = 32,
    SignStart = 33,
    SignData = 34,
    SignFinish = 35,
    GetCustomVars = 40,
    SetCustomVar = 41,
    SendBinaryReq = 50,
}

#[cfg(test)]
mod command_type_tests {

    use crate::app_interface::companion_protocol::CommandPacketType;
    use zerocopy::{IntoBytes, TryFromBytes};

    #[test]
    fn test_type_from_bytes() {
        // test that zerocopy does the right thing
        let command = CommandPacketType::AppStart;
        let bytes = command.as_bytes();
        match CommandPacketType::try_ref_from_prefix(bytes) {
            Ok(p) => assert_eq!(*p.0, CommandPacketType::AppStart),
            Err(e) => panic!("zerocopy is broken, couldn't determine Ack packet type - {e}"),
        }

        // non-exhaustive - if zerocopy does it right once, it should
        //  do the proper thing for all values

        let illegal_bytes: [u8; 1] = [0xFF];
        match CommandPacketType::try_ref_from_prefix(&illegal_bytes) {
            Ok(p) => panic!("zerocopy is broken, illegal packet type returned {:?}", p),
            Err(_) => (/* ignored */),
        }
    }
}

/// https://github.com/meshcore-dev/MeshCore/blob/main/docs/companion_protocol.md#packet-types
///
/// additional info from
/// https://github.com/andrewdavidmackenzie/meshcore-rs/blob/master/src/commands/base.rs#L18
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, PartialEq)]
#[repr(u8)]
#[derive(
    zerocopy::Immutable,
    zerocopy::KnownLayout,
    // zerocopy::Unaligned,
    zerocopy::TryFromBytes,
    zerocopy::IntoBytes,
)]
pub enum ResponsePacketType {
    Ok = 0,
    Error = 1,
    ContactStart = 2,
    Contact = 3,
    ContactEnd = 4,
    SelfInfo = 5,
    MsgSent = 6,
    ContactMsgRecv = 7,
    ChannelMsgRecv = 8,
    CurrentTime = 9,
    NoMoreMsgs = 10,
    ContactUri = 11,
    Battery = 12,
    DeviceInfo = 13,
    PrivateKey = 14,
    Disabled = 15,
    ContactMsgRecvV3 = 16,
    ChannelMsgRecvV3 = 17,
    ChannelInfo = 18,
    SignStart = 19,
    Signature = 20,
    CustomVars = 21,
    Stats = 24,
    AutoaddConfig = 25,

    // Binary/Control (50-55)
    BinaryReq = 50,
    FactoryReset = 51,
    PathDiscovery = 52,
    SetFloodScope = 54,
    SendControlData = 55,

    // Push notifications (0x80-0x8F)
    Advertisement = 0x80,
    PathUpdate = 0x81,
    Ack = 0x82,
    MessagesWaiting = 0x83,
    RawData = 0x84,
    LoginSuccess = 0x85,
    LoginFailed = 0x86,
    StatusResponse = 0x87,
    LogData = 0x88,
    TraceData = 0x89,
    PushCodeNewAdvert = 0x8A,
    TelemetryResponse = 0x8B,
    BinaryResponse = 0x8C,
    PathDiscoveryResponse = 0x8D,
    ControlData = 0x8E,
    AdvertResponse = 0x8F,
    // Unknown packet type
    // Unknown = 0xFF,
}

#[cfg(test)]
mod response_type_tests {

    use crate::app_interface::companion_protocol::ResponsePacketType;
    use zerocopy::{IntoBytes, TryFromBytes};

    #[test]
    fn test_type_from_bytes() {
        // test that zerocopy does the right thing
        let response = ResponsePacketType::Ack;
        let bytes = response.as_bytes();
        match ResponsePacketType::try_ref_from_prefix(bytes) {
            Ok(p) => assert_eq!(*p.0, ResponsePacketType::Ack),
            Err(e) => panic!("zerocopy is broken, couldn't determine Ack packet type - {e}"),
        }

        // non-exhaustive - if zerocopy does it right once, it should
        //  do the proper thing for all values

        let illegal_bytes: [u8; 1] = [0xFF];
        match ResponsePacketType::try_ref_from_prefix(&illegal_bytes) {
            Ok(p) => panic!("zerocopy is broken, illegal packet type returned {:?}", p),
            Err(_) => (/* ignored */),
        }
    }
}
