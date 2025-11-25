use crate::dns::records;
use crate::dns::records::DNSClass;

pub(crate) struct AuthoritySection {
    q_name: String,
    q_type: records::DNSRecordType,
    q_class: DNSClass,
    ttl: u32,
    pub rdlength: u16,
    pub rdata: Vec<u8>,
}

