use std::collections::HashMap;
use crate::dns::records::DNSRecord;

#[derive(Debug)]
pub struct Zone {
    pub name: String,                               
    pub ttl: u32,                                   
    pub soa: Option<DNSRecord>,                        
    pub records: HashMap<String, Vec<DNSRecord>>,
}