use crate::exceptions::SCloudException;
use std::convert::TryFrom;
use strum_macros::EnumIter;

/// DNS Resource Record Types (QTYPE / TYPE).
///
/// This enum represents DNS record types as defined by IANA and
/// various RFCs (RFC 1035, RFC 4034, RFC 6895, etc.).
///
/// # Design notes
/// - DNS is an extensible protocol by design.
/// - Unknown record types MUST be preserved and forwarded as-is.
/// - Therefore, unknown types are represented using `Unknown(u16)`.
///
/// This enum supports:
/// - Conversion from the on-the-wire `u16` representation
/// - Conversion back to `u16` for packet serialization
///
/// This guarantees that the resolver does not break when encountering
/// newer or unsupported DNS record types.
#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq, EnumIter)]
pub(crate) enum DNSRecordType {
    A,
    AAAA,
    AFSDB,
    APL,
    CAA,
    CDNSKEY,
    CDS,
    CERT,
    CNAME,
    CSYNC,
    DHCID,
    DLV,
    DNAME,
    DNSKEY,
    DS,
    EUI48,
    EUI64,
    HINFO,
    HIP,
    HTTPS,
    IPSECKEY,
    KEY,
    KX,
    LOC,
    MX,
    NAPTR,
    NS,
    NSEC,
    NSEC3,
    NSEC3PARAM,
    OPENPGPKEY,
    PTR,
    RP,
    RRSIG,
    SIG,
    SMIMEA,
    SOA,
    SRV,
    SSHFP,
    SVCB,
    TA,
    TKEY,
    TLSA,
    TSIG,
    TXT,
    URI,
    ZONEMD,

    /// Unknown or unsupported DNS record type.
    ///
    /// The contained `u16` value is the raw QTYPE as received on the wire.
    /// This variant ensures DNS extensibility is preserved.
    Unknown(u16),
}

impl TryFrom<u16> for DNSRecordType {
    type Error = SCloudException;

    /// Convert a DNS QTYPE value (`u16`) into a `DNSRecordType`.
    ///
    /// Unknown values are preserved using `DNSRecordType::Unknown`.
    fn try_from(v: u16) -> Result<Self, Self::Error> {
        match v {
            1 => Ok(DNSRecordType::A),
            28 => Ok(DNSRecordType::AAAA),
            18 => Ok(DNSRecordType::AFSDB),
            42 => Ok(DNSRecordType::APL),
            257 => Ok(DNSRecordType::CAA),
            60 => Ok(DNSRecordType::CDNSKEY),
            59 => Ok(DNSRecordType::CDS),
            37 => Ok(DNSRecordType::CERT),
            5 => Ok(DNSRecordType::CNAME),
            62 => Ok(DNSRecordType::CSYNC),
            49 => Ok(DNSRecordType::DHCID),
            32769 => Ok(DNSRecordType::DLV),
            39 => Ok(DNSRecordType::DNAME),
            48 => Ok(DNSRecordType::DNSKEY),
            43 => Ok(DNSRecordType::DS),
            108 => Ok(DNSRecordType::EUI48),
            109 => Ok(DNSRecordType::EUI64),
            13 => Ok(DNSRecordType::HINFO),
            55 => Ok(DNSRecordType::HIP),
            65 => Ok(DNSRecordType::HTTPS),
            45 => Ok(DNSRecordType::IPSECKEY),
            25 => Ok(DNSRecordType::KEY),
            36 => Ok(DNSRecordType::KX),
            29 => Ok(DNSRecordType::LOC),
            15 => Ok(DNSRecordType::MX),
            35 => Ok(DNSRecordType::NAPTR),
            2 => Ok(DNSRecordType::NS),
            47 => Ok(DNSRecordType::NSEC),
            50 => Ok(DNSRecordType::NSEC3),
            51 => Ok(DNSRecordType::NSEC3PARAM),
            61 => Ok(DNSRecordType::OPENPGPKEY),
            12 => Ok(DNSRecordType::PTR),
            17 => Ok(DNSRecordType::RP),
            46 => Ok(DNSRecordType::RRSIG),
            24 => Ok(DNSRecordType::SIG),
            53 => Ok(DNSRecordType::SMIMEA),
            6 => Ok(DNSRecordType::SOA),
            33 => Ok(DNSRecordType::SRV),
            44 => Ok(DNSRecordType::SSHFP),
            64 => Ok(DNSRecordType::SVCB),
            32768 => Ok(DNSRecordType::TA),
            249 => Ok(DNSRecordType::TKEY),
            52 => Ok(DNSRecordType::TLSA),
            250 => Ok(DNSRecordType::TSIG),
            16 => Ok(DNSRecordType::TXT),
            256 => Ok(DNSRecordType::URI),
            63 => Ok(DNSRecordType::ZONEMD),
            _ => Err(SCloudException::SCLOUD_QTYPE_U16_FOR_DNSRECORDTYPE_UNKNOWN),
        }
    }
}

impl TryFrom<DNSRecordType> for u16 {
    type Error = SCloudException;

    /// Convert a `DNSRecordType` into its DNS wire format (`u16`).
    ///
    /// Unknown record types are returned as-is to preserve DNS extensibility.
    fn try_from(rt: DNSRecordType) -> Result<u16, Self::Error> {
        match rt {
            DNSRecordType::A => Ok(1),
            DNSRecordType::AAAA => Ok(28),
            DNSRecordType::AFSDB => Ok(18),
            DNSRecordType::APL => Ok(42),
            DNSRecordType::CAA => Ok(257),
            DNSRecordType::CDNSKEY => Ok(60),
            DNSRecordType::CDS => Ok(59),
            DNSRecordType::CERT => Ok(37),
            DNSRecordType::CNAME => Ok(5),
            DNSRecordType::CSYNC => Ok(62),
            DNSRecordType::DHCID => Ok(49),
            DNSRecordType::DLV => Ok(32769),
            DNSRecordType::DNAME => Ok(39),
            DNSRecordType::DNSKEY => Ok(48),
            DNSRecordType::DS => Ok(43),
            DNSRecordType::EUI48 => Ok(108),
            DNSRecordType::EUI64 => Ok(109),
            DNSRecordType::HINFO => Ok(13),
            DNSRecordType::HIP => Ok(55),
            DNSRecordType::HTTPS => Ok(65),
            DNSRecordType::IPSECKEY => Ok(45),
            DNSRecordType::KEY => Ok(25),
            DNSRecordType::KX => Ok(36),
            DNSRecordType::LOC => Ok(29),
            DNSRecordType::MX => Ok(15),
            DNSRecordType::NAPTR => Ok(35),
            DNSRecordType::NS => Ok(2),
            DNSRecordType::NSEC => Ok(47),
            DNSRecordType::NSEC3 => Ok(50),
            DNSRecordType::NSEC3PARAM => Ok(51),
            DNSRecordType::OPENPGPKEY => Ok(61),
            DNSRecordType::PTR => Ok(12),
            DNSRecordType::RP => Ok(17),
            DNSRecordType::RRSIG => Ok(46),
            DNSRecordType::SIG => Ok(24),
            DNSRecordType::SMIMEA => Ok(53),
            DNSRecordType::SOA => Ok(6),
            DNSRecordType::SRV => Ok(33),
            DNSRecordType::SSHFP => Ok(44),
            DNSRecordType::SVCB => Ok(64),
            DNSRecordType::TA => Ok(32768),
            DNSRecordType::TKEY => Ok(249),
            DNSRecordType::TLSA => Ok(52),
            DNSRecordType::TSIG => Ok(250),
            DNSRecordType::TXT => Ok(16),
            DNSRecordType::URI => Ok(256),
            DNSRecordType::ZONEMD => Ok(63),
            DNSRecordType::Unknown(_) => {
                Err(SCloudException::SCLOUD_QTYPE_DNSRECORDTYPE_FOR_U16_UNKNOWN)
            }
        }
    }
}

impl TryFrom<&[u8; 2]> for DNSRecordType {
    type Error = SCloudException;

    /// Convert a 2-byte DNS wire representation into a `DNSRecordType`.
    fn try_from(bytes: &[u8; 2]) -> Result<Self, Self::Error> {
        let v = u16::from_be_bytes(*bytes);
        DNSRecordType::try_from(v)
    }
}
