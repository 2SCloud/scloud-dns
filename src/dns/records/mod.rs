use crate::dns::q_type::DNSRecordType;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct DNSRecord {
    pub name: String,         // ex: www, @, blog
    pub rtype: DNSRecordType, // type of record
    pub ttl: u32,             // time to live in seconds
    pub value: String,        // IP, domain, text, target, etc.

    // Optional fields depending on type
    pub priority: Option<u16>,       // MX, SRV
    pub weight: Option<u16>,         // SRV
    pub port: Option<u16>,           // SRV
    pub flags: Option<u8>,           // NAPTR, CAA
    pub tag: Option<String>,         // CAA
    pub regex: Option<String>,       // NAPTR
    pub replacement: Option<String>, // NAPTR
}
