use crate::dns::q_type::DNSRecordType;
use crate::dns::records::DNSRecord;
use std::net::IpAddr;

#[allow(unused)]
pub(crate) struct DNSCacheRecord {
    record_type: DNSRecordType,
    record: DNSRecord,
    ip_addr: IpAddr,
    domain_name: String,
    last_request: usize,
    ttl: usize,
}
