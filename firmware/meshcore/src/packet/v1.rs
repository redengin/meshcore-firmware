// /// https://github.com/meshcore-dev/MeshCore/blob/main/docs/packet_format.md
// #[derive(Debug, zerocopy::FromBytes)]
// struct RawPacket
// {

// }

pub struct Header(pub u8);
impl Header {
    pub fn route_type(&self) -> RouteType {
        RouteType::from(self)
    }
    pub fn payload_type(&self) -> PayloadType {
        PayloadType::from(self)
    }
}

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum RouteType {
    /// Flood mode + Transport codes.
    TransportFlood = 0b00,
    /// Flood mode
    Flood = 0b01,
    /// Direct route
    Direct = 0b10,
    /// Direct route + Transport codes.
    TransportDirect = 0b11,
}
impl RouteType {
    pub fn has_transport_codes(&self) -> bool {
        match self {
            RouteType::Flood => false,
            RouteType::Direct => false,
            RouteType::TransportFlood => true,
            RouteType::TransportDirect => true,
        }
    }
}
impl From<&Header> for RouteType {
    fn from(header: &Header) -> Self {
        const ROUTE_TYPE_SHIFT: u8 = 0;
        const ROUTE_TYPE_MASK: u8 = 0b11;
        let route_flag = (header.0 >> ROUTE_TYPE_SHIFT) & ROUTE_TYPE_MASK;
        if route_flag == RouteType::TransportFlood as u8 {
            return RouteType::TransportFlood;
        }
        if route_flag == RouteType::Flood as u8 {
            return RouteType::Flood;
        }
        if route_flag == RouteType::Direct as u8 {
            return RouteType::Direct;
        }
        if route_flag == RouteType::TransportDirect as u8 {
            return RouteType::TransportDirect;
        }
        panic!("unable to determine packet route type");
    }
}

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum PayloadType {
    Request = 0b0000,
    Response = 0b0001,
    TxtMsg = 0b0010,
    Ack = 0b0011,
    Advert = 0b0100,
    GrpText = 0b0101,
    GrpData = 0b0110,
    AnonReq = 0b0111,
    Path = 0b1000,
    Trace = 0b1001,
    Multipart = 0b1010,
    Control = 0b1011,
    Reserved,
    RawCustom = 0b1111,
}
impl From<&Header> for PayloadType {
    fn from(header: &Header) -> Self {
        const PAYLOAD_TYPE_SHIFT: u8 = 2;
        const PAYLOAD_TYPE_MASK: u8 = 0b1111;
        let payload_type_flag = (header.0 >> PAYLOAD_TYPE_SHIFT) & PAYLOAD_TYPE_MASK;
        if payload_type_flag == PayloadType::Request as u8 {
            return PayloadType::Request;
        }
        if payload_type_flag == PayloadType::Response as u8 {
            return PayloadType::Response;
        }


        panic!("unable to determine packet payload type");
    }
}
