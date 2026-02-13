// Implemented per v1 of  https://github.com/meshcore-dev/MeshCore/blob/main/docs/packet_format.md



// #[derive(Debug, zerocopy::FromBytes)]
// struct Packet
// {

// }

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Header(pub u8);
impl Header {
    pub fn version(&self) -> Version {
        Version::from(self)
    }

    pub fn payload_type(&self) -> PayloadType {
        PayloadType::from(self)
    }

    pub fn route_type(&self) -> RouteType {
        RouteType::from(self)
    }
}

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum Version {
    V1 = 0b00,
    V2 = 0b01,
    V3 = 0b10,
    V4 = 0b11,
}
impl From<&Header> for Version {
    fn from(header: &Header) -> Self {
        const VERSION_SHIFT: u8 = 6;
        const VERSION_MASK: u8 = 0b1111;
        let version_flag = (header.0 >> VERSION_SHIFT) & VERSION_MASK;
        if version_flag == Version::V1 as u8 {
            return Version::V1;
        }
        if version_flag == Version::V2 as u8 {
            return Version::V2;
        }
        if version_flag == Version::V3 as u8 {
            return Version::V3;
        }
        if version_flag == Version::V4 as u8 {
            return Version::V4;
        }
        panic!("UNREACHABLE - unable to process packet version")
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
        if payload_type_flag == PayloadType::TxtMsg as u8 {
            return PayloadType::TxtMsg;
        }
        if payload_type_flag == PayloadType::Ack as u8 {
            return PayloadType::Ack;
        }
        if payload_type_flag == PayloadType::Advert as u8 {
            return PayloadType::Advert;
        }
        if payload_type_flag == PayloadType::GrpText as u8 {
            return PayloadType::GrpText;
        }
        if payload_type_flag == PayloadType::GrpData as u8 {
            return PayloadType::GrpData;
        }
        if payload_type_flag == PayloadType::AnonReq as u8 {
            return PayloadType::AnonReq;
        }
        if payload_type_flag == PayloadType::Path as u8 {
            return PayloadType::Path;
        }
        if payload_type_flag == PayloadType::Trace as u8 {
            return PayloadType::Trace;
        }
        if payload_type_flag == PayloadType::Multipart as u8 {
            return PayloadType::Multipart;
        }
        if payload_type_flag == PayloadType::Control as u8 {
            return PayloadType::Control;
        }
        if payload_type_flag == PayloadType::RawCustom as u8 {
            return PayloadType::RawCustom;
        }

        return PayloadType::Reserved;
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
        panic!("UNREACHABLE - unable to determine packet route type")
    }
}
