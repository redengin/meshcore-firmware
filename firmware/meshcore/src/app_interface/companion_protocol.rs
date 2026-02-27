/// https://github.com/meshcore-dev/MeshCore/blob/main/docs/companion_protocol.md#commands
///
/// additional info from
/// https://github.com/andrewdavidmackenzie/meshcore-rs/blob/master/src/commands/base.rs#L18
#[derive(Debug, PartialEq)]
pub enum CommandPacket
{
    AppStart
}




/// https://github.com/meshcore-dev/MeshCore/blob/main/docs/companion_protocol.md#commands
///
/// additional info from
/// https://github.com/andrewdavidmackenzie/meshcore-rs/blob/master/src/commands/base.rs#L18
#[derive(
    Debug,
    PartialEq,
    zerocopy::KnownLayout,
    zerocopy::Unaligned,
    zerocopy::Immutable,
    zerocopy::TryFromBytes,
    zerocopy::IntoBytes,
)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
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

    use super::*;
    use zerocopy::{IntoBytes, TryFromBytes};

    #[test]
    fn test_type_from_bytes() {
        // test that zerocopy does the right thing
        let command = CommandPacketType::AppStart;
        let bytes = command.as_bytes();
        match CommandPacketType::try_ref_from_prefix(bytes) {
            Ok(p) => {
                assert_eq!(*p.0, CommandPacketType::AppStart);
            }
            Err(_) => panic!("zerocopy is broken"),
        }

        // non-exhaustive - if zerocopy does it right once, it should
        //  do the proper thing for all values
    }
}

/// https://github.com/meshcore-dev/MeshCore/blob/main/docs/companion_protocol.md#packet-types
///
/// additional info from
/// https://github.com/andrewdavidmackenzie/meshcore-rs/blob/master/src/commands/base.rs#L18
#[derive(
    Debug,
    PartialEq,
    zerocopy::KnownLayout,
    zerocopy::Unaligned,
    zerocopy::Immutable,
    zerocopy::TryFromBytes,
    zerocopy::IntoBytes,
)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
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

    /// Unknown packet type
    Unknown = 0xFF,
}

#[cfg(test)]
mod response_type_tests {

    use super::*;
    use zerocopy::{IntoBytes, TryFromBytes};

    #[test]
    fn test_type_from_bytes() {
        // test that zerocopy does the right thing
        let response = ResponsePacketType::Ack;
        let bytes = response.as_bytes();
        match ResponsePacketType::try_ref_from_prefix(bytes) {
            Ok(p) => {
                assert_eq!(*p.0, ResponsePacketType::Ack);
            }
            Err(_) => panic!("zerocopy is broken"),
        }

        // non-exhaustive - if zerocopy does it right once, it should
        //  do the proper thing for all values
    }
}
