// TODO implement as a buffered reciever that emits responses upon comprehension of the stream


/// https://github.com/meshcore-dev/MeshCore/blob/main/docs/companion_protocol.md#commands
pub enum CommandPacketType {
    AppStart = 0x01,
    DeviceQuery = 0x16,
    ChannelInfo = 0x1F,
    SetChannel = 0x20,
    SendChannelMessage = 0x03,
    GetMessage = 0x0A,
    GetBattery = 0x14,
}

/// https://github.com/meshcore-dev/MeshCore/blob/main/docs/companion_protocol.md#packet-types
pub enum ResponsePacketType {
    Ok = 0x00,
    Error = 0x01,
    ContactStart = 0x02,
    Contact = 0x03,
    ContactEnd = 0x04,
    SelfInfo = 0x05,
    MsgSent = 0x06,
    ContactMsgRecv = 0x07,
    ChannelMsgRecv = 0x08,
    CurrentTime = 0x09,
    NoMoreMessages = 0x0A,
    Battery = 0x0C,
    DeviceInfo = 0x0D,
    ContactMsgRecvV3 = 0x10,
    ChannelMsgRecvV3 = 0x11,
    ChannelInfo = 0x12,
    Advertisement = 0x80,
    ACK = 0x82,
    MessagesWaiting = 0x83,
    LogData = 0x88,
}
